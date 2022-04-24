
use crate::huffman::HuffmanProcessor;
use std::f32::consts::{PI, FRAC_1_SQRT_2, SQRT_2};

const POINT_EPSILON: f32 = 0.0001f32;

#[derive(Debug)]
pub struct BitStream {
    data: Vec<u8>,
    position: usize,
    shift: usize,
}

impl BitStream {
    pub fn from_buffer(buffer: Vec<u8>) -> Self {
        BitStream {
            data: buffer,
            position: 0,
            shift: 0,
        }
    }

    pub fn new() -> Self {
        BitStream {
            data: vec![0],
            position: 0,
            shift: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn reset(&mut self) {
        self.position = 0;
        self.shift = 0;
    }

    pub fn into_bytes(mut self) -> Vec<u8> {
        if self.shift == 0 {
            self.data.pop();
        }
        self.data
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.data.as_slice()
    }

    pub fn get_bit_pos(&self) -> usize {
        self.position * 8 + self.shift
    }

    pub fn set_bit_pos(&mut self, pos: usize) {
        self.position = pos / 8;
        self.shift = pos % 8;
    }

    fn read_bits(&mut self, bits: usize) -> u8 {
        assert!(bits <= 8);
        if bits == 0 {
            return 0;
        }

        let mut result;

        //If this value is going to push us onto the next item we need to do
        // some extra fun math.
        if self.shift + bits >= 8 {
            //How many bits over 32 are we going to need?
            let extra = (self.shift + bits) % 8;
            //How many bits do we have left before 8?
            let remain = bits - extra;

            //Get the first, lower, part of the number, should be stored at the
            // end of the current top. Shift it over so it's in the correct bit
            let first = self.data[self.position] >> self.shift;
            //Add it to the result
            result = first;
            //Pop the top off because we've used all its bits
            self.position += 1;

            //If we hit 32 exactly then this will just be extra wasted time. Optimize
            // it out unless we need it.
            if extra != 0 {
                //Get the second, upper, part of the number from the new top and
                // shift it over so it lines up
                let second = (self.data[self.position] & (0xFF >> (8 - extra))) << remain;
                //Or it with the result so we get the final value
                result = result | second;
            }
            //Shift should become however many bits we read from that new top
            self.shift = extra;
        } else {
            //We're not popping anything off so we can just grab the bits from
            // the top and have a nice day.
            result = (self.data[self.position] >> self.shift) & (0xFF >> (8 - bits));

            //Just add to the shift
            self.shift += bits;
        }
        return result;
    }

    fn write_bits(&mut self, mut value: u8, bits: usize) -> u8 {
        assert!(bits <= 8);
        if bits == 0 {
            return 0;
        }

        //Sanitize value, don't let it be longer than the number of bits we're promised
        value = value & (0xFF >> (8 - bits));

        //If this value is going to push us onto the next item we need to do
        // some extra fun math.
        if self.shift + bits >= 8 {

            //How many bits over 8 are we going to need?
            let extra = (self.shift + bits) % 8;
            //How many bits do we have left before 8?
            let remain = bits - extra;

            //Get the part of the value that will be pushed onto the current top,
            // should be `remain` bits long.
            let first = value & (0xFF >> (8 - remain));
            let lower = if self.shift == 0 {
                0
            } else {
                self.data[self.position] & (0xFF >> (8 - self.shift))
            };
            //Push it on and make sure we start at the next open bit
            self.data[self.position] = lower | (first << self.shift);

            //Get the second part of the value that will become the next top, should
            // be `extra` bits long.
            let second = if remain == 8 {
                0
            } else if extra == 0 {
                value >> remain
            } else {
                (value >> remain) & (0xFF >> (8 - extra))
            };
            //Start a new top with it
            self.position += 1;
            self.data.push(0);
            self.data[self.position] = second;

            //Shift should become however many bits long that new top is
            self.shift = extra;
        } else {
            //We don't have to create a new top, we can just slap this one on the
            // end of the original one. OR the bits on, make sure to push them over
            // so they line up, and cut off anything at the end
            let lower = if self.shift == 0 {
                0
            } else {
                self.data[self.position] & (0xFF >> (8 - self.shift))
            };
            self.data[self.position] = lower | ((value << self.shift) & (0xFF >> (8 - bits - self.shift)));

            //Just add to the shift
            self.shift += bits;
        }
        return value;
    }

    pub fn read_int(&mut self, mut bits: usize) -> u32 {
        let mut value = 0;
        let mut shift = 0;
        loop {
            value |= (self.read_bits(bits.min(8)) as u32) << shift;
            shift += 8;
            if bits <= 8 {
                break;
            }
            bits -= 8;
        }
        return value;
    }

    pub fn write_int(&mut self, mut value: u32, mut bits: usize) -> u32 {
        let original = value;
        loop {
            self.write_bits((value & 0xFF) as u8, bits.min(8));
            value >>= 8;
            if bits <= 8 {
                break;
            }
            bits -= 8;
        }
        return original;
    }

    pub fn read_flag(&mut self) -> bool {
        return self.read_int(1) == 1;
    }

    pub fn read_u8(&mut self) -> u8 {
        return self.read_int(8) as u8;
    }

    pub fn read_u16(&mut self) -> u16 {
        return self.read_int(16) as u16;
    }

    pub fn read_u32(&mut self) -> u32 {
        return self.read_int(32) as u32;
    }

    pub fn read_string(&mut self) -> String {
        HuffmanProcessor::read_string(self)
    }

    pub fn read_cstring(&mut self) -> String {
        let len = self.read_u8();
        let mut chars = vec![];
        for _ in 0..len {
            chars.push(self.read_u8());
        }
        return chars.into_iter().map(|c| c as char).collect();
    }

    pub fn read_long_cstring(&mut self) -> String {
        let len = self.read_u16();
        let mut chars = vec![];
        for _ in 0..len {
            chars.push(self.read_u8());
        }
        return chars.into_iter().map(|c| c as char).collect();
    }

    pub fn read_float_zero_to_one(&mut self, bit_count: usize) -> f32 {
        let max_int = (1u32 << bit_count) - 1;
        let i = self.read_int(bit_count);
        if i == 0 {
            return 0f32;
        }
        if i == (max_int / 2) + 1 {
            return 0.5f32;
        }
        if i == max_int {
            return 1.0f32;
        }
        return (i as f32) / (max_int as f32);
    }

    pub fn read_signed_float_neg_one_to_one(&mut self, bit_count: usize) -> f32 {
        return self.read_float_zero_to_one(bit_count) * 2f32 - 1f32;
    }

    pub fn read_signed_int(&mut self, bit_count: usize) -> i32 {
        // 1s complement because torque is torque
        if self.read_flag() {
            return -(self.read_int(bit_count - 1) as i32);
        } else {
            return self.read_int(bit_count - 1) as i32;
        }
    }

    pub fn read_normal_vector(&mut self, bit_count: usize) -> (f32, f32, f32) {
        let phi = self.read_signed_float_neg_one_to_one(bit_count + 1) * PI;
        let theta = self.read_signed_float_neg_one_to_one(bit_count) * (PI / 2.0);

        (
            phi.sin() * theta.cos(),
            phi.cos() * theta.cos(),
            theta.sin()
        )
    }

    pub fn read_vector(&mut self, max_magnitude: f32, magnitude_bits: usize, normal_bits: usize) -> (f32, f32, f32) {
        if !self.read_flag() {
            return (0.0, 0.0, 0.0);
        }

        let mag;
        if self.read_flag() {
            mag = self.read_float_zero_to_one(magnitude_bits) * max_magnitude;
        } else {
            mag = f32::from_bits(self.read_int(32));
        }

        let normal = self.read_normal_vector(normal_bits);
        (
            normal.0 * mag,
            normal.1 * mag,
            normal.2 * mag
        )
    }

    pub fn read_quat(&mut self, bit_count: usize) -> (f32, f32, f32, f32) {
        let mut vals = [0f32; 4];
        let mut sum = 0f32;

        let idx_max = self.read_int(2) as usize;
        for i in 0..4 {
            if i == idx_max {
                continue;
            }
            vals[i] = self.read_signed_float_neg_one_to_one(bit_count) * FRAC_1_SQRT_2;
            sum += vals[i] * vals[i];
        }

        if sum > 1.0 {
            vals[idx_max] = 1.0;
        } else {
            vals[idx_max] = (1.0 - sum).sqrt();
        }

        return (vals[0], vals[1], vals[2], vals[3]);
    }

    pub fn read_ranged_u32(&mut self, range_start: u32, range_end: u32) -> u32 {
        let range_size = range_end - range_start + 1;
        let range_bits = range_size.next_power_of_two().trailing_zeros();

        let val = self.read_int(range_bits as usize);
        return val + range_start;
    }

    pub fn read_cussed_u32(&mut self) -> u32 {
        if self.read_flag() {
            return 0;
        } else if self.read_flag() {
            return self.read_ranged_u32(0, 0xF);
        } else if self.read_flag() {
            return self.read_ranged_u32(0, 0xFF);
        } else if self.read_flag() {
            return self.read_ranged_u32(0, 0xFFFF);
        } else if self.read_flag() {
            return self.read_ranged_u32(0, 0xFFFFFF);
        } else {
            return self.read_ranged_u32(0, 0xFFFFFFFF);
        }
    }

    pub fn write_flag(&mut self, value: bool) -> bool {
        self.write_int(value as u32, 1);
        return value;
    }

    pub fn write_u8(&mut self, value: u8) -> u8 {
        self.write_int(value as u32, 8);
        return value;
    }

    pub fn write_u16(&mut self, value: u16) -> u16 {
        self.write_int(value as u32, 16);
        return value;
    }

    pub fn write_u32(&mut self, value: u32) -> u32 {
        self.write_int(value as u32, 32);
        return value;
    }

    pub fn write_string(&mut self, value: String) -> String {
        HuffmanProcessor::write_string(self, &value);
        return value;
    }

    pub fn write_cstring(&mut self, value: String) -> String {
        assert!(value.len() < 256);
        self.write_u8(value.len() as u8);
        for ch in value.chars() {
            self.write_u8(ch as u8);
        }
        return value;
    }

    pub fn write_long_cstring(&mut self, value: String) -> String {
        assert!(value.len() < 65536);
        self.write_u16(value.len() as u16);
        for ch in value.chars() {
            self.write_u8(ch as u8);
        }
        return value;
    }

    pub fn write_float_zero_to_one(&mut self, mut value: f32, bit_count: usize) -> f32 {
        let max_int = (1u32 << bit_count) - 1;
        let i;
        if value < POINT_EPSILON {
            i = 0;
            value = 0.0;
        } else if (value - 0.5).abs() < POINT_EPSILON {
            i = (max_int / 2) + 1;
            value = 0.5;
        } else if value > (1.0f32 - POINT_EPSILON) {
            i = max_int;
            value = 1.0;
        } else {
            i = (value * (max_int as f32)).round() as u32;
            value = (i as f32) / (max_int as f32);
        }

        self.write_int(i, bit_count);

        return value;
    }

    pub fn write_signed_float_neg_one_to_one(&mut self, value: f32, bit_count: usize) -> f32 {
        return self.write_float_zero_to_one((value + 1f32) / 2f32, bit_count) * 2f32 - 1f32;
    }

    pub fn write_signed_int(&mut self, value: i32, bit_count: usize) -> i32 {
        // I will become back my money
        if value < 0 {
            self.write_flag(true);
            self.write_int((-value) as u32, bit_count);
        } else {
            self.write_flag(false);
            self.write_int(value as u32, bit_count);
        }

        return value;
    }

    pub fn write_normal_vector(&mut self, value: (f32, f32, f32), bit_count: usize) -> (f32, f32, f32) {
        let phi = value.0.atan2(value.1) / PI;
        let theta = value.2.atan2((value.0 * value.0 + value.1 * value.1).sqrt()) / (PI / 2.0);

        self.write_signed_float_neg_one_to_one(phi, bit_count + 1);
        self.write_signed_float_neg_one_to_one(theta, bit_count);

        (
            phi.sin() * theta.cos(),
            phi.cos() * theta.cos(),
            theta.sin()
        )
    }

    pub fn write_vector(&mut self, value: (f32, f32, f32), max_magnitude: f32, magnitude_bits: usize, normal_bits: usize) -> (f32, f32, f32) {
        let mag = (value.0 * value.0 + value.1 * value.1 + value.2 * value.2).sqrt();
        if mag < POINT_EPSILON {
            self.write_flag(false);
            return (0.0, 0.0, 0.0);
        }

        if mag < max_magnitude {
            self.write_flag(true);
            self.write_float_zero_to_one(mag / max_magnitude, magnitude_bits);
        } else {
            self.write_int(mag.to_bits(), 32);
        }

        let div = 1.0 / mag;

        self.write_normal_vector((
            value.0 * div,
            value.1 * div,
            value.2 * div,
        ), normal_bits);

        return (
            (value.0 * div) * mag,
            (value.1 * div) * mag,
            (value.2 * div) * mag,
        );
    }

    pub fn write_quat(&mut self, value: (f32, f32, f32, f32), bit_count: usize) -> (f32, f32, f32, f32) {
        let vals = [value.0, value.1, value.2, value.3];
        let mut flip = vals[0] < 0.0;
        let mut max_val = vals[0].abs();
        let mut idx_max = 0;

        for i in 1..4 {
            if vals[i].abs() > max_val {
                idx_max = i;
                max_val = vals[i].abs();
                flip = vals[i] < 0.0;
            }
        }

        self.write_int(idx_max as u32, 2);

        for i in 0..4 {
            if i == idx_max {
                continue;
            }
            let cur_value = if flip {
                -vals[i]
            } else {
                vals[i]
            } * SQRT_2;
            self.write_signed_float_neg_one_to_one(cur_value, bit_count);
        }

        value
    }

    pub fn write_ranged_u32(&mut self, value: u32, range_start: u32, range_end: u32) -> u32 {
        let range_size = range_end - range_start + 1;
        let range_bits = range_size.next_power_of_two().trailing_zeros();

        self.write_int(value - range_start, range_bits as usize);
        return value;
    }

    pub fn write_cussed_u32(&mut self, value: u32) -> u32 {
        if self.write_flag(value == 0) {
            return 0;
        } else if self.write_flag(value <= 0xF) {
            return self.write_ranged_u32(value, 0, 0xF);
        } else if self.write_flag(value <= 0xFF) {
            return self.write_ranged_u32(value, 0, 0xFF);
        } else if self.write_flag(value <= 0xFFFF) {
            return self.write_ranged_u32(value, 0, 0xFFFF);
        } else if self.write_flag(value <= 0xFFFFFF) {
            return self.write_ranged_u32(value, 0, 0xFFFFFF);
        } else {
            return self.write_ranged_u32(value, 0, 0xFFFFFFFF);
        }
    }
}
