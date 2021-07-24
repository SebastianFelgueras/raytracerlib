use std::{
    env::args,
    process::exit,
};

fn main() {
    let mut argumentos = args().skip(1);
    if argumentos.len() != 2{
        println!("Ingrese el la ruta del archivo json con la escena a parsear y la ruta del archivo destino como argumentos");
        exit(1);
    }
    let json = argumentos.next().unwrap();
    let destino = argumentos.next().unwrap();
    let escena: raytracerlib::Scene = serde_json::from_str(&std::fs::read_to_string(json.trim_end()).unwrap()).unwrap();
    escena.render().save(destino.trim_end()).unwrap();
}
