use platform_editor_core::{
    common_util::Vec2f, component::level::board::BoardBase, level::ENTITY_SIZE,
};
use sdl3::{keyboard::Scancode, pixels::Color, render::FPoint};

use crate::{
    HEIGHT, WIDTH,
    logic::Logic,
    render::Render,
    util::{fpos_to_fpoint, frect_from_center},
};

pub const TILE_SIZE: f32 = 62.0;
pub const LEVEL_CENTER: Vec2f = Vec2f::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0);

/// Converts a position in *level space* to a [`FPos`] one on the actual screen.
///
/// `(0, 0)` represents the top-left of the level, and 1 unit is 1 level tile.
fn pos_to_screen(base: &BoardBase, pos: Vec2f) -> Vec2f {
    LEVEL_CENTER
        + Vec2f::new(
            pos.x - base.state.tile_state.size.x as f32 / 2.0,
            pos.y - base.state.tile_state.size.y as f32 / 2.0,
        ) * TILE_SIZE
}

/// Converts a position in *level space* to an [`FPoint`] on the actual screen.
///
/// `(0, 0)` represents the top-left of the level, and 1 unit is 1 level tile.
fn pos_to_screen_point(base: &BoardBase, pos: Vec2f) -> FPoint {
    fpos_to_fpoint(pos_to_screen(base, pos))
}

impl Render for BoardBase {
    fn render(&self, data: &mut crate::render::RenderData) -> crate::render::DrawResult {
        let width = self.state.tile_state.size.x;
        let height = self.state.tile_state.size.y;

        data.canvas.set_draw_color(Color::RGBA(30, 30, 30, 180));
        data.canvas.fill_rect(frect_from_center(
            fpos_to_fpoint(LEVEL_CENTER),
            width as f32 * TILE_SIZE + 20.0,
            height as f32 * TILE_SIZE + 20.0,
        ))?;

        for y in 0..height {
            for x in 0..width {
                // Draw the white tile texture.
                let center = pos_to_screen(self, Vec2f::new(x as f32 + 0.5, y as f32 + 0.5));
                let rect = frect_from_center(fpos_to_fpoint(center), TILE_SIZE, TILE_SIZE);
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
        let player_center = pos_to_screen_point(self, self.state.player.pos);
        data.canvas.copy_ex(
            &data.textures.level.player,
            None,
            frect_from_center(
                player_center,
                ENTITY_SIZE * TILE_SIZE,
                ENTITY_SIZE * TILE_SIZE,
            ),
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
        self.state.tick(delta);

        self.state.player.apply_controls(
            data.is_held(Scancode::Left),
            data.is_held(Scancode::Right),
            data.is_held(Scancode::Up),
            delta,
        );
    }
}
