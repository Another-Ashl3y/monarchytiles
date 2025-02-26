use crate::{Vec2, BLACK, GRAY, BLUE, DARKBLUE, YELLOW, GREEN, ORANGE, Color};
use noise::{NoiseFn, Perlin};
use rand::prelude::*;

pub const LIGHTBLUE: Color = Color{r: 0.6, g: 0.6, b: 1.0, a: 1.0};
pub const CHUNK_SIZE: usize = 10;
const VOID_CHUNK: [Tile; CHUNK_SIZE.pow(2)] = [Tile::Empty; CHUNK_SIZE.pow(2)];



#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Err,
    Empty,
    Grass,
    Forest,
    DenseForest,
    Lake,
    Sea,
    Ocean,
    Sand,
    HotSand,
    Mountain,
    Snow,
    Tents(Tents),
    Village(Village),

}
impl Tile {
    pub fn new(height: f64, temperature: f64, moisture: f64) -> Self {
        if 1.6 <= height {
            return Tile::Mountain
        }
        if 1.0 <= height {
            return Tile::DenseForest
        }
        if 0.5 < height {
            return Tile::Forest
        }
        if 0.0 < height {
            if 0.7 < moisture {
                return Tile::Lake
            }
            else if 0.8 < temperature {
                return Tile::HotSand            
            }
            else {
                return Tile::Grass
            }
        }
        if -0.3 < height {
            if 0.8 < temperature {
                return Tile::HotSand
            }
            return Tile::Sand
        }
        if -0.6 < height || 0.8 < temperature {
            return Tile::Sea
        }
        Tile::Ocean
    }
    pub fn color(&self) -> Color {
        match self {
            Self::Grass => GREEN,
            Self::Ocean => DARKBLUE,
            Self::Sea => BLUE,
            Self::Lake => LIGHTBLUE,
            Self::Mountain => GRAY,
            Self::Sand => YELLOW,
            Self::HotSand => ORANGE,
            _=>BLACK
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tents {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Village {}

#[derive(Debug, Clone)]
pub enum Chunk {
    Chunk([Tile; CHUNK_SIZE.pow(2)], Color, bool),
    Empty,
}
pub struct Terrain {
    pub position: Vec2,
    pub dimensions: (usize, usize),
    pub chunks: Vec<Chunk>
}

impl Terrain {
    pub fn new(position: Vec2, dimensions: (usize, usize)) -> Self {
        let mut chunks: Vec<Chunk> = Vec::new();

        let mut avg_height = 0.0;

        let mut random = rand::rng();
        let height_noise = Perlin::new(random.random());
        let temperature_noise = Perlin::new(random.random());
        let moisture_noise = Perlin::new(random.random());

        println!("Building blocks...");
        for chunk_x in 0..dimensions.0 {
            for chunk_y in 0..dimensions.1 {
                let mut tiles: [Tile; CHUNK_SIZE.pow(2)] = [Tile::Empty; CHUNK_SIZE.pow(2)];
                
                for x in 0..CHUNK_SIZE {
                    for y in 0..CHUNK_SIZE {
                        let real_x = (position.x + (chunk_x*CHUNK_SIZE) as f32 + x as f32) as f64 /dimensions.0 as f64;
                        let real_y = (position.y + (chunk_y*CHUNK_SIZE) as f32 + y as f32) as f64 /dimensions.1 as f64;

                        let height = height_noise.get([real_y, real_x]);
                        let temperature = temperature_noise.get([real_x, real_y, height]);
                        let moisture = moisture_noise.get([real_x, real_y, height]);

                        avg_height += height; avg_height /= 2.0;

                        tiles[y*CHUNK_SIZE + x] = Tile::new(height*2.0, temperature*2.0, moisture*2.0);
                    }
                }
                if tiles == VOID_CHUNK {
                    chunks.push(Chunk::Empty);
                }
                else {
                    let mut avg_r = 0.0;
                    let mut avg_g = 0.0;
                    let mut avg_b = 0.0;
                    let mut one_tile = true;
                    let one_check = tiles[0];
                    for tile in tiles {
                        let color = tile.color();
                        avg_r += color.r;
                        avg_g += color.g;
                        avg_b += color.b;
                        if tile != one_check {
                            one_tile = false;
                        }
                    }
                    avg_r /= tiles.len() as f32;
                    avg_g /= tiles.len() as f32;
                    avg_b /= tiles.len() as f32;
                    chunks.push(Chunk::Chunk(tiles, Color{r: avg_r, g: avg_g, b: avg_b, a: 1.0}, one_tile));
                }
            }
        }
        println!("Blocks built.");
        println!("Correcting orientations");
        for chunk_x in 0..dimensions.0 {
            for chunk_y in 0..dimensions.1 {
                let chunk_index = chunk_y*dimensions.0+chunk_x;
                let mut adjacent_chunks = Vec::new();
                if chunk_x != 0 {
                    adjacent_chunks.push(chunk_y*dimensions.0+chunk_x-1);
                }
                if chunk_y != 0 {
                    adjacent_chunks.push((chunk_y-1)*dimensions.0+chunk_x);
                }
                if chunk_x != dimensions.0 - 1 {
                    adjacent_chunks.push(chunk_y*dimensions.0+chunk_x+1);
                }
                if chunk_y != dimensions.1 - 1 {
                    adjacent_chunks.push((chunk_y+1)*dimensions.0+chunk_x);
                }





            }
        }
        Terrain {position, dimensions, chunks}
    }
}

