use crate::{
    textures::Texture,
};
#[derive(Clone)]
pub enum MaterialType{
    Opaque,
    Reflective{reflectivity: f64},
    Refractive,
}
#[derive(Clone)]
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