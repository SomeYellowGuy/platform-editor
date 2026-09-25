use platform_editor_core::{
    common_util::{Rectf, Vec2, Vec2f},
    component::{
        ComponentId, Event, QueuedComponent,
        level::{
            BoardBase,
            bottom_bar::BottomBarBase,
            end_dialog::{EndDialogBase, EndDialogButtonBase, EndDialogButtonType},
        },
    },
    level::{
        Tile,
        state::{
            CollectibleState, Entity, LevelState, LevelStateOutcome, ShooterBullet, ShooterState,
        },
    },
    screen::Screen,
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

/// Whether to visualize the collision and deadly hitboxes of tiles.
pub const VISUALIZE_TILE_HITBOXES: bool = false;

pub const TILE_SIZE: f32 = 60.0;
pub const COLLECTIBLE_SIZE: f32 = 55.0;
pub const LEVEL_CENTER: Vec2f = Vec2f::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0 + 28.0);

pub const ITEM_PREVIEW_ALPHA: u8 = (u8::MAX as f32 * 0.6) as u8;

/// Converts a position in *level space* to a [`Vec2f`] on the actual screen.
///
/// `(0, 0)` represents the top-left of the level, and 1 unit is 1 level tile.
fn pos_to_screen(state: &LevelState, pos: Vec2f, offset: f32) -> Vec2f {
    LEVEL_CENTER + pos * TILE_SIZE + Vec2f::new(0.0, offset)
        - state.tile_state.size.to_vec2f() / 2.0 * TILE_SIZE
}

/// Converts a [`Rectf`] in *level space* to an [`FRect`] on the actual screen.
///
/// `(0, 0)` represents the top-left of the level, and 1 unit is 1 level tile.
fn rectf_to_screen(state: &LevelState, rect: Rectf, offset: f32) -> FRect {
    let pos = pos_to_screen(state, rect.pos, offset);
    FRect::new(
        pos.x,
        pos.y,
        rect.dimensions.x * TILE_SIZE,
        rect.dimensions.y * TILE_SIZE,
    )
}

/// Converts a position in the actual screen of the game to a [`Vec2f`] on the actual screen.
///
/// `(0, 0)` represents the top-left of the level, and 1 unit is 1 level tile.
fn screen_to_pos(state: &LevelState, screen_pos: Vec2f, offset: f32) -> Vec2f {
    (screen_pos - LEVEL_CENTER - Vec2f::new(0.0, offset)
        + state.tile_state.size.to_vec2f() / 2.0 * TILE_SIZE)
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

fn copy_with_rotation(
    canvas: &mut Canvas<Window>,
    alpha: u8,
    texture: &mut Texture,
    src: Option<FRect>,
    dst: FRect,
    angle: f64,
) -> DrawResult {
    texture.set_alpha_mod(alpha);
    canvas.copy_ex(texture, src, dst, angle, None, false, false)
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

        let level_center = LEVEL_CENTER.add_y(offset);

        data.canvas.set_blend_mode(BlendMode::Blend);
        data.canvas
            .set_draw_color(Color::RGBA(30, 30, 30, multiply_alphas(alpha, 180)));
        data.canvas.fill_rect(FRect::from_center(
            level_center.into_fpoint(),
            width as f32 * TILE_SIZE + 20.0,
            height as f32 * TILE_SIZE + 20.0,
        ))?;

        // Draw the empty board.
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
            }
        }

        // Draw the bullets.
        for bullet in &state.shooter_bullets {
            let size = ShooterBullet::SIZE * TILE_SIZE;
            copy(
                data.canvas,
                alpha,
                &mut data.textures.level.bullet,
                None,
                FRect::from_center(
                    pos_to_screen(state, bullet.pos, offset).into_fpoint(),
                    size,
                    size,
                ),
            )?
        }

        // Draw the actual tiles.
        for y in 0..height {
            for x in 0..width {
                let tile = state.tile_state.tile(x, y);

                let rendering_rect = tile.rendering_rect(Vec2f::new(x as f32, y as f32));
                if let Some(rect) = rendering_rect {
                    let rect = rectf_to_screen(state, rect, offset);

                    // Draw the tile.
                    if let Some(texture) = data.textures.level.tiles.texture_from_tile_mut(tile) {
                        copy(data.canvas, alpha, texture, None, rect)?;
                    }
                }
            }
        }

        // Draw the moving platforms.
        for moving in &state.moving {
            if let Some(texture) = moving.ty.texture_mut(&mut data.textures.level.tiles) {
                copy(
                    data.canvas,
                    alpha,
                    texture,
                    None,
                    FRect::from_center(
                        pos_to_screen(state, moving.pos, offset).into_fpoint(),
                        TILE_SIZE,
                        TILE_SIZE,
                    ),
                )?;
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
                Entity::SIZE * TILE_SIZE,
                Entity::SIZE * TILE_SIZE,
            ),
        )?;

        // Draw the flag.
        let flag_center = pos_to_screen(state, state.flag.pos, offset).into_fpoint();
        let (texture, width_mul, height_mul) = if let Some(instant) = state.finish_instant {
            const FLAG_HIT_ANIMATION_DURATION: f32 = 0.6;
            // 1 - start, 0 - end
            let t: f32 = 1.0 - instant.elapsed().as_secs_f32() / FLAG_HIT_ANIMATION_DURATION;
            let hit_scale_multiplier = if t > 0.0 { 1.0 + (t * t) * 0.3 } else { 1.0 };
            (
                &mut data.textures.level.hit_flag,
                hit_scale_multiplier * 1.4,
                hit_scale_multiplier * 1.6,
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

            let center = pos_to_screen(state, target.to_center_vec2f(), offset);
            let rect = FRect::from_center(center.into_fpoint(), TILE_SIZE, TILE_SIZE);

            copy(
                data.canvas,
                multiply_alphas(alpha, ITEM_PREVIEW_ALPHA),
                texture,
                None,
                rect,
            )?;
        }

        // Draw the collectibles.
        let angle = data.oscillation_angle(2.0);
        let angle_sine = angle.sin();
        let vertical_offset = 4.0 * (2.0 * angle).cos() as f32;
        for collectible in &state.collectibles {
            let (collectible_alpha, size_multiplier) =
                if let Some(time) = collectible.collect_time.map(|i| i.elapsed().as_secs_f32()) {
                    (
                        ((1.0 - time / CollectibleState::FADE_TIME) * (u8::MAX as f32)) as u8,
                        1.0 + time,
                    )
                } else {
                    (u8::MAX, 1.0)
                };

            if let Some(texture) = data
                .textures
                .level
                .collectibles
                .texture_from_collectible_mut(collectible.ty)
            {
                let center = pos_to_screen(state, collectible.pos, offset);
                let scale = collectible.ty.collectible_rect_scale();
                copy_with_rotation(
                    data.canvas,
                    multiply_alphas(alpha, collectible_alpha),
                    texture,
                    None,
                    FRect::from_center(
                        center.add_y(vertical_offset).into_fpoint(),
                        COLLECTIBLE_SIZE * size_multiplier * scale.x,
                        COLLECTIBLE_SIZE * size_multiplier * scale.y,
                    ),
                    10.0 * angle_sine,
                )?;
            }
        }

        // Draw any bullet glows.
        for shooter in &state.shooters {
            if let Some(glow_alpha) = shooter.glow() {
                let size_multiplier = TILE_SIZE * (0.5 + 1.0 - glow_alpha);
                copy(
                    data.canvas,
                    (alpha as f32 * glow_alpha) as u8,
                    &mut data.textures.level.bullet_glow,
                    None,
                    FRect::from_center(
                        (pos_to_screen(state, shooter.pos, offset) - Vec2f::new(2.0, 2.0))
                            .into_fpoint(),
                        ShooterState::GLOW_SIZE * size_multiplier,
                        ShooterState::GLOW_SIZE * size_multiplier,
                    ),
                )?
            }
        }

        if VISUALIZE_TILE_HITBOXES {
            visualize_tile_hitboxes(data, width, height, offset)?;
        }

        Ok(())
    }
}

