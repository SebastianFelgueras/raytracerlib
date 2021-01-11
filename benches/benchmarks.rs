use criterion::{
    black_box, criterion_group, criterion_main, Criterion
};
use raytracerlib::{
    Scene,
};
pub fn criterion_benchmark(c: &mut Criterion){
    let mut escena: Scene = serde_json::from_str(&std::fs::read_to_string("benches\\test_4_esferas_fullHD.json").unwrap()).unwrap();
    c.bench_function("4 esferas y un plano con texturas en 800x800",|b| b.iter(||{
        black_box(escena.render());
    }));
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);