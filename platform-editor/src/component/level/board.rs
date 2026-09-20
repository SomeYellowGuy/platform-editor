use platform_editor_core::{
    common_util::Vec2f,
    component::{
        ComponentId, Event, QueuedComponent,
        level::{
            board::BoardBase,
            bottom_bar::BottomBarBase,
            end_dialog::{EndDialogBase, EndDialogButtonBase, EndDialogButtonType},
        },
    },
    level::ENTITY_SIZE,
};
use sdl3::{
    keyboard::Scancode,
    pixels::Color,
    render::{FPoint, FRect},
};

use crate::{
    HEIGHT, WIDTH,
    component::Component,
    logic::Logic,
    render::Render,
    util::{FRectExt, IntoFPoint},
};

pub const TILE_SIZE: f32 = 60.0;
pub const LEVEL_CENTER: Vec2f = Vec2f::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0 + 28.0);

/// Converts a position in *level space* to a [`Vec2f`] on the actual screen.
///
/// `(0, 0)` represents the top-left of the level, and 1 unit is 1 level tile.
fn pos_to_screen(base: &BoardBase, pos: Vec2f, offset: f32) -> Vec2f {
    LEVEL_CENTER
        + Vec2f::new(0.0, offset)
        + Vec2f::new(
            pos.x - base.state.tile_state.size.x as f32 / 2.0,
            pos.y - base.state.tile_state.size.y as f32 / 2.0,
        ) * TILE_SIZE
}

/// Converts a position in *level space* to an [`FPoint`] on the actual screen.
///
/// `(0, 0)` represents the top-left of the level, and 1 unit is 1 level tile.
fn pos_to_screen_point(base: &BoardBase, pos: Vec2f, offset: f32) -> FPoint {
    pos_to_screen(base, pos, offset).into_fpoint()
}

impl Render for BoardBase {
    fn render(&self, data: &mut crate::render::RenderData) -> crate::render::DrawResult {
        let width = self.state.tile_state.size.x;
        let height = self.state.tile_state.size.y;

        let t = data.transition_offset(1.8);
        let offset = t * t;

        let level_center = LEVEL_CENTER + Vec2f::new(0.0, offset);

        data.canvas.set_draw_color(Color::RGBA(30, 30, 30, 180));
        data.canvas.fill_rect(FRect::from_center(
            level_center.into_fpoint(),
            width as f32 * TILE_SIZE + 20.0,
            height as f32 * TILE_SIZE + 20.0,
        ))?;

        for y in 0..height {
            for x in 0..width {
                // Draw the white tile texture.
                let center =
                    pos_to_screen(self, Vec2f::new(x as f32 + 0.5, y as f32 + 0.5), offset);
                let rect = FRect::from_center(center.into_fpoint(), TILE_SIZE, TILE_SIZE);
                data.canvas.copy_ex(
                    &data.textures.level.tiles.empty,
                    None,
                    rect,
                    0.0,
                    None,
                    false,
                    false,
                )?;

                let tile = self.state.tile_state.tile(x, y);
                // Draw the tile.
                if let Some(texture) = data.textures.level.tiles.texture_from_tile(tile) {
                    data.canvas
                        .copy_ex(texture, None, rect, 0.0, None, false, false)?;
                }
            }
        }

        // Draw the player.
        let player_center = pos_to_screen_point(self, self.state.player.pos, offset);
        data.canvas.copy_ex(
            &data.textures.level.player,
            None,
            FRect::from_center(
                player_center,
                ENTITY_SIZE * TILE_SIZE,
                ENTITY_SIZE * TILE_SIZE,
            ),
            0.0,
            None,
            false,
            false,
        )?;

        // Draw the flag.
        let flag_center = pos_to_screen_point(self, self.state.flag.pos, offset);
        let (texture, width_mul, height_mul) = if let Some(instant) = self.state.finish_instant {
            const FLAG_HIT_ANIMATION_DURATION: f32 = 0.6;
            // 1 - start, 0 - end
            let t: f32 = 1.0 - instant.elapsed().as_secs_f32() / FLAG_HIT_ANIMATION_DURATION;
            let hit_scale_multplier = if t > 0.0 { 1.0 + (t * t) * 0.3 } else { 1.0 };
            (
                &data.textures.level.hit_flag,
                hit_scale_multplier * 1.4,
                hit_scale_multplier * 1.6,
            )
        } else {
            let flag_animation_state = (data.oscillation_angle(16.0) as usize) % 9;
            (&data.textures.level.flags[flag_animation_state], 0.9, 1.0)
        };
        data.canvas.copy_ex(
            texture,
            None,
            FRect::from_center(flag_center, width_mul * TILE_SIZE, height_mul * TILE_SIZE),
            0.0,
            None,
            false,
            false,
        )?;

        Ok(())
    }
}

impl Logic for BoardBase {
    fn run_logic(&mut self, data: &mut crate::logic::LogicData) {
        let delta = data.delta_time as f32 / 1_000_000_000.0;
        if self.state.tick(delta) {
            // Finish the level. Add an end dialog.
            self.state.mark_finished();

            let displayed_time = self
                .state
                .go_instant
                .unwrap()
                .elapsed()
                .as_secs()
                .min(BottomBarBase::MAX_DISPLAY_TIME) as u32;

            data.queue_event(Event::LevelFinish(displayed_time));
            data.queue_component(QueuedComponent {
                id: ComponentId::EndDialog,
                component: Component::EndDialog(EndDialogBase::from_level_state(&self.state)),
                render_priority: 20,
                logic_priority: 10,
            });

            for ty in EndDialogButtonType::ALL {
                data.queue_component(QueuedComponent {
                    id: ComponentId::EndDialogButton(ty),
                    component: Component::EndDialogButton(EndDialogButtonBase::new(
                        &self.state,
                        ty,
                    )),
                    render_priority: 1000,
                    logic_priority: 20,
                });
            }
        }

        if !self.state.is_finished() {
            let left = data.is_held(Scancode::Left);
            let right = data.is_held(Scancode::Right);
            let jump = data.is_held(Scancode::Up);
            let go = left || right || jump;

            self.state.player.apply_controls(left, right, jump, delta);

            if go {
                let (instant, first_go) = self.state.go();
                if first_go {
                    data.queue_event(Event::LevelGo(instant));
                }
            }
        }
    }
}
