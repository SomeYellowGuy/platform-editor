use platform_editor_core::{
    common_util::{Vec2, Vec2f},
    component::{
        ComponentId, Event, QueuedComponent,
        level::{
            BoardBase,
            bottom_bar::BottomBarBase,
            end_dialog::{EndDialogBase, EndDialogButtonBase, EndDialogButtonType},
        },
    },
    level::state::{LevelState, LevelStateOutcome, entity::ENTITY_SIZE},
    screen::{Screen, TransitionCall, TransitionData},
};
use sdl3::{
    keyboard::Scancode,
    mouse::MouseButton,
    pixels::Color,
    render::{BlendMode, Canvas, FRect, Texture},
    video::Window,
};

use crate::{
    HEIGHT, WIDTH,
    component::Component,
    logic::Logic,
    render::{DrawResult, Render, RenderData},
    util::{FRectExt, IntoFPoint},
};

pub const TILE_SIZE: f32 = 60.0;
pub const LEVEL_CENTER: Vec2f = Vec2f::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0 + 28.0);

pub const ITEM_PREVIEW_ALPHA: u8 = (u8::MAX as f32 * 0.6) as u8;

/// Converts a position in *level space* to a [`Vec2f`] on the actual screen.
///
/// `(0, 0)` represents the top-left of the level, and 1 unit is 1 level tile.
fn pos_to_screen(state: &LevelState, pos: Vec2f, offset: f32) -> Vec2f {
    LEVEL_CENTER + pos * TILE_SIZE + Vec2f::new(0.0, offset)
        - state.tile_state.size.map(|u| u as f32) / 2.0 * TILE_SIZE
}

/// Converts a position in the actual screen of the game to a [`Vec2f`] on the actual screen.
///
/// `(0, 0)` represents the top-left of the level, and 1 unit is 1 level tile.
fn screen_to_pos(state: &LevelState, screen_pos: Vec2f, offset: f32) -> Vec2f {
    (screen_pos - LEVEL_CENTER - Vec2f::new(0.0, offset)
        + state.tile_state.size.map(|u| u as f32) / 2.0 * TILE_SIZE)
        / TILE_SIZE
}

/// Returns the tile position corresponding to the provided mouse position.
///
/// Note that this clamps down invalid positions.
fn tile_from_mouse_pos(
    state: &LevelState,
    mouse_pos: Vec2f,
    width: usize,
    height: usize,
    offset: f32,
) -> Vec2<usize> {
    let mut target = screen_to_pos(state, mouse_pos, offset).map(|f| f as usize);
    target.x = target.x.clamp(0, width - 1);
    target.y = target.y.clamp(0, height - 1);

    target
}

// Texture rendering functions with a specifiable alpha value.

fn copy(
    canvas: &mut Canvas<Window>,
    alpha: u8,
    texture: &mut Texture,
    src: Option<FRect>,
    dst: FRect,
) -> DrawResult {
    texture.set_alpha_mod(alpha);
    canvas.copy(texture, src, dst)
}

fn multiply_alphas(a1: u8, a2: u8) -> u8 {
    ((a1 as u16 * a2 as u16 + 127) / 255) as u8
}

//

