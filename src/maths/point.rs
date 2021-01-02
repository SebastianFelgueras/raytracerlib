use std::ops;
use super::vector3::{Vector3,Vector3D};
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
    pub fn into_vector(self)->Vector3D{
        Vector3D::new_from_point(self)
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
impl Vector3 for Point3D{
    fn x(&self)->f64{
        self.x
    }
    fn y(&self)->f64{
        self.y
    }
    fn z(&self)->f64{
        self.z
    }
}