pub fn visualize_tile_hitboxes(
    data: &mut RenderData,
    width: usize,
    height: usize,
    offset: f32,
) -> DrawResult {
    let state = RenderData::level_state(data.extracted_data)?;

    for y in 0..height {
        for x in 0..width {
            let tile = state.tile_state.tile(x, y);
            let tile_pos = Vec2::new(x, y);
            if let Some(rect) = tile.hitbox(Vec2::new(x as f32, y as f32)) {
                // Normal hitboxes are colored blue.
                data.canvas
                    .draw_rect(rectf_to_screen(state, rect, offset))?;
            }
            if let Tile::Spike(d) = tile {
                // Deadly hitboxes are colored red.
                data.canvas.set_draw_color(Color::RED);
                let rects = Tile::spike_hitboxes(tile_pos, *d);
                data.canvas
                    .draw_rect(rectf_to_screen(state, rects[0], offset))?;
                data.canvas
                    .draw_rect(rectf_to_screen(state, rects[0], offset))?;
            }
        }
    }

    Ok(())
}

impl Logic for BoardBase {
    fn run_logic(&mut self, data: &mut crate::logic::LogicData) {
        let delta = data.delta_time as f32 / 1_000_000_000.0;
        let finished = data
            .app_data
            .level_state
            .as_ref()
            .is_some_and(LevelState::is_finished);

        let mouse_pos = data.mouse_pos();

        let (mouse_up, left, right, jump) = if finished {
            (false, false, false, false)
        } else {
            (
                data.is_mouse_button_up(MouseButton::Left),
                data.is_held(Scancode::Left),
                data.is_held(Scancode::Right),
                data.is_held(Scancode::Up),
            )
        };

        let outcome = {
            let Some(state) = &mut data.app_data.level_state else {
                return;
            };

            // Check for an item to be placed.
            if mouse_up {
                let mouse_pos = Vec2f::new(mouse_pos.x, mouse_pos.y);
                state.try_place_item(
                    screen_to_pos(state, mouse_pos, 0.0).map(|f| f as i32),
                    delta,
                );
            }

            state.tick(delta)
        };

        if finished {
            return;
        }

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
                data.reset_level_call();
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