impl Render for BoardBase {
    fn render(&self, data: &mut crate::render::RenderData) -> crate::render::DrawResult {
        let t = data.transition_offset(1.8);
        let (alpha, offset) = if data.transitioned_from(Screen::Level) {
            ((255.0 - 5.0 * t) as u8, 0.0)
        } else {
            (u8::MAX, t * t)
        };

        let state = RenderData::level_state(data.extracted_data)?;

        let width = state.tile_state.size.x;
        let height = state.tile_state.size.y;

        let level_center = LEVEL_CENTER + Vec2f::new(0.0, offset);

        data.canvas.set_blend_mode(BlendMode::Blend);
        data.canvas
            .set_draw_color(Color::RGBA(30, 30, 30, multiply_alphas(alpha, 180)));
        data.canvas.fill_rect(FRect::from_center(
            level_center.into_fpoint(),
            width as f32 * TILE_SIZE + 20.0,
            height as f32 * TILE_SIZE + 20.0,
        ))?;

        for y in 0..height {
            for x in 0..width {
                // Draw the white tile texture.
                let center =
                    pos_to_screen(state, Vec2f::new(x as f32 + 0.5, y as f32 + 0.5), offset);
                let rect = FRect::from_center(center.into_fpoint(), TILE_SIZE, TILE_SIZE);
                copy(
                    data.canvas,
                    alpha,
                    &mut data.textures.level.tiles.empty,
                    None,
                    rect,
                )?;

                if y == height - 1 {
                    // Draw the void texture.
                    copy(
                        data.canvas,
                        alpha,
                        &mut data.textures.level.tiles.void,
                        None,
                        rect,
                    )?;
                }

                let tile = state.tile_state.tile(x, y);
                // Draw the tile.
                if let Some(texture) = data.textures.level.tiles.texture_from_tile_mut(tile) {
                    copy(data.canvas, alpha, texture, None, rect)?;
                }
            }
        }

        // Draw the player.
        let player_center = pos_to_screen(state, state.player.pos, offset).into_fpoint();
        copy(
            data.canvas,
            alpha,
            &mut data.textures.level.player,
            None,
            FRect::from_center(
                player_center,
                ENTITY_SIZE * TILE_SIZE,
                ENTITY_SIZE * TILE_SIZE,
            ),
        )?;

        // Draw the flag.
        let flag_center = pos_to_screen(state, state.flag.pos, offset).into_fpoint();
        let (texture, width_mul, height_mul) = if let Some(instant) = state.finish_instant {
            const FLAG_HIT_ANIMATION_DURATION: f32 = 0.6;
            // 1 - start, 0 - end
            let t: f32 = 1.0 - instant.elapsed().as_secs_f32() / FLAG_HIT_ANIMATION_DURATION;
            let hit_scale_multplier = if t > 0.0 { 1.0 + (t * t) * 0.3 } else { 1.0 };
            (
                &mut data.textures.level.hit_flag,
                hit_scale_multplier * 1.4,
                hit_scale_multplier * 1.6,
            )
        } else {
            let flag_animation_state = (data.oscillation_angle(16.0) as usize) % 9;
            (
                &mut data.textures.level.flags[flag_animation_state],
                0.9,
                1.0,
            )
        };
        copy(
            data.canvas,
            alpha,
            texture,
            None,
            FRect::from_center(flag_center, width_mul * TILE_SIZE, height_mul * TILE_SIZE),
        )?;

        // Draw the hypothetical placed item.
        if let Some(selected) = state.selected_item
            && let Some(texture) = state.items[selected]
                .item
                .icon_texture_mut(&mut data.textures.level.tiles)
        {
            let mouse = data.extracted_data.mouse_pos;
            let target =
                tile_from_mouse_pos(state, Vec2::new(mouse.x, mouse.y), width, height, offset);

            let center = pos_to_screen(
                state,
                Vec2f::new(target.x as f32 + 0.5, target.y as f32 + 0.5),
                offset,
            );
            let rect = FRect::from_center(center.into_fpoint(), TILE_SIZE, TILE_SIZE);

            copy(
                data.canvas,
                multiply_alphas(alpha, ITEM_PREVIEW_ALPHA),
                texture,
                None,
                rect,
            )?;
        }

        Ok(())
    }
}

impl Logic for BoardBase {
    fn run_logic(&mut self, data: &mut crate::logic::LogicData) {
        let delta = data.delta_time as f32 / 1_000_000_000.0;

        let mouse_up = data.is_mouse_button_up(MouseButton::Left);
        let mouse_pos = data.mouse_pos();

        let left = data.is_held(Scancode::Left);
        let right = data.is_held(Scancode::Right);
        let jump = data.is_held(Scancode::Up);

        let outcome = {
            let Some(state) = &mut data.app_data.level_state else {
                return;
            };

            // Check for an item to be placed.
            if mouse_up {
                let mouse_pos = Vec2f::new(mouse_pos.x, mouse_pos.y);
                state.try_place_item(screen_to_pos(state, mouse_pos, 0.0).map(|f| f as i32));
            }

            state.tick(delta)
        };

        match outcome {
            LevelStateOutcome::Win => {
                let state = data.app_data.level_state.as_mut().unwrap();

                // Finish the level. Add an end dialog.
                state.mark_finished();

                let displayed_time = state
                    .go_instant
                    .unwrap()
                    .elapsed()
                    .as_secs()
                    .min(BottomBarBase::MAX_DISPLAY_TIME)
                    as u32;

                data.queued.add_event(Event::LevelFinish(displayed_time));
                data.queued.add_component(QueuedComponent {
                    id: ComponentId::EndDialog,
                    component: Component::EndDialog(EndDialogBase::from_level_state(state)),
                    render_priority: 20,
                    logic_priority: 10,
                });

                for ty in EndDialogButtonType::ALL {
                    data.queued.add_component(QueuedComponent {
                        id: ComponentId::EndDialogButton(ty),
                        component: Component::EndDialogButton(EndDialogButtonBase::new(state, ty)),
                        render_priority: 1000,
                        logic_priority: 20,
                    });
                }
            }
            LevelStateOutcome::Lose => {
                // Reset the level by calling a transition.
                data.set_transition_call(TransitionCall::Start(TransitionData::new(
                    0,
                    300_000_000,
                    Screen::Level,
                )));
            }
            LevelStateOutcome::None => {
                let state = data.app_data.level_state.as_mut().unwrap();

                let go = left || right || jump;

                state.player.apply_controls(left, right, jump, delta);

                if go {
                    let (instant, first_go) = state.go();
                    if first_go {
                        data.queue_event(Event::LevelGo(instant));
                    }
                }
            }
        }
    }
}
