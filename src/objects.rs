use crate::maths::{
    point::Point3D,
    vector3::{Vector3D,Vector3},
};
use crate::Scene;
use crate::color::Color;
use image::{Rgba,Pixel};


#[derive(Debug,Clone)]
pub struct Ray{
    pub punto: Point3D,
    pub direccion: Vector3D,
    //pub intensidad: f64,
}
impl Ray{
    #[inline]
    pub fn new(point: Point3D, direction: Vector3D)->Self{
        Ray{
            punto: point,
            direccion: direction,
            //intensidad: 0.0,
        }
    }
    #[inline]
    pub fn new_from_points(point1: Point3D,point2: Point3D)->Self{
        Ray{
            punto: point1.clone(),
            direccion: Vector3D::new_from_point(point2 - point1),
           // intensidad: 0.0,
        }
    }
    #[inline]
    pub fn new_camera_ray(x: u32,y: u32,scene: &crate::Scene)->Self{
        Ray::new(
            Point3D::new_zeros(),
            Vector3D::new(
                ((x as f64+0.5)/ scene.widht as f64)*2.0 -1.0,
                1.0-((y as f64+0.5)/scene.height as f64)*2.0,
                -1.0).normalize()
            ) //Notese que el 1.0- resto en y es porque la convención para formatos de imágen es y para abajo
    }
    #[inline]
    pub fn new_null()->Self{
        Ray::new(Point3D::new_zeros(),Vector3D::new_zeros())
    }
}


#[derive(Debug)]
pub struct Intersection{
    color_at_intersection: Color,
    reflected_ray: Ray,
    alpha_channel: u8,
    distance_to_intersection: f64,
}
impl Intersection{
    #[inline]
    pub fn rgba(&self)->Rgba<u8>{
        Rgba::from_channels(self.color_at_intersection.to_r(),self.color_at_intersection.to_g(),self.color_at_intersection.to_b(),self.alpha_channel)
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
#[derive(Debug,Clone)]
pub struct DirectionalLight{
    pub color: Color,
    pub direction: Vector3D,
    pub intensity: f64,
}
impl DirectionalLight{
    pub fn new()->Self{
        DirectionalLight{
            color: Color::new_white(),
            direction: Vector3D::new(0.0,-1.0, 0.0),
            intensity: 20.0,
        }
    }
    pub fn new_values(color: Color, direction: Vector3D,intensity:f64)->Self{
        DirectionalLight{
            color,
            direction,
            intensity,
        }
    }
}


pub trait SceneObject{
    ///Returns the wrapped intersection if the ray intersects the object
    fn intersects(&self,ray: &Ray,scene: &Scene)->Option<Intersection>;
    fn surface_normal(&self,hit_point: &Point3D)->Vector3D;
    fn color_at_intersection(&self,hit_point: &Point3D,scene: &Scene)->Color{
        let shadow_ray = Ray::new(hit_point.clone(),scene.light_sources.direction.clone());
        let light_intensity;
        if scene.object_between(&shadow_ray){
            light_intensity = 0.0; 
        }else{
            light_intensity = scene.light_sources.intensity;
        }
        let light = scene.light_sources.clone();
        let light_power = self.surface_normal(hit_point).dot_product(&(-1.0*light.direction.clone().normalize())).max(0.0) * light_intensity;
        let light_reflected = self.albedoo() / std::f64::consts::PI;
        let mut color = self.object_color() * light.color.clone() * light_power * light_reflected;
        color.clamp();
        color
    }
    fn albedoo(&self)->f64;
    fn object_color(&self)->Color;
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
        Scene,
    };
    use super::{
        SceneObject,
        Ray,
        Intersection,
        DirectionalLight,
    };


    pub struct Sphere{
        pub center: Point3D,
        pub radio: f64,
        pub color: Color,
        albedoo: f64,
    }
    impl SceneObject for Sphere{
        fn intersects(&self,ray: &Ray,scene: &Scene)->Option<Intersection>{
            let alpha = 255;
            let light = scene;
            let vec_posicion = Vector3D::new_from_point(self.center.clone());
            let vec_posicion_module = vec_posicion.module();
            let cateto = f64::sin(vec_posicion.angle_between(&ray.direccion))*vec_posicion_module;
            if self.radio >= cateto{
                let aux = ray.direccion.clone().normalize().dot_product(&(ray.punto.clone() - self.center.clone()));
                let lambda1 = -aux+(aux.powi(2)-((ray.punto.clone()-self.center.clone()).module().powi(2)-self.radio.powi(2))).sqrt();
                let lambda2 = -aux-(aux.powi(2)-((ray.punto.clone()-self.center.clone()).module().powi(2)-self.radio.powi(2))).sqrt();
                if lambda1>lambda2{
                    Some(Intersection::new_values(self.color_at_intersection(
                        &(lambda2 * ray.direccion.clone() + ray.punto.clone().into_vector()).into_point(), &light),
                        Ray::new_null(), alpha,f64::sqrt(vec_posicion_module.powi(2)+cateto.powi(2))))
                }else{
                    Some(Intersection::new_values(self.color_at_intersection(
                        &(lambda1 * ray.direccion.clone() + ray.punto.clone().into_vector()).into_point(), &light),
                        Ray::new_null(), alpha,f64::sqrt(vec_posicion_module.powi(2)+cateto.powi(2))))
                }
            }else{
                None
            }
        }
        fn surface_normal(&self,hit_point: &Point3D)->Vector3D{
            hit_point.substract(&self.center).normalize()
        }
        fn albedoo(&self)->f64{
            self.albedoo
        }
        fn object_color(&self)->Color{
            self.color.clone()
        }
    }
    impl Sphere{
        pub fn new()->Self{
            Sphere{
                center: Point3D::new(0.0,0.0,-5.0),
                radio: 2.0,
                color: Color::new_white(),
                albedoo: 0.5,
            }
        }
        pub fn new_with_coordinates(center:Point3D,radio:f64,color:Color,albedoo:f64)->Self{
            Sphere{
                center,
                radio,
                color,
                albedoo,
            }
        }
    }

    pub struct Plane{
        pub punto: Point3D,
        pub normal: Vector3D,
        pub color: Color,
        pub albedoo: f64,
    }
    impl Plane{
        pub fn new(punto: Point3D,normal: Vector3D, color: Color,albedoo: f64)->Plane{
            Plane{
                punto,
                normal,
                color,
                albedoo
            }
        }
    }
    impl SceneObject for Plane{
        fn intersects(&self,ray: &Ray,scene: &Scene)->Option<Intersection>{
            if ray.direccion.dot_product(&self.normal).abs() < 1e-6{ //es epsilon
                None
            }else{
                let lambda = self.normal.dot_product(&(self.punto.clone()-ray.punto.clone()))
                /self.normal.dot_product(&ray.direccion);
                if lambda < 0.0{
                    return None
                }
                let interseccion_punto = (lambda * ray.direccion.clone() + ray.punto.clone().into_vector())-ray.punto.clone().into_vector();
                let color_at_intersection = self.color_at_intersection(&interseccion_punto.into_point(), &scene);
                Some(Intersection::new_values(color_at_intersection, Ray::new_null(), 255,interseccion_punto.module()))
            }
        }
        fn surface_normal(&self,_: &Point3D)->Vector3D{
            self.normal.clone()
        } 
        fn albedoo(&self)->f64{
            self.albedoo
        }
        fn object_color(&self)->Color{
            self.color.clone()
        }
    }
}