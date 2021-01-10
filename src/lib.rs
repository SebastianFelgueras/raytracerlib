//! NOTICE THAT THE HOLE LIB OPERATES IN THE ASSUMPTION THAT THE CAMERA IS IN (0,0,0) 
//! AND WITH Y POSITIVE AXIS UP, X POSITIVE AXIS ON THE RIGHT AND Z NEGATIVE AXIS ON FRONT
use serde::{Serialize, Deserialize};
pub mod maths;
pub mod objects;
pub mod color;
pub mod textures;
pub mod material;
use maths::vector3::Vector3;
use maths::point::Point3D;
use color::Color;
use objects::Ray;
use image::{DynamicImage, GenericImage};
pub use image;
#[derive(Serialize,Deserialize)]
pub struct Scene{
    pub height: u32, //u32 porque eso es lo que exige imagebuffer como argumento
    pub widht: u32,
    pub lights: Vec<objects::Light>,
    pub objects_list: Vec<objects::Object>,
    pub max_reflections: usize,
    pub color_de_fondo: color::Color,
    pub indice_refraccion_medio: f64,
    shadow_bias: f64,
}
impl Scene{
    ///Creates a new scene with the source of light in the sky, above the camera. By default, it's rendering in a low definition
    #[inline]
    pub fn new()->Self{
        Scene{
            height: 800,
            widht: 800,
            lights: Vec::new(),
            objects_list: Vec::new(),
            max_reflections: 5,
            shadow_bias: 1e-13,
            indice_refraccion_medio: 1.0,
            color_de_fondo: color::Color::black(),
        }
    }
    pub fn render(&mut self)->DynamicImage{
        let mut imagen = DynamicImage::new_rgb8(self.widht,self.height);
        for x in 0..self.widht{
            for y in 0..self.height{
                let rayo_actual = Ray::new_camera_ray(x,y,&self);
                match self.ray_caster(&rayo_actual,0){
                    Some(valor)=>imagen.put_pixel(x, y, valor.to_rgba(255)),
                    None=>imagen.put_pixel(x, y, self.color_de_fondo.clone().to_rgba(255)),
                }
            }
        }
        imagen
    }
    fn object_between(&self,ray: &Ray)->bool{
        for objeto in self.parse_array(){
            if objeto.intersection_point(&ray).is_some() {
                 return true;
            }
        }
        false
    }
    fn object_between_with_point(&self,ray:&Ray)->Vec<Point3D>{
        let mut intersecciones = Vec::new();
        for objeto in &self.parse_array(){
            if let Some(valor) = objeto.intersection_point(&ray){
                intersecciones.push(valor);
            }
        }
        intersecciones
    }
    //esto es ineficiente pero me evita tener que reescribir mucho codigo
    #[inline]
    fn parse_array(&self)->Vec<Box<&dyn objects::SceneObject>>{
        let mut vector: Vec<Box<&dyn objects::SceneObject>> = Vec::new();
        for objeto in &self.objects_list{
            match objeto{
                objects::Object::Plane(valor)=>vector.push(Box::new(valor)),
                objects::Object::Sphere(valor)=>vector.push(Box::new(valor)),
            }
        }
        vector
    }
    fn ray_caster(&self,rayo: &Ray,current_iteration: usize)->Option<Color>{
        let mut temp = None;
        let mut minimum_distance_to_intersection = (0.0,true);
        for objeto in self.parse_array(){
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
                MaterialType::Refractive{refraction_index: 1.5,transparency:0.6}
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
        escena.render().save("esfera_centrada.png").unwrap();
    }
    #[test]
    #[ignore]
    fn refraccion_solamente(){
        let mut escena = Scene::new();
        escena.color_de_fondo = Color::from_rgb(135, 206, 235);
        escena.widht = 800;
        escena.height = 800;
        escena.lights.push(Light::Directional(DirectionalLight::new_values(Color::new(0.1,0.1, 0.2),Vector3D::new(0.5, -1.0, 0.25),20.0)));
        escena.objects_list.push(Object::Sphere(Sphere::new(
            Point3D::new(0.0, 0.0, -5.0),
            2.0,
            Material::new( 
                Texture::SolidColor(Color::new(1.0,0.0,0.0)),
                0.5,
                MaterialType::Refractive{refraction_index: 0.5,transparency:0.6}
            )
        )));
        escena.objects_list.push(Object::Plane(Plane::new(
            Point3D::new(0.0,-5.0,0.0),
            Vector3D::new(0.0, 1.0, 0.0), 
            Material::new( 
                Texture::new_texture("checkerboard.png".to_string()),
                0.5,
                MaterialType::Opaque,
            )
        )));
        escena.render().save("esfera_refractada.png").unwrap();
    }
}