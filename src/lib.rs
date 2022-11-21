//! NOTICE THAT THE HOLE LIB OPERATES IN THE ASSUMPTION THAT THE CAMERA IS IN (0,0,0) 
//! AND WITH Y POSITIVE AXIS UP, X POSITIVE AXIS ON THE RIGHT AND Z NEGATIVE AXIS ON FRONT
use serde::{Serialize, Deserialize};
pub mod maths;
pub mod objects;
pub mod color;
pub mod textures;
pub mod material;
use maths::point::Point3D;
use maths::vector3::Vector3;
use color::Color;
use objects::{Ray,SceneObject};
use image::{DynamicImage, GenericImage};
pub use image;
#[derive(Serialize,Deserialize)]
///Notice that gamma correction is disabled by default because it makes the images to render much slow and because
/// in some cases images might look awkward with corrected gamma.
pub struct Scene{
    pub height: u32, //u32 porque eso es lo que exige imagebuffer como argumento
    pub widht: u32,
    pub lights: Vec<objects::Light>,
    pub objects_list: Vec<objects::Object>,
    pub max_reflections: usize,
    pub color_de_fondo: color::Color,
    pub indice_refraccion_medio: f32,
    pub numero_threads: Nthreads,
    pub gamma_correction: bool,
    shadow_bias: f32,
}
#[derive(Serialize,Deserialize)]
pub enum Nthreads{
    Auto,
    Defined(usize),
}
impl Scene{
    ///Creates a new scene with the source of light in the sky, above the camera. By default, it's rendering in a low definition and uses the number of processor's logical cores as the number of threads
    #[inline]
    pub fn new()->Self{
        Scene{
            height: 800,
            widht: 800,
            lights: Vec::new(),
            objects_list: Vec::new(),
            max_reflections: 5,
            shadow_bias: 1e-6, //este valor evita shadow bias en luces puntuales
            indice_refraccion_medio: 1.0,
            color_de_fondo: color::Color::black(),
            numero_threads: Nthreads::Auto,
            gamma_correction: false,
        }
    }
    pub fn render(&self)->DynamicImage{
        if let Nthreads::Defined(mut numero) = self.numero_threads {
            if numero == 0{
                numero = 1;
            }
            self.internal_renderer(numero)
        }else{
            self.internal_renderer(num_cpus::get())
        }
    }
    #[inline] //Solamente la llama el renderer, se justifica
    fn internal_renderer(&self,n_threads: usize)->DynamicImage{
        use std::{
            sync::{
                Arc,
                Mutex,
            }
        };
        use crossbeam_utils::thread;
        let imagen = Arc::new(Mutex::new(DynamicImage::new_rgba8(self.widht,self.height)));
        let n_threads = n_threads as u32;
        thread::scope(|s|{
            let mut threads_handlers = Vec::new();
            for modulo_asociado in 0..n_threads{
                let imagen_transfer = Arc::clone(&imagen);
                threads_handlers.push(s.spawn(move |_|{
                    let mut cache: Vec<image::Rgba<u8>> = Vec::with_capacity(self.widht as usize);
                    let mut y:u32 = modulo_asociado;
                    while y < self.height{
                        for x in 0..self.widht{
                            let rayo_actual = Ray::new_camera_ray(x,y,&self);
                            cache.push(
                                self.ray_caster(&rayo_actual,0)
                                .unwrap_or(self.color_de_fondo.clone())
                                .to_rgba(255,self.gamma_correction)
                            );
                        }
                        let mut mutex_lock = imagen_transfer.lock().unwrap();
                        let mut x = 0;
                        for pixel in &cache{
                            mutex_lock.put_pixel(x,y,*pixel);
                            x += 1;
                        }
                        cache.clear();
                        y += n_threads;
                    }
                }));
            }
            for handler in threads_handlers{
                handler.join().unwrap();
            }
        }).unwrap();
        match Arc::try_unwrap(imagen){
            Ok(valor)=>return valor.into_inner().unwrap(),
            Err(_)=>panic!("No deberia haber mas referencias al Arc de la imagen"),
        }
    }
    fn object_between(&self,ray: &Ray)->bool{
        for objeto in &self.objects_list{
            if objeto.intersection_point(&ray).is_some() {
                return true;
            }
        }
        false
    }
    fn object_between_with_point(&self,ray:&Ray)->Vec<Point3D>{
        let mut intersecciones = Vec::new();
        for objeto in &self.objects_list{
            if let Some(valor) = objeto.intersection_point(&ray){
                intersecciones.push(valor);
            }
        }
        intersecciones
    }
    fn ray_caster(&self,rayo: &Ray,current_iteration: usize)->Option<Color>{
        let mut temp = None;
        let mut minimum_distance_to_intersection = (0.0,true);
        for objeto in &self.objects_list{
            if let Some(hit_point) = objeto.intersection_point(&rayo){
                let hit_point_module = hit_point.module();
                if hit_point_module<minimum_distance_to_intersection.0 || minimum_distance_to_intersection.1 {
                    temp = Some(objeto.color_at_intersection(&rayo,&hit_point,&self,current_iteration+1));
                    minimum_distance_to_intersection.0 = hit_point_module;
                    minimum_distance_to_intersection.1 = false;
                }  
            }
        }
        temp
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::{
        objects::{
            objects::*,
            DirectionalLight,
            Light,
            SphericalLight,
            Object
        },
        maths::{
            point::Point3D,
            vector3::Vector3D,
        },
        color::Color,
        textures::{
            Texture,
        },
        material::{
            Material,
            MaterialType,
        }
    };
    #[test]
    #[ignore]
    fn esfera_centrada() {
        let mut escena = Scene::new();
        escena.color_de_fondo = Color::from_rgb(135, 206, 235);
        escena.widht = 800;
        escena.height = 800;
        escena.lights.push(Light::Directional(DirectionalLight::new_values(Color::new(0.1,0.1, 0.2),Vector3D::new(0.5, -1.0, 0.25),20.0)));
        escena.lights.push(Light::Directional(DirectionalLight::new_values(Color::new(0.05, 0.25, 0.15),Vector3D::new(0.0, -1.0, 0.0),20.0)));
        escena.lights.push(
            Light::Spherical(
                SphericalLight::new(
                    Point3D::new(0.0, 6.0, -2.0), 
                    Color::new(0.3,0.25,0.09), 
                    20000.0,
                )
            )
        );
        escena.objects_list.push(
            Object::Sphere(Sphere::new(
                Point3D::new(0.0, 0.0, 2.1),
                2.0,
                Material::new( 
                    Texture::SolidColor(Color::new(1.0,1.0,1.0)),
                    0.5,
                    MaterialType::Opaque
                )
            ))
        );
        escena.objects_list.push(Object::Sphere(Sphere::new(
            Point3D::new(0.0, 0.0, -5.0),
            2.0,
            Material::new( 
                Texture::SolidColor(Color::new(1.0,0.0,0.0)),
                0.5,
                MaterialType::Reflective{reflectivity:0.6}
            )
        )));
        escena.objects_list.push(Object::Sphere(Sphere::new(
            Point3D::new(4.0, -4.0, -10.0),
            2.0,
            Material::new( 
                Texture::new_texture("checkerboard.png".to_string()),
                0.5,
                MaterialType::Opaque
            )
        )));
        escena.objects_list.push(Object::Sphere(Sphere::new(
            Point3D::new(4.0, -2.0, -10.0),
            2.0,
            Material::new( 
                Texture::SolidColor(Color::new(1.0,0.0,0.0)),
                0.5,
                MaterialType::Reflective{reflectivity: 0.5}
                
            )
        )));
        escena.objects_list.push(Object::Plane(Plane::new(
            Point3D::new(0.0,-5.0,0.0),
            Vector3D::new(0.0, 1.0, 0.0), 
            Material::new( 
                Texture::new_texture("checkerboard.png".to_string()),
                0.5,
                MaterialType::Reflective{reflectivity:0.6}
            )
        )));
        std::fs::write("esfera_centrada.json",serde_json::to_string(&escena).unwrap()).unwrap();
        escena.render().save("esfera_centrada.png").unwrap();
    }
    #[test]
    fn refraccion_solamente(){
        let mut escena = Scene::new();
        escena.color_de_fondo = Color::from_rgb(255, 72, 33);
        escena.widht = 1500;
        escena.height = 1500;
        escena.lights.push(Light::Directional(DirectionalLight::new_values(Color::new_white(),Vector3D::new(1.0, -1.0, 1.0),200.0)));
        escena.objects_list.push(Object::Sphere(Sphere::new(
            Point3D::new(0.0, -2.0, -10.0),
            2.0,
            Material::new( 
                Texture::SolidColor(Color::new_white()),
                0.5,
                MaterialType::Refractive{refraction_index: 1.3,transparency:0.5}
            )
        )));
        /*escena.objects_list.push(Object::Sphere(Sphere::new(
            Point3D::new(0.0, 5.0, -10.0),
            2.0,
            Material::new( 
                Texture::SolidColor(Color::black()),
                0.5,
                MaterialType::Opaque
            )
        )));*/
        /*escena.objects_list.push(Object::Plane(Plane::new(
            Point3D::new(0.0,-3.0,0.0),
            Vector3D::new(0.0, 1.0, 0.0), 
            Material::new( 
                Texture::SolidColor(Color::new(0.9,0.9,0.9)),
                0.5,
                MaterialType::Refractive{refraction_index: 1.5,transparency: 1.0},
            )
        )));*/
        escena.objects_list.push(Object::Plane(Plane::new(
            Point3D::new(0.0,-5.0,0.0),
            Vector3D::new(0.0, 1.0, 0.0), 
            Material::new( 
                Texture::new_texture(
                    "checkerboard.png".to_string()
                ),
                0.5,
                MaterialType::Opaque,
            )
        )));
        std::fs::write("prueba.json",serde_json::to_string(&escena).unwrap()).unwrap();
        escena.render().save("esfera_refractada.png").unwrap();
    }
    #[test]
    #[ignore]
    fn prueba_texturas() {
        let mut escena = Scene::new();
        escena.color_de_fondo = Color::from_rgb(135, 206, 235);
        escena.widht = 800;
        escena.height = 800;
        escena.lights.push(Light::Directional(DirectionalLight::new_values(Color::new_white(),Vector3D::new(-1.0, -1.0, -1.0),20.0)));
        escena.objects_list.push(Object::Sphere(Sphere::new(
            Point3D::new(0.0, 0.0, -5.0),
            2.0,
            Material::new( 
                Texture::new_texture("checkerboard.png".to_string()),
                    0.5,
                    MaterialType::Opaque
            )
        )));
        escena.objects_list.push(Object::Sphere(Sphere::new(
            Point3D::new(4.0, -4.0, -10.0),
            2.0,
            Material::new( 
                Texture::new_texture("checkerboard.png".to_string()),
                0.5,
                MaterialType::Opaque
            )
        )));
        escena.objects_list.push(Object::Sphere(Sphere::new(
            Point3D::new(4.0, -2.0, -10.0),
            2.0,
            Material::new( 
                Texture::new_texture("checkerboard.png".to_string()),
                    0.5,
                    MaterialType::Opaque
                
            )
        )));
        escena.objects_list.push(Object::Plane(Plane::new(
            Point3D::new(0.0,-5.0,0.0),
            Vector3D::new(0.0, 1.0, 0.0), 
            Material::new( 
                Texture::new_texture("checkerboard.png".to_string()),
                    0.5,
                    MaterialType::Opaque
            )
        )));
        escena.objects_list.push(Object::Plane(Plane::new(
            Point3D::new(0.0,-5.0,-15.0),
            Vector3D::new(0.0, 0.0, 1.0), 
            Material::new( 
                Texture::new_texture("checkerboard.png".to_string()),
                    0.5,
                    MaterialType::Opaque
            )
        )));
        escena.objects_list.push(Object::Plane(Plane::new(
            Point3D::new(-4.0,0.0,0.0),
            Vector3D::new(1.0, 0.0, 0.0), 
            Material::new( 
                Texture::new_texture("checkerboard.png".to_string()),
                    0.5,
                    MaterialType::Opaque
            )
        )));
        std::fs::write("texturas.json",serde_json::to_string(&escena).unwrap()).unwrap();
        escena.render().save("texturas.png").unwrap();
    }
    #[test]
    #[ignore]
    fn luz_puntual() {
        let mut escena = Scene::new();
        escena.widht = 800;
        escena.height = 800;
        escena.lights.push(
            Light::Spherical(
                SphericalLight::new(
                    Point3D::new(0.0, 5.0, 0.0), 
                    Color::new_white(), 
                    2000000.0,
                )
            )
        );
        escena.objects_list.push(Object::Sphere(Sphere::new(
            Point3D::new(0.0, 0.0, -5.0),
            2.0,
            Material::new( 
                Texture::SolidColor(Color::new(1.0,0.0,0.0)),
                0.5,
                MaterialType::Opaque
            )
        )));
        escena.objects_list.push(Object::Sphere(Sphere::new(
            Point3D::new(4.0, -4.0, -10.0),
            2.0,
            Material::new( 
                Texture::new_texture("checkerboard.png".to_string()),
                0.5,
                MaterialType::Opaque
            )
        )));
        escena.objects_list.push(Object::Sphere(Sphere::new(
            Point3D::new(4.0, -2.0, -10.0),
            2.0,
            Material::new( 
                Texture::SolidColor(Color::new(1.0,0.0,0.0)),
                0.5,
                MaterialType::Reflective{reflectivity: 0.5}
                
            )
        )));
        escena.objects_list.push(Object::Plane(Plane::new(
            Point3D::new(0.0,-5.0,0.0),
            Vector3D::new(0.0, 1.0, 0.0), 
            Material::new( 
                Texture::new_texture("checkerboard.png".to_string()),
                0.5,
                MaterialType::Opaque
            )
        )));
        std::fs::write("puntual.json",serde_json::to_string(&escena).unwrap()).unwrap();
        escena.render().save("luz_puntual.png").unwrap();
    }
}