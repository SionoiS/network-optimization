#![allow(dead_code)]

/// Quantize any value to range 0..255
pub fn quantize_to_byte(value: f32, min: f32, max: f32) -> u8 {
    ((value - min) / (max - min) * u8::MAX as f32) as u8
}

/// Quantize any value to range 0..65535
pub fn quantize_to_u16(value: f32, min: f32, max: f32) -> u16 {
    ((value - min) / (max - min) * u16::MAX as f32) as u16
}

/// Quantize any value to range -32768..32767
pub fn quantize_to_i16(value: f32, min: f32, max: f32) -> i16 {
    (((value - min) / (max - min) * u16::MAX as f32) + i16::MIN as f32) as i16
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::{Distribution, Uniform};

    #[test]
    fn random_to_u8() {
        let mut rng = rand::thread_rng();

        let min_range = Uniform::new(f32::MIN / 2.0, 0.0);
        let max_range = Uniform::new_inclusive(0.0, f32::MAX / 2.0);

        for _ in 0..1000000 {
            let min = min_range.sample(&mut rng);
            let max = max_range.sample(&mut rng);

            let result = quantize_to_byte(min, min, max);

            assert_eq!(result, u8::MIN);

            let result = quantize_to_byte(max, min, max);

            assert_eq!(result, u8::MAX);

            let half = ((max - min) / 2.0) + min;
            let result = quantize_to_byte(half, min, max);

            assert_eq!(result, u8::MAX / 2);

            let low_quart = ((max - min) / 4.0) + min;
            let result = quantize_to_byte(low_quart, min, max);

            assert_eq!(result, u8::MAX / 4);

            let up_quart = max - ((max - min) / 4.0);
            let result = quantize_to_byte(up_quart, min, max);

            assert_eq!(result, (u8::MAX / 2) + (u8::MAX / 4) + 1);
        }
    }

    #[test]
    fn random_to_u16() {
        let mut rng = rand::thread_rng();

        let min_range = Uniform::new(f32::MIN / 2.0, 0.0);
        let max_range = Uniform::new_inclusive(0.0, f32::MAX / 2.0);

        for _ in 0..1000000 {
            let min = min_range.sample(&mut rng);
            let max = max_range.sample(&mut rng);

            let result = quantize_to_u16(min, min, max);

            assert_eq!(result, u16::MIN);

            let result = quantize_to_u16(max, min, max);

            assert_eq!(result, u16::MAX);

            let half = ((max - min) / 2.0) + min;
            let result = quantize_to_u16(half, min, max);

            assert_eq!(result, u16::MAX / 2);

            let low_quart = ((max - min) / 4.0) + min;
            let result = quantize_to_u16(low_quart, min, max);

            assert_eq!(result, u16::MAX / 4);

            let up_quart = max - ((max - min) / 4.0);
            let result = quantize_to_u16(up_quart, min, max);

            assert_eq!(result, (u16::MAX / 2) + (u16::MAX / 4) + 1);
        }
    }

    #[test]
    fn random_to_i16() {
        let mut rng = rand::thread_rng();

        let min_range = Uniform::new(f32::MIN / 2.0, 0.0);
        let max_range = Uniform::new_inclusive(0.0, f32::MAX / 2.0);

        for _ in 0..1000000 {
            let min = min_range.sample(&mut rng);
            let max = max_range.sample(&mut rng);

            let result = quantize_to_i16(min, min, max);

            assert_eq!(result, i16::MIN);

            let result = quantize_to_i16(max, min, max);

            assert_eq!(result, i16::MAX);

            let half = ((max - min) / 2.0) + min;
            let result = quantize_to_i16(half, min, max);

            assert_eq!(result, 0);

            let low_quart = ((max - min) / 4.0) + min;
            let result = quantize_to_i16(low_quart, min, max);

            assert_eq!(result, i16::MIN / 2);

            let up_quart = max - ((max - min) / 4.0);
            let result = quantize_to_i16(up_quart, min, max);

            assert_eq!(result, i16::MAX / 2);
        }
    }
}
