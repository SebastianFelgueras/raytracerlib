use crate::{
    color::Color,
};
use image::{
    DynamicImage,
    GenericImageView,
};
pub struct TextureCoordinates{
    pub x: f64,
    pub y:f64,
}
impl TextureCoordinates{
    #[inline]
    pub fn new(x:f64,y:f64)->Self{
        TextureCoordinates{
            x,y,
        }
    }
    #[inline]
    pub fn new_ceros()->Self{
        TextureCoordinates{
            x:0.0,y:0.0,
        }
    }
}
#[derive(Clone)]
pub enum Texture{
    SolidColor(Color),
    Texture(DynamicImage),
}
impl Texture{
    pub fn color_at(&self,coordinates: TextureCoordinates)->Color{
        match self{
            Texture::SolidColor(valor)=>valor.clone(),
            Texture::Texture(valor)=>{
                fn wrap(val: f64, bound: u32)->u32{
                    let signed_bound = bound as i32;
                    let float_coord = val * bound as f64;
                    let wrapped_coord = (float_coord as i32) % signed_bound;
                    if wrapped_coord < 0 {
                        (wrapped_coord + signed_bound) as u32
                    } else {
                        wrapped_coord as u32
                    }
                }
                Color::from_rgba(
                    valor.get_pixel(
                        wrap(coordinates.x, valor.width()),
                        wrap(coordinates.y,valor.height())
                    )
                )
            },
        }
    }
}