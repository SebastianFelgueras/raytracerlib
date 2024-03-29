use std::ops;
use serde::{Serialize, Deserialize};
use super::vector3::{Vector3,Vector3D};
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Point3D{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Point3D{
    #[inline]
    pub fn new(x: f32,y: f32, z: f32)->Self{
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
    pub fn normalize(mut self)->Self{
        let largo = self.module();
        if largo == 0.0{
            return self;
        }
        self.x /= largo;
        self.y /= largo;
        self.z /= largo; 
        self
    }
    #[inline]
    pub fn compare(&self,point:&Self,epsilon: f32)->bool{
        if (self.x - point.x).abs() < epsilon && (self.y - point.y).abs() < epsilon &&(self.z - point.z).abs() < epsilon{
            return true;
        }
        false
    }
}
impl ops::Sub for Point3D{
    type Output = Point3D;
    fn sub(mut self,other:Self)->Self{
        self.x -= other.x;
        self.z -= other.z;
        self.y -= other.y;
        self
    }
}
impl Vector3 for Point3D{
    fn x(&self)->f32{
        self.x
    }
    fn y(&self)->f32{
        self.y
    }
    fn z(&self)->f32{
        self.z
    }
}
impl ops::Mul<f32> for Point3D{
    type Output = Point3D;
    fn mul(mut self,other: f32)->Self{
        self.x *= other;
        self.y *= other;
        self.z *= other;
        self  
    }
}
impl ops::Add for Point3D{
    type Output = Point3D;
    fn add(mut self,other: Point3D)->Self{
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self
    }
}