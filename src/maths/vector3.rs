use std::ops;
///Implements 3D mathematical operations (like dot product, cross product, etc) for objects
pub trait Vector3{
    fn x(&self)->f64;
    fn y(&self)->f64;
    fn z(&self)->f64;
    #[inline]
    fn dot_product<T: Vector3>(&self,vector: &T)->f64{
        self.x() * vector.x() + self.y() * vector.y() + self.z() * vector.z()
    }
    #[inline]
    fn angle_between<T: Vector3>(&self,vector: &T)->f64{
        (self.dot_product(vector)/(vector.module()*self.module())).acos()
    }
    #[inline]
    fn module(&self)->f64{
        (self.x().powi(2)+self.y().powi(2)+self.z().powi(2)).sqrt()
    }
    #[inline]
    fn is_orthogonal<T: Vector3>(&self,vector: &T)->bool{
        if self.dot_product(vector) == 0.0{
            true
        }else{
            false
        }
    }
    #[inline]
    ///By now, it returns a Vector3D that implements Vector3 trait
    fn cross_product<T: Vector3>(&self,vector:&T)->Vector3D{
        Vector3D::new(
        self.y() * vector.z() - self.z() * vector.y(), //x
        -(self.x() * vector.z() - vector.x() * self.z()), //y
        self.x() * vector.y() - self.y() * vector.x()) //z

    }
    #[inline]
    fn addition<T: Vector3>(&self,vector: &T)->Vector3D{
        Vector3D::new(
            self.x() + vector.x(),
            self.y() + vector.y(),
            self.z() + vector.z())
    }
    #[inline]
    fn escalar_product(&self,escalar:f64)->Vector3D{
        Vector3D::new(self.x()*escalar, self.y()*escalar, self.z()*escalar)
    }
    #[inline]
    fn product<T: Vector3>(&self,vector: &T)->Vector3D{
        Vector3D::new(
            vector.x() * self.x(),
            vector.y() * self.y(),
            vector.z() * self.z())
    }
    #[inline]
    ///substract vector from self
    fn substract<T: Vector3>(&self,vector: &T)->Vector3D{
        self.addition(&vector.escalar_product(-1.0))
    }
}
#[derive(PartialEq,Debug,Clone)]
///A structure that represents a 3D vector and implements Vector3, its recomended to use it, it implements standard ops and PartialEq (consider that it is meaningless to consider one gretear than the other)
pub struct Vector3D{
    x: f64,
    y: f64,
    z: f64,
}
impl ops::Add for Vector3D{
    type Output = Vector3D;
    fn add(self,other: Self)->Self{
        self.addition(&other)
    }
}
impl ops::Sub for Vector3D{
    type Output = Vector3D;
    fn sub(self,other:Self)->Self{
        self.substract(&other)
    }
}
impl ops::Mul<f64> for Vector3D{
    type Output = Vector3D;
    fn mul(self,numero:f64)->Self{
        self.escalar_product(numero)
    }
}
impl ops::Mul<Vector3D> for f64{
    type Output = Vector3D;
    fn mul(self,vector:Vector3D)->Vector3D{
        vector.escalar_product(self)
    }
}
impl ops::Mul<Vector3D> for Vector3D{
    type Output = Vector3D;
    fn mul(self,vector:Vector3D)->Vector3D{
        vector.product(&self)
    }
}
impl Vector3D{
    #[inline]
    pub fn new_zeros()->Self{
        Vector3D{
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    #[inline]
    pub fn new(x: f64, y: f64,z: f64)->Self{
        Vector3D{
            x,
            y,
            z,
        }
    }
    #[inline]
    pub fn new_from_point(point: crate::maths::point::Point3D)->Vector3D{
        Vector3D::new(point.x, point.y,point.z)
    }
}
impl Vector3 for Vector3D{
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
#[cfg(test)]
mod tests{
    use super::Vector3D;
    #[test]
    fn add_substraction_comparison_escalar_multiplication(){
        if Vector3D::new(8.0, 66.0, 25.0)-Vector3D::new(9.0, -4.0,25.0)!=Vector3D::new(-1.0, 70.0, 0.0){
            panic!("Substraction fail")
        }
        if Vector3D::new(8.0, 66.0, 25.0)+Vector3D::new(-9.0, 4.0,-25.0)!=Vector3D::new(-1.0, 70.0, 0.0){
            panic!("Addition failed")
        }
        if Vector3D::new(8.0, 66.0, -25.0) * 2.0 != Vector3D::new(16.0, 132.0, 50.0) && -0.5 * Vector3D::new(8.0, 66.0, -50.0)  != Vector3D::new(-4.0, -33.0, 25.0){
            panic!("Escalar product fail")
        }
        if Vector3D::new(8.0, 66.0, -25.0) * Vector3D::new(-0.5, 0.0, -2.0) != Vector3D::new(-4.0, 0.0, 50.0){
            panic!(format!("Product fail {:?}*{:?} = {:?}",Vector3D::new(8.0, 66.0, -25.0),Vector3D::new(-0.5, 0.0, -2.0),Vector3D::new(8.0, 66.0, -25.0) * Vector3D::new(-0.5, 0.0, -2.0)))
        }
    } 
}