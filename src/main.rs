use macroquad::prelude::*;
use crate::tiles::*;
mod tiles;
use crate::tileset::*;
mod tileset;

#[macroquad::main("King Tile")]
async fn main() {
    println!("Loading Textures...");
    let textures = load_textures().await;
    println!("Textures Loaded.");
    println!("Generating Terrain...");
    let world = Terrain::new(vec2(0.0, 0.0), (50, 50));
    println!("Terrain Generated.");
    let mut camera_pos = vec2(0.0, 0.0);
    let mut scale = 16.0;

    let mut fast_mode = true;

    set_cursor_grab(true);

    loop {
        clear_background(BLACK);

        let width = screen_width();
        let height = screen_height();


        let min_x: f32 = -camera_pos.x/(CHUNK_SIZE as f32 * scale);        
        let min_y: f32 = -camera_pos.y/(CHUNK_SIZE as f32 * scale);        
        let max_x: f32 = (width-camera_pos.x)/(CHUNK_SIZE as f32 * scale);
        let max_y: f32 = (height-camera_pos.y)/(CHUNK_SIZE as f32 * scale);


        let x_range = (min_x.floor() as isize).max(0)..(max_x.ceil() as isize).min(world.dimensions.0 as isize);
        let y_range = (min_y.floor() as isize).max(0)..(max_y.ceil() as isize).min(world.dimensions.1 as isize);
        

        for chunk_x in x_range {
            for chunk_y in y_range.clone() {
                let chunk_index = chunk_y as usize * world.dimensions.0 + chunk_x as usize;
                match world.chunks[chunk_index] {
                    Chunk::Chunk(tiles, general_color, one_tile) => {
                        if fast_mode {
                            let chunk_scale = CHUNK_SIZE as f32 * scale;
                            draw_rectangle( 
                                chunk_x as f32 * chunk_scale + camera_pos.x + world.position.x,
                                chunk_y as f32 * chunk_scale + camera_pos.y + world.position.y,
                                chunk_scale,
                                chunk_scale,
                                general_color
                            );
                            continue;
                        }
                        else if one_tile {
                        }
                        for (i, tile) in tiles.iter().enumerate() {
                            let x = chunk_x as f32 * CHUNK_SIZE as f32 + (i / CHUNK_SIZE) as f32;
                            let y = chunk_y as f32 * CHUNK_SIZE as f32 + i as f32 % CHUNK_SIZE as f32;
                            draw_texture_ex(&textures[get_texture(*tile)], x*scale + camera_pos.x, y*scale + camera_pos.y, WHITE, DrawTextureParams {
                                dest_size: Some(vec2(scale, scale)),
                                ..Default::default()
                            });
                            //draw_rectangle(
                            //    x*scale + camera_pos.x + world.position.x, 
                            //    y*scale + camera_pos.y + world.position.y, 
                            //    scale, scale, 
                            //    tile.color()
                            //);
                        }
                    }
                    _=>()
                }    
            } 
        }

        camera_pos += mouse_delta_position()*100.0;
        
        draw_text(&format!("{}", get_fps()), 10.0, 60.0, 64.0, WHITE);

        if get_fps() < 20 {
            fast_mode = true;
        }

        if is_key_pressed(KeyCode::F) {
            fast_mode = !fast_mode;
        }
        if is_key_down(KeyCode::W) {
            scale += 1.0;
        }
        if is_key_down(KeyCode::S) {
            scale -= 1.0;
        }

        next_frame().await
    }
}

