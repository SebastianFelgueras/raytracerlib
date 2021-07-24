use crate::{
    maths::{
        point::Point3D,
        vector3::{Vector3D,Vector3},
    },
    Scene,
    color::Color,
    textures::{
        TextureCoordinates,
    }, 
    material::{Material,MaterialType},  
};
use serde::{Serialize, Deserialize};
#[derive(Debug,Clone,Serialize,Deserialize)]
///Ray's direction is guaranteed to be normalized
pub struct Ray{
    pub punto: Point3D,
    direccion: Vector3D,
}
impl Ray{
    #[inline]
    pub fn new(point: Point3D, direction: Vector3D)->Self{
        Ray{
            punto: point,
            direccion: direction.normalize(),
        }
    }
    #[inline]
    pub fn new_from_points(point1: Point3D,point2: Point3D)->Self{
        Ray{
            punto: point1.clone(),
            direccion: Vector3D::new_from_point(point2 - point1).normalize(),
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
                -1.0)
            ) //Notese que el 1.0- resto en y es porque la convención para formatos de imágen es y para abajo
    }
    #[inline]
    pub fn new_null()->Self{
        Ray::new(Point3D::new_zeros(),Vector3D::new_zeros())
    }
    #[inline]
    pub fn reflection(&self,shadow_bias:f64,surface_normal:&Vector3D,origin: Point3D)->Self{
        Ray::new(
                origin + (surface_normal.clone() * shadow_bias).into_point(),
                self.direccion.clone()-2.0*(self.direccion.dot_product(surface_normal))*surface_normal.clone()
            )
    }
    /*pub fn refraction(&self,scene: &Scene,surface_normal: &Vector3D, index: f64, hit_point: &Point3D)->Option<Ray>{
        let mut normal = surface_normal.clone().normalize();
        let incidente = self.direction().clone();
        let mut indice_incidente = scene.indice_refraccion_medio;
        let mut indice_transmitido = index;
        let mut dot = normal.dot_product(&incidente);
        if dot < 0.0{
            normal = -1.0 * normal;
            dot = normal.dot_product(&incidente);
            indice_incidente = index;
            indice_transmitido = scene.indice_refraccion_medio;
        }
        let n = indice_incidente/indice_transmitido;
        let cross = incidente.cross_product(&normal);
        let seno_de_incidente_normal = cross.module();
        let seno_refractado_normal = seno_de_incidente_normal * n;
        let angulo_refractado_normal = seno_refractado_normal.asin();
        let coseno_refractado = angulo_refractado_normal.cos();
        let componente_normal = -1.0 * coseno_refractado *normal.clone();
        //cross NO ESTA NORMALIZADO TODAVIA
        let mut horizontal = cross.cross_product(&normal).normalize();
        if !(horizontal.dot_product(&incidente).acos() > std::f64::consts::FRAC_PI_2){
            horizontal = -1.0 * horizontal;
        }
        let componente_horizontal = (std::f64::consts::FRAC_PI_2 - angulo_refractado_normal).cos() * horizontal;
        let direccion_refractado = componente_horizontal + componente_normal;
        Some(Ray::new(hit_point.clone() + (1e-6 * direccion_refractado.clone()).into_point(),direccion_refractado))
        /*
        let ir = dot.acos().sin() * indice_incidente/indice_transmitido;
        if ir.abs() > 1.0{
            return None;
        }
        let nr = ir.asin().cos();
        let nr_vec = -1.0 * normal.clone();
        let ir_vec = normal.cross_product(&normal.cross_product(&incidente));
        let direccion = ((nr/nr_vec.module()) * nr_vec + (ir/ir_vec.module()) * ir_vec).normalize();
        Some(Ray::new(
            hit_point.clone() + (direccion.clone() * scene.shadow_bias).into_point(),
            direccion, 
        ))*/


        /*let n = indice_incidente / indice_transmitido;
        let argumento = 1.0- n*n * (1.0-dot * dot);
        if argumento < 0.0{
            return None;
        }
        let direccion_transmitido = (incidente.clone() * n
        + normal.clone() *(n*dot-argumento.sqrt())).normalize();
        Some(Ray{
            punto: hit_point.clone() + (direccion_transmitido.clone() * scene.shadow_bias).into_point(),
            direccion: direccion_transmitido,
        })*/
    }*/
    #[inline]
    pub fn direction(&self)->&Vector3D{
        &self.direccion
    }
    #[inline]
    pub fn set_direction(&mut self, direction: Vector3D){
        self.direccion = direction.normalize();
    }
}
// LA IDEA ES ELIMINARLO CUANDO SEA MAS VIABLE TRABAJAR CON TRAIT OBJECTS, QUE VUELVA A COMO AL COMIENZO
//con serde como esta y los otros problemas que no permiten comparar trait objects, es muy dificil
#[derive(Serialize,Deserialize)]
pub enum Object{
    Plane(objects::Plane),
    Sphere(objects::Sphere),
}
impl SceneObject for Object{
    #[inline]
    fn intersection_point(&self,ray:&Ray)->Option<Point3D>{
        match self{
            Object::Plane(valor)=>valor.intersection_point(&ray),
            Object::Sphere(valor)=>valor.intersection_point(&ray),
        }
    }
    #[inline]
    fn surface_normal(&self,hit_point: &Point3D)->Vector3D{
        match self{
            Object::Plane(valor)=>valor.surface_normal(&hit_point),
            Object::Sphere(valor)=>valor.surface_normal(&hit_point),
        }
    }
    #[inline]
    fn object_material(&self)->&Material{
        match self{
            Object::Plane(valor)=>valor.object_material(),
            Object::Sphere(valor)=>valor.object_material(),
        }
    }
    #[inline]
    fn texture_coordinates(&self,hit_point: &Point3D)->TextureCoordinates{
        match self{
            Object::Plane(valor)=>valor.texture_coordinates(&hit_point),
            Object::Sphere(valor)=>valor.texture_coordinates(&hit_point),
        }
    }
}
#[derive(Debug,Clone,Serialize,Deserialize)]
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

