#![allow(dead_code)]

use std::f32::consts::FRAC_1_SQRT_2;

fn quantize_unit_quaternion_component_to_i16(value: f32) -> i16 {
    ((value / FRAC_1_SQRT_2) * i16::MAX as f32) as i16
}

fn unquantize_unit_quaternion_component_from_i16(value: i16) -> f32 {
    value as f32 * FRAC_1_SQRT_2 / i16::MAX as f32
}

/// Encode a quaternion into 7 bytes using smallest three technique.
pub fn encode_quaternion(x: f32, y: f32, z: f32, w: f32) -> [u8; 7] {
    let rotation = &[x, y, z, w];

    let mut largest_component_index = 0;
    let mut largest_component_sign = x.is_sign_positive();

    for i in 1..4 {
        if rotation[i].abs() > rotation[largest_component_index].abs() {
            largest_component_index = i;
            largest_component_sign = rotation[i].is_sign_positive();
        }
    }

    let mut quantized_components = [largest_component_index as u8; 7];

    let mut index = 1;
    for (i, value) in rotation.iter().enumerate() {
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
pub fn decode_quaternion(encoded: [u8; 7]) -> (f32, f32, f32, f32) {
    let mut quaternion = [0.0f32; 4];

    let mut index = 1;
    for (i, value) in quaternion.iter_mut().enumerate() {
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

    (quaternion[0], quaternion[1], quaternion[2], quaternion[3])
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::{Distribution, Uniform};

    #[test]
    fn quaternions() {
        let mut rng = rand::thread_rng();

        let range = Uniform::new_inclusive(-1.0, 1.0);

        let precision = 0.0001;

        for _ in 0..1000000 {
            let before_quaternion = random_quaternion(&mut rng, &range);

            let encoded = encode_quaternion(
                before_quaternion.0,
                before_quaternion.1,
                before_quaternion.2,
                before_quaternion.3,
            );

            let after_quaternion = decode_quaternion(encoded);

            let status = (after_quaternion.0.abs() - before_quaternion.0.abs()) > precision
                || (after_quaternion.1.abs() - before_quaternion.1.abs()) > precision
                || (after_quaternion.2.abs() - before_quaternion.2.abs()) > precision
                || (after_quaternion.3.abs() - before_quaternion.3.abs()) > precision;

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
        rng: &mut rand::rngs::ThreadRng,
        range: &rand::distributions::uniform::Uniform<f32>,
    ) -> (f32, f32, f32, f32) {
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

        (x, y, s * u, s * v)
    }
}
