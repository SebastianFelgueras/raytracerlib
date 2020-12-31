use crate::maths::{
    point::Point3D,
    vector3::Vector3D,
};
use crate::color::Color;
use image::{Rgba,Pixel};
pub struct Ray{
    pub punto: Point3D,
    pub direccion: Vector3D,
    pub intensidad: f64,
}
impl Ray{
    #[inline]
    pub fn new(point: Point3D, direction: Vector3D)->Self{
        Ray{
            punto: point,
            direccion: direction,
            intensidad: 0.0,
        }
    }
    #[inline]
    pub fn new_from_points(point1: Point3D,point2: Point3D)->Self{
        Ray{
            punto: point1.clone(),
            direccion: Vector3D::new_from_point(point2 - point1),
            intensidad: 0.0,
        }
    }
    #[inline]
    pub fn new_camera_ray(x: u32,y: u32,scene: &crate::Scene)->Self{
        Ray::new(Point3D::new_zeros(), Vector3D::new(((x as f64+0.5)/ scene.widht as f64)*2.0 -1.0,1.0-((y as f64+0.5)/scene.height as f64)*2.0,-1.0)) //Notese que el 1.0- resto en y es porque la convención para formatos de imágen es y para abajo
    }
    #[inline]
    pub fn new_null()->Self{
        Ray::new(Point3D::new_zeros(),Vector3D::new_zeros())
    }
}
pub struct Intersection{
    pub color_at_intersection: Color,
    pub reflected_ray: Ray,
    pub alpha_channel: u8,
}
impl Intersection{
    #[inline]
    pub fn new()->Self{
        Intersection{
            color_at_intersection: Color::new(),
            reflected_ray: Ray::new(Point3D::new_zeros(), Vector3D::new_zeros()),
            alpha_channel: 255, //opaco
        }
    }
    #[inline]
    pub fn rgba(&self)->Rgba<u8>{
        Rgba::from_channels(self.color_at_intersection.red,self.color_at_intersection.green,self.color_at_intersection.blue,self.alpha_channel)
    }
    #[inline]
    pub fn new_values(color_at_intersection: Color,reflected_ray: Ray,alpha_channel: u8)->Self{
        Intersection{
            color_at_intersection,
            reflected_ray,
            alpha_channel,
        }
    }
}
pub trait SceneObject{
    ///Returns true if the given ray intersects the object
    fn intersects(&self,ray: &Ray)->Option<Intersection>;
}
pub mod objects{
    use crate::{
        maths::{
            point::Point3D,
            vector3::{
                Vector3D,
                Vector3,
            },
        },
        color::Color,
    };
    use super::{
        SceneObject,
        Ray,
        Intersection,
    };
    pub struct Sphere{
        pub center: Point3D,
        pub radio: f64,
        pub color: Color,
    }
    impl SceneObject for Sphere{
        fn intersects(&self,ray: &Ray)->Option<Intersection>{
            let vec_posicion = Vector3D::new_from_point(self.center.clone());
            if self.radio >= f64::sin(vec_posicion.angle_between(&ray.direccion))*vec_posicion.module(){
                Some(Intersection::new_values(self.color.clone(),Ray::new_null(), 255))
            }else{
                None
            }
        }
    }
    impl Sphere{
        pub fn new()->Self{
            Sphere{
                center: Point3D::new(0.0,0.0,-5.0),
                radio: 2.0,
                color: Color::new_white(),
            }
        }
        pub fn new_with_coordinates(center:Point3D,radio:f64,color:Color)->Self{
            Sphere{
                center,
                radio,
                color,
            }
        }
    }
}