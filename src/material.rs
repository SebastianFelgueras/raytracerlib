use crate::{
    textures::Texture,
};
use serde::{Serialize, Deserialize};
#[derive(Clone,Serialize,Deserialize)]
pub enum MaterialType{
    Opaque,
    Reflective{reflectivity: f64},
    Refractive{refraction_index: f64, transparency: f64},
}
#[derive(Clone,Serialize,Deserialize)]
pub struct Material{
    pub texture: Texture,
    pub albedoo: f64,
    pub tipo: MaterialType, 
}
impl Material{
    #[inline]
    pub fn new(texture: Texture,albedoo: f64, tipo: MaterialType)->Self{
        Material{
            texture,
            albedoo,
            tipo,
        }
    }
}