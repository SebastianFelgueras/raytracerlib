use crate::{
    color::Color,
};
use image::{
    DynamicImage,
    GenericImageView,
};
use serde::{
    Serialize, Deserialize,Deserializer
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
#[derive(Clone,Serialize)]
pub struct TextureData{
    path: String,
    #[serde(skip)]
    texture: Option<DynamicImage>,
}
use std::fmt;

use serde::de::{self, Visitor, SeqAccess, MapAccess};

impl<'de> Deserialize<'de> for TextureData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {Path}
        struct DurationVisitor;
        
        impl<'de> Visitor<'de> for DurationVisitor {
            type Value = TextureData;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct TextureData")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<TextureData, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let path = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                Ok(TextureData{
                    texture: Some(image::open(&path).unwrap()),
                    path,
                })
            }

            fn visit_map<V>(self, mut map: V) -> Result<TextureData, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut path = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Path => {
                            if path.is_some() {
                                return Err(de::Error::duplicate_field("secs"));
                            }
                            path = Some(map.next_value()?);
                        }
                    }
                }
                let path = path.ok_or_else(|| de::Error::missing_field("secs"))?;
                Ok(TextureData{
                    texture: Some(image::open(&path).unwrap()),
                    path,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["path"];
        deserializer.deserialize_struct("TextureData", FIELDS, DurationVisitor)
    }
}
