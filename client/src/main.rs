use macroquad::prelude::*;
use crate::tiles::*;
mod tiles;
use crate::tileset::*;
mod tileset;
use crate::player::*;
mod player;

use std::ops::Range;

#[macroquad::main("King Tile")]
async fn main() {
    let mut game: Game = Game::new((10, 10)).await;

    game.run().await;

}


struct Game {
    // Assets
    textures: Vec<Texture2D>,
    // Objects
    terrain: Terrain,
    // Render Options
    camera_offset: Vec2,
    tile_scale: f32,
}



impl Game {
    const DEFAULT_CAMERA_POSITION: Vec2 = vec2(0.0, 0.0);
    const DEFAULT_SCALE: f32 = 16.0;

    async fn new(terrain_dimensions: (usize, usize)) -> Self {
        Self {
            textures: Self::get_textures().await,
            terrain: Self::generate_terrain(terrain_dimensions),
            camera_offset: Self::DEFAULT_CAMERA_POSITION,
            tile_scale: Self::DEFAULT_SCALE,
        }
    } 
    async fn get_textures() -> Vec<Texture2D> {
        load_textures().await
    }
    fn generate_terrain(dimensions: (usize, usize)) -> Terrain {
        Terrain::new(vec2(0.0, 0.0), dimensions)
    }


    async fn run(&mut self) {
        set_cursor_grab(true);

        loop {
            clear_background(BLACK);

       
            let (x_range, y_range) = self.get_chunk_range();

                    
        
            for chunk_x in x_range {
                for chunk_y in y_range.clone() {
                    
                    let chunk_index = chunk_y as usize * self.terrain.dimensions.0 + chunk_x as usize;
                    

                    match self.terrain.chunks[chunk_index] {
                        Chunk::Chunk(tiles, _general_color, _one_tile) => {
                            for (i, tile) in tiles.iter().enumerate() {
                                let x = chunk_x as f32 * CHUNK_SIZE as f32 + (i / CHUNK_SIZE) as f32;
                                let y = chunk_y as f32 * CHUNK_SIZE as f32 + i as f32 % CHUNK_SIZE as f32;
                                draw_texture_ex(&self.textures[get_texture(*tile)], x*self.tile_scale + self.camera_offset.x, y*self.tile_scale + self.camera_offset.y, WHITE, DrawTextureParams {
                                dest_size: Some(vec2(self.tile_scale, self.tile_scale)),
                                    ..Default::default()
                                });
                            }
                        }
                        _=>()
                    }    
                } 
            }

            self.camera_offset += mouse_delta_position()*100.0;

            draw_text(&format!("{}", get_fps()), 10.0, 60.0, 64.0, WHITE);



            let delta = get_frame_time();
            let prev_scale = self.tile_scale;
            if is_key_down(KeyCode::W) {
                self.tile_scale += delta * 10.0;
            }
            if is_key_down(KeyCode::S) {
                self.tile_scale -= delta * 10.0;
                }
        
            self.camera_offset *= self.tile_scale/prev_scale;
            next_frame().await
        }

    }


    fn min_x(&self) -> f32 {
        -self.camera_offset.x/(CHUNK_SIZE as f32 * self.tile_scale)
    }
    fn min_y(&self) -> f32 {
        -self.camera_offset.y/(CHUNK_SIZE as f32 * self.tile_scale)
    }
    fn max_x(&self) -> f32 {
        (screen_width()-self.camera_offset.x)/(CHUNK_SIZE as f32 * self.tile_scale)
    }
    fn max_y(&self) -> f32 {
        (screen_height()-self.camera_offset.y)/(CHUNK_SIZE as f32 * self.tile_scale)
    }

    fn top_left(&self) -> Vec2 {
        vec2(self.min_x(), self.min_y())
    }
    fn top_right(&self) -> Vec2 {
        vec2(self.max_x(), self.min_y())
    }
    fn bottom_left(&self) -> Vec2 {
        vec2(self.min_x(), self.max_y())
    }
    fn bottom_right(&self) -> Vec2 {
        vec2(self.max_x(), self.max_y())
    }
    
    fn get_chunk_range(&self) -> (Range<isize>, Range<isize>) {
        let x_range = (self.min_x().ceil() as isize).max(0)..(self.max_x().floor() as isize).min(self.terrain.dimensions.0 as isize);
        let y_range = (self.min_y().ceil() as isize).max(0)..(self.max_y().floor() as isize).min(self.terrain.dimensions.1 as isize);

        
        (x_range, y_range)
    }

    fn game_to_screen(&self, game_coords: Vec2) -> Vec2 {
        ( game_coords + self.camera_offset ) * self.tile_scale
    }
    fn chunk_to_screen(&self, chunk_coords: Vec2) -> Vec2 {
        ( chunk_coords * CHUNK_SIZE as f32 + self.camera_offset ) * self.tile_scale
    }

    fn chunk_scale(&self) -> f32 {
        self.tile_scale * CHUNK_SIZE as f32
    }



}


