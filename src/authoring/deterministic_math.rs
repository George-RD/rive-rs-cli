use std::f64::consts::PI;

const RADIANS_PER_DEGREE: f64 = PI / 180.0;

pub(crate) fn radians_from_degrees(degrees: f64) -> f64 {
    degrees * RADIANS_PER_DEGREE
}

pub(crate) fn sin_cos(radians: f64) -> (f64, f64) {
    (libm::sin(radians), libm::cos(radians))
}

#[cfg(test)]
mod tests {
    use super::{radians_from_degrees, sin_cos};

    #[test]
    fn degree_conversion_has_pinned_bits() {
        assert_eq!(radians_from_degrees(30.0).to_bits(), 0x3fe0_c152_382d_7365);
    }

    #[test]
    fn trigonometry_has_pinned_bits() {
        let angle = f64::from_bits(0xbfe9_0003_ce1c_711f);
        let (sine, cosine) = sin_cos(angle);
        assert_eq!(sine.to_bits(), 0xbfe6_888d_01ba_048a);
        assert_eq!(cosine.to_bits(), 0x3fe6_b896_4cae_d975);
    }
}
