//! NOTICE THAT THE HOLE LIB OPERATES IN THE ASSUMPTION THAT THE CAMERA IS IN (0,0,0)

pub mod maths;
pub mod objects;
pub mod color;
use objects::Ray;
use image::{DynamicImage, GenericImage};
pub struct Scene{
    pub height: u32, //u32 porque eso es lo que exige imagebuffer como argumento
    pub widht: u32,
    pub light_source: maths::point::Point3D,
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
            light_source: maths::point::Point3D::new(0.0,10.0,0.0),
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
                for objeto in &self.objects_list{
                    let interseccion = objeto.intersects(&rayo_actual); //que pasa si dos lo intersectan en planos distintos?
                    if let Some(interseccion) = interseccion{
                        imagen.put_pixel(x, y, interseccion.rgba());
                    }
                }
            }
        }
        self.image_rendered = Some(imagen);
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
        Ray,
    };
    use maths::point::Point3D;
    use maths::vector3::Vector3D;
    use color::Color;
    #[test]
    fn esfera_centrada() {
        let mut escena = Scene::new();
        escena.widht = 800;
        escena.height = 800;
        escena.objects_list.push(Box::new(objects::objects::Sphere::new()));
        escena.objects_list.push(Box::new(objects::objects::Sphere::new_with_coordinates(
            Point3D::new(2.0, -2.0, -10.0),
            2.0,
            Color::new_rgb(255,0,0)
        )));
        escena.render();
        escena.unwrap_image().save("esfera_centrada.png").unwrap();
    }
}