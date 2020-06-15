#![allow(dead_code)]

use nalgebra::{Quaternion, UnitQuaternion};
use std::f32::consts::FRAC_1_SQRT_2;

fn quantize_unit_quaternion_component_to_i16(value: f32) -> i16 {
    ((value / FRAC_1_SQRT_2) * i16::MAX as f32) as i16
}

fn unquantize_unit_quaternion_component_from_i16(value: i16) -> f32 {
    value as f32 * FRAC_1_SQRT_2 / i16::MAX as f32
}

/// Encode a quaternion into 7 bytes using smallest three technique.
pub fn encode_quaternion(quaternion: UnitQuaternion<f32>) -> [u8; 7] {
    let mut largest_component_index = 0;
    let mut largest_component_sign = quaternion.coords.x.is_sign_positive();

    for (i, value) in quaternion.coords.iter().enumerate() {
        if value.abs() > quaternion.coords[largest_component_index].abs() {
            largest_component_index = i;
            largest_component_sign = value.is_sign_positive();
        }
    }

    let mut quantized_components = [largest_component_index as u8; 7];

    let mut index = 1;
    for (i, value) in quaternion.coords.iter().enumerate() {
        if i == largest_component_index {
            continue;
        }

        let value = {
            if !largest_component_sign {
                -*value
            } else {
                *value
            }
        };

        let bytes = quantize_unit_quaternion_component_to_i16(value).to_be_bytes();

        quantized_components[index] = bytes[0];
        quantized_components[index + 1] = bytes[1];

        index += 2;
    }

    quantized_components
}

/// Decode 7 bytes into a quaternion using smallest three technique.
pub fn decode_quaternion(encoded: [u8; 7]) -> UnitQuaternion<f32> {
    let mut quaternion = Quaternion::identity();

    let mut index = 1;
    for (i, value) in quaternion.coords.iter_mut().enumerate() {
        if i == encoded[0] as usize {
            *value = (1.0
                - unquantize_unit_quaternion_component_from_i16(i16::from_be_bytes([
                    encoded[1], encoded[2],
                ]))
                .powi(2)
                - unquantize_unit_quaternion_component_from_i16(i16::from_be_bytes([
                    encoded[3], encoded[4],
                ]))
                .powi(2)
                - unquantize_unit_quaternion_component_from_i16(i16::from_be_bytes([
                    encoded[5], encoded[6],
                ]))
                .powi(2))
            .sqrt();
        } else {
            *value = unquantize_unit_quaternion_component_from_i16(i16::from_be_bytes([
                encoded[index],
                encoded[index + 1],
            ]));

            index += 2;
        }
    }

    UnitQuaternion::from_quaternion(quaternion)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::uniform::Uniform;
    use rand::distributions::Distribution;
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256StarStar;

    #[test]
    fn quaternions() {
        let mut rng = Xoshiro256StarStar::from_entropy();

        let range = Uniform::new_inclusive(-1.0, 1.0);

        let precision = 0.0001;

        for _ in 0..1000000 {
            let before_quaternion = random_quaternion(&mut rng, &range);

            let encoded = encode_quaternion(before_quaternion);

            let after_quaternion = decode_quaternion(encoded);

            let status = (after_quaternion.coords.x.abs() - before_quaternion.coords.x.abs())
                > precision
                || (after_quaternion.coords.y.abs() - before_quaternion.coords.y.abs()) > precision
                || (after_quaternion.coords.z.abs() - before_quaternion.coords.z.abs()) > precision
                || (after_quaternion.coords.w.abs() - before_quaternion.coords.w.abs()) > precision;

            if status {
                println!(
                    "Before {:#?} != After {:#?}",
                    before_quaternion, after_quaternion
                );
            }

            assert_eq!(status, false);
        }
    }

    #[allow(clippy::many_single_char_names)]
    fn random_quaternion(
        rng: &mut Xoshiro256StarStar,
        range: &Uniform<f32>,
    ) -> UnitQuaternion<f32> {
        let (x, y, z) = loop {
            let x: f32 = range.sample(rng);
            let y: f32 = range.sample(rng);
            let z: f32 = x * x + y * y;

            if z > 1.0 {
                continue;
            } else {
                break (x, y, z);
            }
        };

        let (u, v, w) = loop {
            let u: f32 = range.sample(rng);
            let v: f32 = range.sample(rng);
            let w: f32 = u * u + v * v;

            if w > 1.0 {
                continue;
            } else {
                break (u, v, w);
            }
        };

        let s: f32 = ((1.0 - z) / w).sqrt();

        let mut quaternion = Quaternion::identity();

        quaternion.coords.x = x;
        quaternion.coords.y = y;
        quaternion.coords.z = s * u;
        quaternion.coords.w = s * v;

        UnitQuaternion::from_quaternion(quaternion)
    }
}
