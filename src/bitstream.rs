
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
            shift: 0
        }
    }

    pub fn new() -> Self {
        BitStream {
            data: vec![0],
            position: 0,
            shift: 0
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

    fn read_bits(&mut self, bits: usize) -> u8 {
        assert!(bits <= 8);
        if bits == 0 {
            return 0;
        }

        let mut result = 0;

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
        value = (value & (0xFF >> (8 - bits)));

        //If this value is going to push us onto the next item we need to do
        // some extra fun math.
        if self.shift + bits >= 8 {

            //How many bits over 32 are we going to need?
            let extra = (self.shift + bits) % 8;
            //How many bits do we have left before 8?
            let remain = bits - extra;

            //Get the part of the value that will be pushed onto the current top,
            // should be `remain` bits long.
            let first = value & (0xFF >> (8 - remain));
            //Push it on and make sure we start at the next open bit
            self.data[self.position] |= first << self.shift;

            //Get the second part of the value that will become the next top, should
            // be `extra` bits long.
            let second = if remain == 8 {
                0
            } else if extra == 0 {
                (value >> remain)
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
            self.data[self.position] |= (value << self.shift) & (0xFF >> (8 - bits - self.shift));

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
        if self.read_flag() {
            // todo!("huffman strings");
            "".into()
        } else {
            let len = self.read_int(8);
            let mut chars = vec![];
            for i in 0..len {
                chars.push(self.read_u8());
            }
            String::from_utf8(chars).expect("String is valid utf8")
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
        self.write_flag(false); // compressed
        self.write_int(value.len() as u32, 8);
        for ch in value.as_bytes() {
            self.write_u8(*ch);
        }
        return value;
    }
}
