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
    color_at_intersection: Color,
    reflected_ray: Ray,
    alpha_channel: u8,
    distance_to_intersection: f64,
}
impl Intersection{
    #[inline]
    pub fn new()->Self{
        Intersection{
            color_at_intersection: Color::new(),
            reflected_ray: Ray::new(Point3D::new_zeros(), Vector3D::new_zeros()),
            alpha_channel: 255, //opaco
            distance_to_intersection: 0.0,
        }
    }
    #[inline]
    pub fn rgba(&self)->Rgba<u8>{
        Rgba::from_channels(self.color_at_intersection.red,self.color_at_intersection.green,self.color_at_intersection.blue,self.alpha_channel)
    }
    #[inline]
    pub fn new_values(color_at_intersection: Color,reflected_ray: Ray,alpha_channel: u8,distance_to_intersection:f64)->Self{
        Intersection{
            color_at_intersection,
            reflected_ray,
            alpha_channel,
            distance_to_intersection,
        }
    }
    #[inline]
    pub fn color(&self)->&Color{
        &self.color_at_intersection
    }
    #[inline]
    pub fn distance(&self)->f64{
        self.distance_to_intersection
    }
    #[inline]
    pub fn reflexion(&self)->&Ray{
        &self.reflected_ray
    }
    #[inline]
    pub fn alpha_channel(&self)->u8{
        self.alpha_channel
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
            let alpha = 255;
            let vec_posicion = Vector3D::new_from_point(self.center.clone());
            let vec_posicion_module = vec_posicion.module();
            let cateto = f64::sin(vec_posicion.angle_between(&ray.direccion))*vec_posicion_module;
            if self.radio >= cateto{
                Some(Intersection::new_values(self.color.clone(),Ray::new_null(), alpha,f64::sqrt(vec_posicion_module.powi(2)+cateto.powi(2))))
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

    pub struct Plane{
        pub punto: Point3D,
        pub normal: Vector3D,
        pub color: Color,
    }
    impl Plane{
        pub fn new(punto: Point3D,normal: Vector3D, color: Color)->Plane{
            Plane{
                punto,
                normal,
                color,
            }
        }
    }
    impl SceneObject for Plane{
        fn intersects(&self,ray: &Ray)->Option<Intersection>{
            if ray.direccion.dot_product(&self.normal) < 1e-6{ //es epsilon
                None
            }else{
                let lambda = (self.normal.dot_product(&self.punto)-self.normal.dot_product(&ray.punto))/self.normal.dot_product(&ray.direccion);
                let interseccion_punto = lambda * ray.direccion.clone() + ray.punto.clone().into_vector();
                if interseccion_punto.z() <= 0.0{
                    return None
                }
                Some(Intersection::new_values(self.color.clone(), Ray::new_null(), 255, interseccion_punto.module()))
            }
        }
    }
}