use crate::maths::{
    point::Point3D,
    vector3::{Vector3D,Vector3},
};
use crate::Scene;
use crate::color::Color;

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

#[derive(Clone,Debug)]
pub struct SphericalLight{
    pub punto: Point3D,
    pub color: Color,
    pub intensidad: f64,
}
impl SphericalLight{
    #[inline]
    pub fn new(punto: Point3D,color:Color,intensidad:f64)->Self{
        SphericalLight{
            punto,
            color,
            intensidad, 
        }
    }
}
#[derive(Clone,Debug)]
pub enum Light{
    Directional(DirectionalLight),
    Spherical(SphericalLight),
}

pub trait  SceneObject{
    ///Returns the wrapped intersection if the ray intersects the object
    fn intersects(&self,ray: &Ray)->bool;
    ///If the intersection point does not exists, it might be undefined behavior
    fn intersection_point(&self,ray:&Ray)->Option<Point3D>;
    fn surface_normal(&self,hit_point: &Point3D)->Vector3D;
    fn color_at_intersection(&self,hit_point: &Point3D,scene: &Scene)->Color{
        let mut color = Color::new(0.0,0.0,0.0);
        //INICIO CALCULO LUCES
        for light in &scene.lights{
            let direction;
            let light_intensity;
            let light_color; 
            if let Light::Directional(luz) = light{
                direction = luz.direction.clone();
                let shadow_ray = Ray::new(hit_point.clone() + (self.surface_normal(&hit_point)*scene.shadow_bias).into_point(),-1.0 * direction.clone());          
                //Lo que se le suma al punto evita el shadow acne sobre los planos
                if scene.object_between(&shadow_ray){
                    light_intensity = 0.0; 
                }else{
                    light_intensity = luz.intensity;
                }
                light_color = &luz.color;
            }else if let Light::Spherical(luz) = light {
                light_color = &luz.color;
                direction = (hit_point.clone() - luz.punto.clone()).into_vector();
                let shadow_ray = Ray::new(luz.punto.clone() + (self.surface_normal(&hit_point)*scene.shadow_bias).into_point(),direction.clone());          
                let mut temp:f64 = luz.intensidad / (std::f64::consts::PI * 4.0 * direction.module().powi(2));
                for punto in scene.object_between_with_point(&shadow_ray) {
                    if (punto - luz.punto.clone()).module() < direction.module(){
                        println!("hola");
                        temp = 0.0; 
                        break;
                    }
                }
                light_intensity = temp;
            }else{
                unimplemented!();
            }
            if light_intensity == 0.0{
                println!("cero");
            }
            let light_power = self.surface_normal(hit_point).dot_product(&(-1.0*direction.normalize())).max(0.0) * light_intensity;
            let light_reflected = self.albedoo() / std::f64::consts::PI;
            color = color + self.object_color() * light_color.clone() * light_power * light_reflected;
        }
        //FIN CALCULO LUCES 
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
    };
    use super::{
        SceneObject,
        Ray,
    };


    pub struct Sphere{
        pub center: Point3D,
        pub radio: f64,
        pub color: Color,
        albedoo: f64,
    }
    impl SceneObject for Sphere{
        fn intersects(&self,ray: &Ray)->bool{
            //Como uso trait objects, esta fue la mejor solucion que encontré para evitar que marque sombra en sus propias intersecciones
            if ((ray.punto.clone()-self.center.clone()).module() - self.radio).abs() < 1e-6{
                return false;
            }
            let aux = ray.direccion.clone().normalize().dot_product(&(ray.punto.clone() - self.center.clone()));
            let discriminante = aux.powi(2)-((ray.punto.clone()-self.center.clone()).module().powi(2)-self.radio.powi(2));
            if discriminante < 0.0 {
                false
            }else{
                true
            }
        }
        fn intersection_point(&self,ray: &Ray)->Option<Point3D>{
                let aux = ray.direccion.clone().normalize().dot_product(&(ray.punto.clone() - self.center.clone()));
                let lambda1 = -aux+(aux.powi(2)-((ray.punto.clone()-self.center.clone()).module().powi(2)-self.radio.powi(2))).sqrt();
                let lambda2 = -aux-(aux.powi(2)-((ray.punto.clone()-self.center.clone()).module().powi(2)-self.radio.powi(2))).sqrt();
                if lambda1>lambda2{
                    Some((lambda2 * ray.direccion.clone() + ray.punto.clone().into_vector()).into_point())
                }else{
                    Some((lambda1 * ray.direccion.clone() + ray.punto.clone().into_vector()).into_point())
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
        fn intersects(&self,ray: &Ray)->bool{
            if ray.direccion.dot_product(&self.normal).abs() < 1e-6{ //es epsilon
                false
            }else{
                let lambda = self.normal.dot_product(&(self.punto.clone()-ray.punto.clone()))
                /self.normal.dot_product(&ray.direccion);
                if lambda <= 0.0{
                    return false
                }
                true
            }
        }
        fn intersection_point(&self,ray: &Ray)->Option<Point3D>{
            let lambda = self.normal.dot_product(&(self.punto.clone()-ray.punto.clone()))
                /self.normal.dot_product(&ray.direccion);
            Some((ray.direccion.clone().into_point() * lambda + ray.punto.clone())-ray.punto.clone())
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