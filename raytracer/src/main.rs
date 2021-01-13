fn main() {
    println!("Ingrese el la ruta del archivo json con la escena a parsear y la ruta del archivo destino separados por un enter");
    let mut json = String::new();
    let mut destino = String::new();
    std::io::stdin().read_line(&mut json).unwrap();
    std::io::stdin().read_line(&mut destino).unwrap();
    let escena: raytracerlib::Scene = serde_json::from_str(&std::fs::read_to_string(json.trim_end()).unwrap()).unwrap();
    escena.render().save(destino.trim_end()).unwrap();
}
