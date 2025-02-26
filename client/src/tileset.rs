use crate::{Tile, load_texture, Texture2D};

const FOREST: usize = 0;
const DENSE_FOREST: usize = 1;



pub fn get_texture(tile: Tile) -> usize {
    match tile {
        Tile::Forest => FOREST,
        Tile::DenseForest => DENSE_FOREST,
        _=>0,
    }
}

pub async fn load_textures() -> Vec<Texture2D> {
    let textures = vec![
        (load_texture("client/src/assets/images/Forest.png").await,
            "client/src/assets/images/Forest.png"),
        (load_texture("client/src/assets/images/Dense_Forest.png").await,
            "client/src/assets/images/Dense_Forest.png")
    ];
    
    let mut loaded_textures = Vec::new();

    for texture in textures {
        match texture.0 {
            Ok(t) => loaded_textures.push(t),
            Err(e) => {
                println!("Failed to load {}. \n{}", texture.1, e);
                loaded_textures.push(Texture2D::empty());
            }
        }
    }
    
    loaded_textures
}
