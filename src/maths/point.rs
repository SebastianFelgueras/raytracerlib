pub struct Point3D{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Point3D{
    pub fn new(x: f64,y: f64, z: f64)->Self{
        Point3D{
            x,y,z
        }
    }
    pub fn new_zeros()->Self{
        Point3D{
            x:0.0,y:0.0,z:0.0
        }
    }
}