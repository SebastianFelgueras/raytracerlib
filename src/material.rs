use crate::{
    textures::Texture,
};
use serde::{Serialize, Deserialize};
#[derive(Clone,Serialize,Deserialize)]
pub enum MaterialType{
    Opaque,
    Reflective{reflectivity: f32},
    Refractive{refraction_index: f32, transparency: f32},
}
#[derive(Clone,Serialize,Deserialize)]
pub struct Material{
    pub texture: Texture,
    pub albedoo: f32,
    pub tipo: MaterialType, 
}
impl Material{
    #[inline]
    pub fn new(texture: Texture,albedoo: f32, tipo: MaterialType)->Self{
        Material{
            texture,
            albedoo,
            tipo,
        }
    }
}