use criterion::{
    black_box, criterion_group, criterion_main, Criterion
};
use raytracerlib::{
    objects::{
        objects::*,
        DirectionalLight,
        Light,
        SphericalLight,
    },
    maths::{
        point::Point3D,
        vector3::Vector3D,
    },
    color::Color,
    textures::{
        Texture,
    },
    *,
};
pub fn criterion_benchmark(c: &mut Criterion){
    c.bench_function("3esferas y un plano con texturas",|b| b.iter(||{
            let mut escena = Scene::new();
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
            escena.objects_list.push(Box::new(Sphere::new(
                Point3D::new(0.0, 0.0, -5.0),
                2.0,
                Texture::Texture(
                    image::open("checkerboard.png").unwrap()
                ),
                0.5
            )));
            escena.objects_list.push(Box::new(Sphere::new(
                Point3D::new(4.0, -4.0, -10.0),
                2.0,
                Texture::SolidColor(Color::new(1.0,0.0,0.0)),
                0.5
            )));
            escena.objects_list.push(Box::new(Sphere::new(
                Point3D::new(4.0, -2.0, -10.0),
                2.0,
                Texture::SolidColor(Color::new(1.0,0.0,0.0)),
                0.5
            )));
            escena.objects_list.push(Box::new(Plane::new(
                Point3D::new(0.0,-5.0,0.0),
                Vector3D::new(0.0, 1.0, 0.0), 
                Texture::Texture(
                    image::open("checkerboard.png").unwrap()
                ),
                0.5
            )));
            escena.render();
    }));
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);