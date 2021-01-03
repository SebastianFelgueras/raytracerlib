use std::ops;
#[derive(Clone,Debug)]
pub struct Color{
    pub red: f64, //Quizás no sea necesario que sea un f32, evaluar después
    pub green: f64,
    pub blue: f64,
}
impl Color{
    pub fn new(red: f64,green:f64,blue:f64)->Self{
        Color{
            red,
            green,
            blue,
        }
    }
    pub fn from_rgb(r:u8,g:u8,b:u8)->Self{
        Color::new(r as f64/255.0, g as f64/255.0, b as f64/255.0)
    }
    pub fn to_rgb(mut self)->(u8,u8,u8){
        self.clamp();
        ((self.red * 255.0) as u8,(self.green * 255.0) as u8,(self.blue * 255.0) as u8)
    }
    pub fn to_r(&self)->u8{
        (self.red.min(1.0).max(0.0) * 255.0) as u8
    }
    pub fn to_g(&self)->u8{
        (self.green.min(1.0).max(0.0) * 255.0) as u8
    }
    pub fn to_b(&self)->u8{
        (self.blue.min(1.0).max(0.0) * 255.0) as u8
    }
    pub fn clamp(&mut self){
        self.blue = self.blue.min(1.0).max(0.0);
        self.red = self.red.min(1.0).max(0.0);
        self.green = self.green.min(1.0).max(0.0);
    }
    pub fn new_white()->Self{
        Color{
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        }
    }
}
impl ops::Mul for Color{
    type Output = Color;
    fn mul(self,other: Color)->Color{
        Color::new(self.red * other.red, self.green * other.green, self.blue * other.blue)
    }
}
impl ops::Mul<f64> for Color{
    type Output = Color;
    fn mul(self,other: f64)->Color{
        Color::new(self.red * other, self.green * other, self.blue * other)
    }
}