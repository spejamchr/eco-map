extern crate noise;

use crate::noise::utils::NoiseMapBuilder;
use crate::noise::MultiFractal;
use noise::utils::PlaneMapBuilder;
use noise::HybridMulti;

fn main() {
    let n = HybridMulti::default().set_octaves(8).set_persistence(0.5);

    PlaneMapBuilder::new(&n)
        .set_size(1000, 1000)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("perlin.png");
}