#[derive(Clone,Debug,Serialize,Deserialize)]
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
#[derive(Clone,Debug,Serialize,Deserialize)]
pub enum Light{
    Directional(DirectionalLight),
    Spherical(SphericalLight),
}


/*fn fresnel(incident: Vector3D, normal: Vector3D, index: f64) -> f64 {
    let i_dot_n = incident.dot_product(&normal);
    let mut eta_i = 1.0;
    let mut eta_t = index as f64;
    if i_dot_n > 0.0 {
        eta_i = eta_t;
        eta_t = 1.0;
    }

    let sin_t = eta_i / eta_t * (1.0 - i_dot_n * i_dot_n).max(0.0).sqrt();
    if sin_t > 1.0 {
        //Total internal reflection
        return 1.0;
    } else {
        let cos_t = (1.0 - sin_t * sin_t).max(0.0).sqrt();
        let cos_i = cos_t.abs();
        let r_s = ((eta_t * cos_i) - (eta_i * cos_t)) / ((eta_t * cos_i) + (eta_i * cos_t));
        let r_p = ((eta_i * cos_i) - (eta_t * cos_t)) / ((eta_i * cos_i) + (eta_t * cos_t));
        return (r_s * r_s + r_p * r_p) / 2.0;
    }
}*/
pub trait SceneObject{
    ///Returns the intersection point
    fn intersection_point(&self,ray:&Ray)->Option<Point3D>;
    fn surface_normal(&self,hit_point: &Point3D)->Vector3D;
    fn color_at_intersection(&self,ray: &Ray,hit_point: &Point3D,scene: &Scene,current_recurtion: usize)->Color{
        if current_recurtion == scene.max_reflections{
            return Color::black();
        }
        match self.object_material().tipo{
            MaterialType::Opaque=>return self.get_color(hit_point, scene),
            MaterialType::Refractive{refraction_index, transparency}=>{
                todo!();
                /*let surface_color = self.get_color(hit_point, scene);
                let mut refraction_color = Color::black();
                let normal = self.surface_normal(&hit_point);
                let fresnel_val = fresnel(ray.direction().clone(), normal.clone(), refraction_index);
                if fresnel_val < 1.0{
                    let refractado = ray.refraction(
                        &scene,
                        &normal,
                        refraction_index,
                        &hit_point).unwrap();
                    refraction_color = scene.ray_caster(&refractado, current_recurtion).unwrap_or(scene.color_de_fondo.clone())*transparency;
                }
                let reflejado = ray.reflection(scene.shadow_bias,&normal,hit_point.clone());
                let reflejado_color = scene.ray_caster(&reflejado,current_recurtion).unwrap_or(scene.color_de_fondo.clone());
                let mut color = reflejado_color * fresnel_val + refraction_color * (1.0 - fresnel_val);
                color = color * transparency * surface_color;                
                return color;*/
            },
            MaterialType::Reflective{reflectivity}=>{
                let mut color = self.get_color(hit_point, scene);
                if reflectivity > 0.0{ //si es = a cero es opaco 
                    let reflejo = ray.reflection(scene.shadow_bias,&self.surface_normal(hit_point),hit_point.clone());
                    color = color * (1.0-reflectivity) + scene.ray_caster(&reflejo,current_recurtion).unwrap_or(scene.color_de_fondo.clone()) * reflectivity;
                }
                return color;
            }
        }
    }
    fn get_color(&self,hit_point: &Point3D,scene: &Scene)->Color{
        let mut color = Color::black();
        'lights_loop: for light in &scene.lights{
            let direction;
            let light_intensity;
            let light_color; 
            match light{
                Light::Directional(luz) =>{
                    direction = luz.direction.clone();
                    let shadow_ray = Ray::new(hit_point.clone() + (-1.0 * direction.clone()*scene.shadow_bias).into_point(),-1.0 * direction.clone());          
                    //Lo que se le suma al punto evita el shadow acne sobre los planos
                    if scene.object_between(&shadow_ray){
                        continue 'lights_loop;
                    }
                    light_intensity = luz.intensity;
                    light_color = &luz.color;
                },
                Light::Spherical(luz) =>{
                    direction = (hit_point.clone() - luz.punto.clone()).into_vector();
                    let direction_module = direction.module();
                    let shadow_ray = Ray::new(luz.punto.clone(),direction.clone());          
                    for punto in scene.object_between_with_point(&shadow_ray) {
                        let coso = (punto - luz.punto.clone()).module();
                        if coso < direction_module 
                        && !((coso - direction_module).abs() < scene.shadow_bias){ //esta linea evita shadow bias
                            continue 'lights_loop;
                        }
                    }
                    light_color = &luz.color;
                    light_intensity = luz.intensidad / (std::f64::consts::PI * 4.0 * direction_module * direction_module);
                },
            }
            let coordinates = self.texture_coordinates(hit_point);
            let light_power = self.surface_normal(hit_point).dot_product(&(-1.0*direction.normalize())).max(0.0) * light_intensity;
            let light_reflected = self.object_material().albedoo / std::f64::consts::PI;
            color = color + self.object_material().texture.color_at(coordinates,scene.gamma_correction) * light_color.clone() * light_power * light_reflected;
        }
        color.clamp();
        color
    }
    fn object_material(&self)->&Material;
    fn texture_coordinates(&self,hit_point: &Point3D)->TextureCoordinates;
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
        textures::{
            TextureCoordinates,
        },
        material::Material,
    };
    use super::{
        SceneObject,
        Ray,
    };
    use serde::{Serialize, Deserialize};
    #[derive(Serialize,Deserialize)]
    pub struct Sphere{
        pub center: Point3D,
        pub radio: f64,
        pub material: Material,
    }
    impl SceneObject for Sphere{
        fn intersection_point(&self,ray: &Ray)->Option<Point3D>{
            let direccion = ray.direction();
            let aux = direccion.dot_product(&(ray.punto.clone() - self.center.clone()));
            let discriminante = aux*aux-((ray.punto.clone()-self.center.clone()).module().powi(2)-self.radio * self.radio);
            if discriminante < 0.0 {
                return None
            }else{
                let lambda1 = -aux+(discriminante.sqrt());
                let lambda2 = -aux-(discriminante.sqrt());
                let punto;
                if lambda1>lambda2 && lambda2>=0.0{
                    punto =(lambda2 * direccion.clone()).into_point() + ray.punto.clone();
                }else{
                    if lambda1<0.0{
                        return None;
                    }
                    punto =(lambda1 * direccion.clone()).into_point() + ray.punto.clone();
                }
                //evita intersecciones consigo mismo
                if punto.compare(&ray.punto,1e-6){
                    return None;
                }
                Some(punto)  
            }         
        }
        fn surface_normal(&self,hit_point: &Point3D)->Vector3D{
            hit_point.substract(&self.center).normalize()
        }
        fn texture_coordinates(&self,hit_point: &Point3D)->TextureCoordinates{
            let punto = hit_point.clone() - self.center.clone();
            let x = (1.0+punto.z.atan2(punto.x)/std::f64::consts::PI)*0.5; //declinacion
            let y = f64::acos(punto.y/self.radio)/std::f64::consts::PI;// altura
            TextureCoordinates::new(x,y)
        }
        fn object_material(&self)->&Material{
            &self.material
        }
    }
    impl Sphere{
        pub fn new(center:Point3D,radio:f64,material: Material)->Self{
            Sphere{
                center,
                radio,
                material,
            }
        }
    }
    #[derive(Serialize,Deserialize)]
    pub struct Plane{
        pub punto: Point3D,
        pub normal: Vector3D,
        pub material: Material,
    }
    impl Plane{
        pub fn new(punto: Point3D,normal: Vector3D, material: Material)->Plane{
            Plane{
                punto,
                normal,
                material,
            }
        }
    }
    impl SceneObject for Plane{
        fn intersection_point(&self,ray: &Ray)->Option<Point3D>{
            if ray.direccion.dot_product(&self.normal).abs() < 1e-6{ //es epsilon
                return None
            }else{
                let lambda = self.normal.dot_product(&(self.punto.clone()-ray.punto.clone()))
                /self.normal.dot_product(&ray.direccion);
                if lambda <= 0.0{
                    return None
                }
                return Some((ray.direccion.clone().into_point() * lambda + ray.punto.clone())-ray.punto.clone())
            }   
        }
        fn surface_normal(&self,_: &Point3D)->Vector3D{
            self.normal.clone()
        } 
        fn texture_coordinates(&self,hit_point: &Point3D)->TextureCoordinates{
            //Sistema de coordenadas en el plano
            let mut eje1 = self.normal.cross_product(&Point3D::new(1.0,0.0,0.0)).normalize();
            if eje1.module() == 0.0{
                eje1 = self.normal.cross_product(&Point3D::new(0.0,1.0,0.0)).normalize()
            }
            let eje2 = self.normal.cross_product(&eje1).normalize();
            let punto = hit_point.clone() - self.punto.clone();
            let x = punto.dot_product(&eje1);
            let y = punto.dot_product(&eje2);
            TextureCoordinates::new(x,y)
        }
        fn object_material(&self)->&Material{
            &self.material
        } 
    }
}