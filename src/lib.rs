//! NOTICE THAT THE HOLE LIB OPERATES IN THE ASSUMPTION THAT THE CAMERA IS IN (0,0,0) 
//! AND WITH Y POSITIVE AXIS UP, X POSITIVE AXIS ON THE RIGHT AND Z NEGATIVE AXIS ON FRONT

pub mod maths;
pub mod objects;
pub mod color;
use maths::vector3::Vector3;
use maths::point::Point3D;
use objects::Ray;
use image::{DynamicImage, GenericImage};
pub struct Scene{
    pub height: u32, //u32 porque eso es lo que exige imagebuffer como argumento
    pub widht: u32,
    pub lights: Vec<objects::Light>,
    pub objects_list: Vec<Box<dyn objects::SceneObject>>,
    pub rendering_max_distance: usize,
    image_rendered: Option<DynamicImage>,
    shadow_bias: f64,
}
impl Scene{
    ///Creates a new scene with the source of light in the sky, above the camera. By default, it's rendering in a low definition
    #[inline]
    pub fn new()->Self{
        Scene{
            height: 480,
            widht: 720,
            lights: Vec::new(),
            objects_list: Vec::new(),
            rendering_max_distance: 100,
            image_rendered: None,
            shadow_bias: 1e-13,
        }
    }
    pub fn render(&mut self){
        let mut imagen = DynamicImage::new_rgb8(self.widht,self.height);
        for x in 0..self.widht{
            for y in 0..self.height{
                let rayo_actual = Ray::new_camera_ray(x,y,&self);
                let mut minimum_distance_to_intersection = (0.0,true);
                for objeto in &self.objects_list{
                    if objeto.intersects(&rayo_actual){
                        let hit_point = objeto.intersection_point(&rayo_actual).unwrap();
                        let hit_point_module = hit_point.module();
                        if hit_point_module<minimum_distance_to_intersection.0 || minimum_distance_to_intersection.1 {
                            imagen.put_pixel(x, y, objeto.color_at_intersection(&hit_point,&self).to_rgba(255));
                            minimum_distance_to_intersection.0 = hit_point_module;
                            minimum_distance_to_intersection.1 = false;
                        }
                        
                    }
                }
            }
        }
        self.image_rendered = Some(imagen);
    }
    fn object_between(&self,ray: &Ray)->bool{
        for objeto in &self.objects_list{
            if objeto.intersects(&ray) {
                 return true;
            }
        }
        false
    }
    fn object_between_with_point(&self,ray:&Ray)->Vec<Point3D>{
        let mut intersecciones = Vec::new();
        for objeto in &self.objects_list{
            if objeto.intersects(&ray){
                intersecciones.push(objeto.intersection_point(&ray).unwrap());
            }
        }
        intersecciones
    }
    pub fn unwrap_image(self)->DynamicImage{
        self.image_rendered.unwrap()
    }
    pub fn image(&self)->&Option<DynamicImage>{
        &self.image_rendered
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use objects::{
        objects::*,
        DirectionalLight,
        Light,
        SphericalLight,
    };
    use maths::point::Point3D;
    use maths::vector3::Vector3D;
    use color::Color;
    #[test]
    fn esfera_centrada() {
        let mut escena = Scene::new();
        escena.widht = 800;
        escena.height = 800;
        escena.lights.push(Light::Directional(DirectionalLight::new_values(Color::new(0.1,0.1, 0.2),Vector3D::new(0.5, -1.0, 0.25),20.0)));
        escena.lights.push(Light::Directional(DirectionalLight::new_values(Color::new(0.05, 0.25, 0.15),Vector3D::new(0.0, -1.0, 0.0),20.0)));
        escena.lights.push(
            Light::Spherical(
                SphericalLight::new(
                    Point3D::new(0.0, 6.0, -5.0), 
                    Color::new(0.3,0.25,0.09), 
                    20000.0,
                )
            )
        );
        escena.objects_list.push(Box::new(Sphere::new()));
        escena.objects_list.push(Box::new(Sphere::new_with_coordinates(
            Point3D::new(4.0, -4.0, -10.0),
            2.0,
            Color::new(1.0,0.0,0.0),
            0.5
        )));
        escena.objects_list.push(Box::new(Sphere::new_with_coordinates(
            Point3D::new(4.0, -2.0, -10.0),
            2.0,
            Color::new(1.0,0.0,0.0),
            0.5
        )));
        escena.objects_list.push(Box::new(Plane::new(
            Point3D::new(0.0,-5.0,0.0),
            Vector3D::new(0.0, 1.0, 0.0), 
            Color::new(0.8,0.8,0.8),
            0.5
        )));
        escena.render();
        escena.unwrap_image().save("esfera_centrada.png").unwrap();
    }
}