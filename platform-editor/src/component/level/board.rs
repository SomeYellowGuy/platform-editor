use platform_editor_core::{common_util::FPos, component::level::board::BoardBase};
use sdl3::pixels::Color;

use crate::{HEIGHT, WIDTH, logic::Logic, render::Render, util::{fpos_to_fpoint, frect_from_center}};

impl Render for BoardBase {
    fn render(&self, data: &mut crate::render::RenderData) -> crate::render::DrawResult {
        let center = FPos::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0);

        pub const TILE_SIZE: f32 = 62.0;

        let width = self.state.size.x;
        let height = self.state.size.y;

        data.canvas.set_draw_color(Color::RGBA(30, 30, 30, 180));
        data.canvas.fill_rect(frect_from_center(fpos_to_fpoint(center), width as f32 * TILE_SIZE + 20.0, height as f32 * TILE_SIZE + 20.0))?;

        for y in 0..height {
            for x in 0..width {
                // Draw the white tile texture.
                let center = center + FPos::new(x as f32 - width as f32 / 2.0 + 0.5, y as f32 - height as f32 / 2.0 + 0.5) * TILE_SIZE;
                let rect = frect_from_center(fpos_to_fpoint(center), TILE_SIZE, TILE_SIZE);
                data.canvas.copy_ex(&data.textures.level.tiles.empty, None, rect, 0.0, None, false, false)?;

                let tile = self.state.tile(x, y);
                // Draw the tile.
                if let Some(texture) = data.textures.level.tiles.texture_from_tile(tile) {
                    data.canvas.copy_ex(texture, None, rect, 0.0, None, false, false)?;
                }
            }
        }

        Ok(())
    }
}

impl Logic for BoardBase {

}