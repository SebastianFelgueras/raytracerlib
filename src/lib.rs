//! NOTICE THAT THE HOLE LIB OPERATES IN THE ASSUMPTION THAT THE CAMERA IS IN (0,0,0)

pub mod maths;
pub mod objects;
pub mod color;
use objects::Ray;
use image::{DynamicImage, GenericImage};
pub struct Scene{
    pub height: u32, //u32 porque eso es lo que exige imagebuffer como argumento
    pub widht: u32,
    pub light_sources: objects::DirectionalLight,
    pub objects_list: Vec<Box<dyn objects::SceneObject>>,
    pub rendering_max_distance: usize,
    image_rendered: Option<DynamicImage>,
}
impl Scene{
    ///Creates a new scene with the source of light in the sky, above the camera. By default, it's rendering in a low definition
    #[inline]
    pub fn new()->Self{
        Scene{
            height: 480,
            widht: 720,
            light_sources: objects::DirectionalLight::new(),
            objects_list: Vec::new(),
            rendering_max_distance: 100,
            image_rendered: None,
        }
    }
    pub fn render(&mut self){
        let mut imagen = DynamicImage::new_rgb8(self.widht,self.height);
        for x in 0..self.widht{
            for y in 0..self.height{
                let rayo_actual = Ray::new_camera_ray(x,y,&self);
                let mut minimum_distance_to_intersection = (0.0,true);
                for objeto in &self.objects_list{
                    let interseccion = objeto.intersects(&rayo_actual,&self);
                    if let Some(interseccion) = interseccion{
                        if interseccion.distance()<minimum_distance_to_intersection.0 || minimum_distance_to_intersection.1 {
                            imagen.put_pixel(x, y, interseccion.rgba());
                            minimum_distance_to_intersection.0 = interseccion.distance();
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
            let interseccion = objeto.intersects(&ray,&self);
            if interseccion.is_none(){
                 continue;
            }else{
                return true;
            }
        }
        false
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
    };
    use maths::point::Point3D;
    use maths::vector3::Vector3D;
    use color::Color;
    #[test]
    fn esfera_centrada() {
        let mut escena = Scene::new();
        escena.widht = 800;
        escena.height = 800;
        escena.light_sources = DirectionalLight::new_values(Color::new(0.5, 0.5, 0.25),Vector3D::new(0.5, -1.0, 0.25),20.0);
        escena.objects_list.push(Box::new(Sphere::new()));
        escena.objects_list.push(Box::new(Sphere::new_with_coordinates(
            Point3D::new(4.0, -4.0, -10.0),
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