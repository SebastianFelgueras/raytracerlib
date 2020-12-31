#[derive(Clone)]
pub struct Color{
    pub red: u8, //Quizás no sea necesario que sea un f32, evaluar después
    pub green: u8,
    pub blue: u8,
}
impl Color{
    pub fn new()->Self{
        Color{
            red: 0,
            green: 0,
            blue: 0,
        }
    }
    pub fn new_white()->Self{
        Color{
            red: 255,
            green: 255,
            blue: 255,
        }
    }
    pub fn new_rgb(red: u8,green:u8,blue:u8)->Self{
        Color{
            red,
            green,
            blue,
        }
    }
}