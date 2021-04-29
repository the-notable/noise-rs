pub struct ChainBuilder {}

impl ChainBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use crate::{Blend, Seedable, MultiFractal, Constant, NoiseFn, Turbulence, RidgedMulti, Cache};
    use std::rc::Rc;
    use crate::utils::{PlaneMapBuilder, ImageRenderer, NoiseMapBuilder, ColorGradient};

    #[test]
    fn chain_builder_test() {

        #[allow(non_snake_case)]
            let mtn_base_mod = || -> Box<dyn NoiseFn<_, 3>> {

            const CURRENT_SEED: u32 = 0;
            const MOUNTAIN_LACUNARITY: f64 = 2.142578125;
            const MOUNTAINS_TWIST: f64 = 1.0;

            let mountainBaseDef_rm0 = Rc::new(RidgedMulti::new()
                .set_seed(CURRENT_SEED + 30)
                .set_frequency(1723.0)
                .set_lacunarity(MOUNTAIN_LACUNARITY)
                .set_octaves(4));

            // let mountainBaseDef_sb0 = ScaleBias::new(&mountainBaseDef_rm0)
            //     .set_scale(0.5)
            //     .set_bias(0.375);

            let mountainBaseDef_rm1 = Rc::new(RidgedMulti::new()
                .set_seed(CURRENT_SEED + 31)
                .set_frequency(367.0)
                .set_lacunarity(MOUNTAIN_LACUNARITY)
                .set_octaves(1));

            // let mountainBaseDef_sb1 = ScaleBias::new(&mountainBaseDef_rm1)
            //     .set_scale(-2.0)
            //     .set_bias(-0.5);

            let mountainBaseDef_co = Rc::new(Constant::new(-1.0));

            let mountainBaseDef_bl = Rc::new(Blend::new(
                mountainBaseDef_co.clone(),
                mountainBaseDef_rm0.clone(),
                mountainBaseDef_rm1.clone(),
            ));

            let mountainBaseDef_tu0 = Rc::new(Turbulence::new(mountainBaseDef_bl)
                .set_seed(CURRENT_SEED + 32)
                .set_frequency(1337.0)
                .set_power(1.0 / 6730.0 * MOUNTAINS_TWIST)
                .set_roughness(4));

            let mountainBaseDef_tu1 = Rc::new(Turbulence::new(mountainBaseDef_tu0)
                .set_seed(CURRENT_SEED + 33)
                .set_frequency(21221.0)
                .set_power(1.0 / 120157.0 * MOUNTAINS_TWIST)
                .set_roughness(6));

            let mountainBaseDef = Cache::new(mountainBaseDef_tu1);

            Box::new(mountainBaseDef)
        };

        let noise_map = PlaneMapBuilder::new(&*mtn_base_mod())
            .set_size(100, 100)
            .set_x_bounds(-0.0, 0.25)
            .set_y_bounds(-0.125, 0.125)
            .build();

        ImageRenderer::new()
            .set_gradient(ColorGradient::new().build_terrain_gradient())
            .render(&noise_map)
            .write_to_file("unscaledFinalPlanet_16x_zoom.png");
    }
}