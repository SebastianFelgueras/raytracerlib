use std::ops;
use image::{Rgba,Pixel};
use serde::{Serialize, Deserialize};
const GAMMA_CORRCTION_VALUE:f32 = 2.2;
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Color{
    pub red: f32, //No es claro que con f32 sea más rápido por ahora
    pub green: f32,
    pub blue: f32,
}
#[inline]
fn gamma_decode(linear:f32)->f32{
    linear.powf(GAMMA_CORRCTION_VALUE)
}
#[inline]
fn gamma_encode(linear:f32)->f32{
    linear.powf(1.0 / GAMMA_CORRCTION_VALUE)   
}
impl Color{
    #[inline]
    pub fn new(red: f32,green:f32,blue:f32)->Self{
        Color{
            red,
            green,
            blue,
        }
    }
    #[inline]
    pub fn from_rgb(r:u8,g:u8,b:u8)->Self{
        Color::new(r as f32/255.0, g as f32/255.0, b as f32/255.0)
    }
    #[inline]
    pub fn to_rgb(mut self,gamma_correction: bool)->(u8,u8,u8){
        self.clamp();
        if gamma_correction{
            ((gamma_encode(self.red) * 255.0) as u8,(gamma_encode(self.green) * 255.0) as u8,(gamma_encode(self.blue) * 255.0) as u8)
        }else{
            ((self.red * 255.0) as u8,(self.green * 255.0) as u8,(self.blue * 255.0) as u8)
        }
    }
    #[inline]
    pub fn to_r(&self)->u8{
        (self.red.min(1.0).max(0.0) * 255.0) as u8
    }
    #[inline]
    pub fn to_g(&self)->u8{
        (self.green.min(1.0).max(0.0) * 255.0) as u8
    }
    #[inline]
    pub fn to_b(&self)->u8{
        (self.blue.min(1.0).max(0.0) * 255.0) as u8
    }
    #[inline]
    pub fn clamp(&mut self){
        self.blue = self.blue.min(1.0).max(0.0);
        self.red = self.red.min(1.0).max(0.0);
        self.green = self.green.min(1.0).max(0.0);
    }
    #[inline]
    pub fn new_white()->Self{
        Color{
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        }
    }
    #[inline]
    pub fn black()->Self{
        Color{
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }
    #[inline]
    pub fn to_rgba(self,a:u8,gamma_correction: bool)->Rgba<u8>{
        let canales = self.to_rgb(gamma_correction);
        Rgba::from_channels(canales.0,canales.1,canales.2,a)
    }
    #[inline]
    pub fn from_rgba(rgba: Rgba<u8>,gamma_correction: bool)->Self{
        if gamma_correction{
            Color::new(gamma_decode(rgba[0] as f32 / 255.0), gamma_decode(rgba[1] as f32 / 255.0), gamma_decode(rgba[2] as f32 / 255.0))
        }else{
            Color::new(rgba[0] as f32 / 255.0, rgba[1] as f32 / 255.0, rgba[2] as f32 / 255.0)
        }
    }
}
impl ops::Mul for Color{
    type Output = Color;
    fn mul(self,other: Color)->Color{
        Color::new(self.red * other.red, self.green * other.green, self.blue * other.blue)
    }
}
impl ops::Mul<f32> for Color{
    type Output = Color;
    fn mul(self,other: f32)->Color{
        Color::new(self.red * other, self.green * other, self.blue * other)
    }
}
impl ops::Add for Color{
    type Output = Color;
    fn add(mut self,other: Color)->Color{
        self.red += other.red;
        self.blue += other.blue;
        self.green += other.green;
        self
    }
}