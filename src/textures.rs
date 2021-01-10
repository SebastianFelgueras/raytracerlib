use crate::{
    color::Color,
};
use image::{
    DynamicImage,
    GenericImageView,
};
use serde::{
    Serialize, Deserialize,
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
#[derive(Clone,Serialize,Deserialize)]
pub enum Texture{
    SolidColor(Color),
    Texture(TextureData),
}
impl Texture{
    pub fn load_texture(&mut self){
        if let Texture::Texture(valor) = self{
            valor.texture = Some(image::open(&valor.path).expect(&format!("Error when loading texture: \"{}\"",valor.path)));
        }
    }
    pub fn color_at(&self,coordinates: TextureCoordinates)->Color{
        match self{
            Texture::SolidColor(valor)=>valor.clone(),
            Texture::Texture(valor)=>{
                if let None = valor.texture {
                    
                }
                if let Some(imagen) = &valor.texture{
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
                    return Color::from_rgba(
                        imagen.get_pixel(
                            wrap(coordinates.x, imagen.width()),
                            wrap(coordinates.y,imagen.height())
                        )
                    )
                }else{
                    unreachable!();
                }
            },
        }
    }
    pub fn new_texture(path: String)->Texture{
        Texture::Texture(TextureData{
            texture: Some(image::open(&path).unwrap()),
            path,
        })
    }
}
#[derive(Clone,Serialize,Deserialize)]
pub struct TextureData{
    pub path: String,
    #[serde(skip)]
    #[serde(default = "texture_deserializer")]
    pub texture: Option<DynamicImage>,
}
#[inline]
pub fn texture_deserializer()->Option<DynamicImage>{
    None
}
