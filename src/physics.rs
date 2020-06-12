#![allow(dead_code)]

use crate::quantizer::quantize_to_i16;

pub fn encode_absolute_position(x: f32, y: f32, z: f32) -> [u8; 12] {
    let x = x.to_be_bytes();
    let y = y.to_be_bytes();
    let z = z.to_be_bytes();

    [
        x[0], x[1], x[2], x[3], y[0], y[1], y[2], y[3], z[0], z[1], z[2], z[3],
    ]
}

pub fn decode_absolute_position(encoded: [u8; 12]) -> (f32, f32, f32) {
    let x = f32::from_be_bytes([encoded[0], encoded[1], encoded[2], encoded[3]]);
    let y = f32::from_be_bytes([encoded[4], encoded[5], encoded[6], encoded[7]]);
    let z = f32::from_be_bytes([encoded[8], encoded[9], encoded[10], encoded[11]]);

    (x, y, z)
}

/// Encode a 3D velocity into 6 bytes.
pub fn encode_velocity(x: f32, y: f32, z: f32, max_vel: f32) -> [u8; 6] {
    let x = quantize_to_i16(x, -max_vel, max_vel).to_be_bytes();
    let y = quantize_to_i16(y, -max_vel, max_vel).to_be_bytes();
    let z = quantize_to_i16(z, -max_vel, max_vel).to_be_bytes();

    [x[0], x[1], y[0], y[1], z[0], z[1]]
}

/// Decode 6 bytes into a 3D velocity.
pub fn decode_velocity(encoded: [u8; 6], max_vel: f32) -> (f32, f32, f32) {
    let x = max_vel * i16::from_be_bytes([encoded[0], encoded[1]]) as f32 / i16::MAX as f32;
    let y = max_vel * i16::from_be_bytes([encoded[2], encoded[3]]) as f32 / i16::MAX as f32;
    let z = max_vel * i16::from_be_bytes([encoded[4], encoded[5]]) as f32 / i16::MAX as f32;

    (x, y, z)
}

/// Encode a position relative to the player into 6 bytes.
pub fn encode_relative_position(x: f32, y: f32, z: f32) -> [u8; 6] {
    let x = quantize_to_i16(x, i16::MIN as f32, i16::MAX as f32).to_be_bytes();
    let y = quantize_to_i16(y, i16::MIN as f32, i16::MAX as f32).to_be_bytes();
    let z = quantize_to_i16(z, i16::MIN as f32, i16::MAX as f32).to_be_bytes();

    [x[0], x[1], y[0], y[1], z[0], z[1]]
}

/// Decode 6 bytes into a Vector3 representing a position relative to the ship.
pub fn decode_relative_position(encoded: [u8; 6]) -> (f32, f32, f32) {
    let x = i16::from_be_bytes([encoded[0], encoded[1]]) as f32;
    let y = i16::from_be_bytes([encoded[2], encoded[3]]) as f32;
    let z = i16::from_be_bytes([encoded[4], encoded[5]]) as f32;

    (x, y, z)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::uniform::Uniform;
    use rand::distributions::Distribution;
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256StarStar;

    #[test]
    fn absolute_position() {
        let mut rng = Xoshiro256StarStar::from_entropy();

        let range = Uniform::new_inclusive(f32::MIN / 2.0, f32::MAX / 2.0);

        let precision = 0.0001;

        for _ in 0..1000000 {
            let before = (
                range.sample(&mut rng),
                range.sample(&mut rng),
                range.sample(&mut rng),
            );

            let encoded = encode_absolute_position(before.0, before.1, before.2);

            let after = decode_absolute_position(encoded);

            let status = (after.0.abs() - before.0.abs()) > precision
                || (after.1.abs() - before.1.abs()) > precision
                || (after.2.abs() - before.2.abs()) > precision;

            if status {
                println!("Before {:#?} != After {:#?}", before, after);
            }

            assert_eq!(status, false);
        }
    }

    #[test]
    fn relative_position() {
        let mut rng = Xoshiro256StarStar::from_entropy();

        let range = Uniform::new_inclusive(i16::MIN as f32, i16::MAX as f32);

        let precision = 0.0001;

        for _ in 0..1000000 {
            let before = (
                range.sample(&mut rng),
                range.sample(&mut rng),
                range.sample(&mut rng),
            );

            let encoded = encode_relative_position(before.0, before.1, before.2);

            let after = decode_relative_position(encoded);

            let status = (after.0.abs() - before.0.abs()) > precision
                || (after.1.abs() - before.1.abs()) > precision
                || (after.2.abs() - before.2.abs()) > precision;

            if status {
                println!("Before {:#?} != After {:#?}", before, after);
            }

            assert_eq!(status, false);
        }
    }

    #[test]
    fn velocity() {
        let mut rng = Xoshiro256StarStar::from_entropy();

        let range = Uniform::new_inclusive(0.0, 144.0);

        let precision = 0.0001;

        for _ in 0..1000000 {
            let before = (
                range.sample(&mut rng),
                range.sample(&mut rng),
                range.sample(&mut rng),
            );

            let encoded = encode_velocity(before.0, before.1, before.2, 144.0);

            let after = decode_velocity(encoded, 144.0);

            let status = (after.0.abs() - before.0.abs()) > precision
                || (after.1.abs() - before.1.abs()) > precision
                || (after.2.abs() - before.2.abs()) > precision;

            if status {
                println!("Before {:#?} != After {:#?}", before, after);
            }

            assert_eq!(status, false);
        }
    }
}
