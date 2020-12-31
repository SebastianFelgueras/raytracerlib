use std::ops;
#[derive(Clone)]
pub struct Point3D{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Point3D{
    #[inline]
    pub fn new(x: f64,y: f64, z: f64)->Self{
        Point3D{
            x,y,z
        }
    }
    #[inline]
    pub fn new_zeros()->Self{
        Point3D{
            x:0.0,y:0.0,z:0.0
        }
    }
}
impl ops::Sub for Point3D{
    type Output = Point3D;
    fn sub(self,other:Self)->Self{
        Point3D{
            x: self.x - other.x,
            y: self.y -other.y,
            z: self.z - other.z,
        }
    }
}