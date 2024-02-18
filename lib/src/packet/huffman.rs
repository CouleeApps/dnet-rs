#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use super::bitstream::BitStream;
use anyhow::Result;
use std::ptr::null_mut;

// DANGER: Chock full of danger
static mut g_huffProcessor: HuffmanProcessor = HuffmanProcessor::new();

// This giant array of u32s is your last chance to not see the danger
const csm_charFreqs: [u32; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 329, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 2809, 68, 0, 27, 0, 58, 3, 62, 4, 7, 0, 0, 15, 65, 554, 3, 394, 404, 189, 117, 30, 51, 27,
    15, 34, 32, 80, 1, 142, 3, 142, 39, 0, 144, 125, 44, 122, 275, 70, 135, 61, 127, 8, 12, 113,
    246, 122, 36, 185, 1, 149, 309, 335, 12, 11, 14, 54, 151, 0, 0, 2, 0, 0, 211, 0, 2090, 344,
    736, 993, 2872, 701, 605, 646, 1552, 328, 305, 1240, 735, 1533, 1713, 562, 3, 1775, 1149, 1469,
    979, 407, 553, 59, 279, 31, 0, 0, 0, 68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

// Ok, I warned you

#[derive(Clone, Debug)]
struct HuffNode {
    pop: u32,
    index0: i16,
    index1: i16,
}

#[derive(Clone, Debug)]
struct HuffLeaf {
    pop: u32,
    numBits: u8,
    symbol: u8,
    code: u32,
}

#[derive(Clone, Debug)]
struct HuffWrap {
    // This should be red flag #2
    pNode: *mut HuffNode,
    pLeaf: *mut HuffLeaf,
}

#[derive(Clone, Debug)]
pub struct HuffmanProcessor {
    m_tablesBuilt: bool,
    m_huffNodes: Vec<HuffNode>,
    m_huffLeaves: Vec<HuffLeaf>,
}

impl HuffWrap {
    pub fn new() -> Self {
        HuffWrap {
            pNode: null_mut(),
            pLeaf: null_mut(),
        }
    }

    pub fn set_leaf(&mut self, in_leaf: *mut HuffLeaf) {
        self.pNode = null_mut();
        self.pLeaf = in_leaf;
    }

    pub fn set_node(&mut self, in_node: *mut HuffNode) {
        self.pNode = in_node;
        self.pLeaf = null_mut();
    }

    pub fn getPop(&self) -> u32 {
        // SAFETY: Not safe
        unsafe {
            if !self.pNode.is_null() {
                (*self.pNode).pop
            } else if !self.pLeaf.is_null() {
                (*self.pLeaf).pop
            } else {
                panic!("Null");
            }
        }
    }
}

// XXX: Cursed. It's all cursed. Oh God
impl HuffmanProcessor {
    const fn new() -> Self {
        HuffmanProcessor {
            m_tablesBuilt: false,
            m_huffNodes: vec![],
            m_huffLeaves: vec![],
        }
    }

    fn buildTables(&mut self) {
        assert!(!self.m_tablesBuilt);
        self.m_tablesBuilt = true;

        self.m_huffLeaves.resize_with(256, || HuffLeaf {
            pop: 0,
            numBits: 0,
            symbol: 0,
            code: 0,
        });
        self.m_huffNodes.reserve(256);
        self.m_huffNodes.push(HuffNode {
            pop: 0,
            index0: 0,
            index1: 0,
        });

        for i in 0..256 {
            self.m_huffLeaves[i].pop = csm_charFreqs[i] + 1;
            self.m_huffLeaves[i].symbol = i as u8;
            self.m_huffLeaves[i].code = 0;
            self.m_huffLeaves[i].numBits = 0;
        }

        let mut currWraps = 256;
        let mut pWrap = (0..256)
            .into_iter()
            .map(|_| HuffWrap::new())
            .collect::<Vec<_>>();
        for i in 0..256 {
            pWrap[i].set_leaf(&mut self.m_huffLeaves[i]);
        }

        while currWraps != 1 {
            let mut min1 = 0xfffffffeu32;
            let mut min2 = 0xffffffffu32;

            let mut index1 = -1i32;
            let mut index2 = -1i32;

            for i in 0..currWraps {
                if pWrap[i].getPop() < min1 {
                    min2 = min1;
                    index2 = index1;

                    min1 = pWrap[i].getPop();
                    index1 = i as _;
                } else if pWrap[i].getPop() < min2 {
                    min2 = pWrap[i].getPop();
                    index2 = i as _;
                }
            }

            assert!(index1 != -1);
            assert!(index2 != -1);
            assert!(index1 != index2);

            let det_index0 = self.determineIndex(&pWrap[index1 as usize]);
            let det_index1 = self.determineIndex(&pWrap[index2 as usize]);

            self.m_huffNodes.push(HuffNode {
                pop: pWrap[index1 as usize].getPop() + pWrap[index2 as usize].getPop(),
                index0: det_index0,
                index1: det_index1,
            });

            let mergeIndex = if index1 > index2 { index2 } else { index1 };
            let nukeIndex = if index1 > index2 { index1 } else { index2 };

            pWrap[mergeIndex as usize].set_node(self.m_huffNodes.last_mut().unwrap());

            if index2 != (currWraps as i32 - 1) {
                pWrap.swap(nukeIndex as usize, currWraps - 1);
            }
            currWraps -= 1;
        }
        assert!(currWraps == 1);
        assert!(!pWrap[0].pNode.is_null());
        assert!(pWrap[0].pLeaf.is_null());

        // SAFETY: Likely safe
        self.m_huffNodes[0] = unsafe { (*pWrap[0].pNode).clone() };

        let mut bs = BitStream::from_buffer(vec![0u8; 4]);
        self.generateCodes(&mut bs, 0, 0);
    }

    fn determineIndex(&mut self, wrap: &HuffWrap) -> i16 {
        if !wrap.pLeaf.is_null() {
            assert!(wrap.pNode.is_null());

            // SAFETY: DEFINITELY not safe
            (unsafe { -((wrap.pLeaf.offset_from(self.m_huffLeaves.as_ptr())) + 1) }) as i16
        } else {
            assert!(!wrap.pNode.is_null());

            // SAFETY: Extremely likely not safe
            (unsafe { wrap.pNode.offset_from(self.m_huffNodes.as_ptr()) }) as i16
        }
    }

    fn generateCodes(&mut self, stream: &mut BitStream, index: i32, depth: i32) {
        if index < 0 {
            stream.set_bit_pos(0);
            // Real torque doesn't limit `code` to `numBits` but this is fine for us
            self.m_huffLeaves[-(index + 1) as usize].code = stream
                .read_int(depth as usize)
                .expect("probably enough bytes in here");
            self.m_huffLeaves[-(index + 1) as usize].numBits = depth as u8;
        } else {
            let index0 = self.m_huffNodes[index as usize].index0;
            let index1 = self.m_huffNodes[index as usize].index1;

            let pos = stream.get_bit_pos();

            stream.write_flag(false);
            self.generateCodes(stream, index0 as i32, depth + 1);

            stream.set_bit_pos(pos);
            stream.write_flag(true);
            self.generateCodes(stream, index1 as i32, depth + 1);

            stream.set_bit_pos(pos);
        }
    }

    fn readHuffBuffer(&mut self, stream: &mut BitStream, buffer: &mut [u8]) -> Result<u32> {
        if !self.m_tablesBuilt {
            self.buildTables();
        }

        if stream.read_flag()? {
            let mut len = stream.read_int(8)?;
            if len >= buffer.len() as u32 {
                len = buffer.len() as u32;
            }

            for i in 0..len {
                let mut index = 0i16;
                // TERMINATION: Lol maybe
                loop {
                    if index >= 0 {
                        if stream.read_flag()? {
                            index = self.m_huffNodes[index as usize].index1;
                        } else {
                            index = self.m_huffNodes[index as usize].index0;
                        }
                    } else {
                        buffer[i as usize] = self.m_huffLeaves[-(index + 1) as usize].symbol;
                        break;
                    }
                }
            }
            return Ok(len);
        } else {
            let mut len = stream.read_int(8)?;
            if len >= buffer.len() as u32 {
                len = buffer.len() as u32;
            }

            for i in 0..len {
                buffer[i as usize] = stream.read_u8()?;
            }
            return Ok(len);
        }
    }

    fn writeHuffBuffer(
        &mut self,
        stream: &mut BitStream,
        buffer: Option<&[u8]>,
        maxLen: u32,
    ) -> u32 {
        if buffer.is_none() {
            stream.write_flag(false);
            stream.write_int(0, 8);
            return 0;
        }

        if !self.m_tablesBuilt {
            self.buildTables();
        }

        let buffer = buffer.unwrap();
        let mut len = buffer.len() as u32;
        assert!(len <= 255);
        if len > maxLen {
            len = maxLen;
        }

        let mut numBits = 0u32;
        for i in 0..len {
            numBits += self.m_huffLeaves[buffer[i as usize] as usize].numBits as u32;
        }

        if numBits >= (len * 8) {
            stream.write_flag(false);
            stream.write_int(len as u32, 8);

            for i in 0..len {
                stream.write_u8(buffer[i as usize]);
            }
        } else {
            stream.write_flag(true);
            stream.write_int(len as u32, 8);
            for i in 0..len {
                // Avert your eyes
                stream.write_int(
                    self.m_huffLeaves[buffer[i as usize] as usize].code,
                    self.m_huffLeaves[buffer[i as usize] as usize].numBits as usize,
                );
            }
        }

        return len;
    }

    // Functions provided to nicely hide all the danger from you

    pub fn read_buffer(stream: &mut BitStream, buffer: &mut [u8]) -> Result<usize> {
        // SAFETY: Dangerous
        return Ok(unsafe { g_huffProcessor.readHuffBuffer(stream, buffer)? as usize });
    }

    pub fn write_buffer(stream: &mut BitStream, buffer: Option<&[u8]>, maxLen: usize) -> usize {
        // SAFETY: Dangerous
        return unsafe { g_huffProcessor.writeHuffBuffer(stream, buffer, maxLen as u32) as usize };
    }

    pub fn read_string(stream: &mut BitStream) -> Result<String> {
        let mut buffer = [0u8; 256];
        let length = Self::read_buffer(stream, &mut buffer)?;

        return Ok(buffer[0..length].iter().map(|&c| c as char).collect());
    }

    pub fn write_string(stream: &mut BitStream, value: &String) -> usize {
        Self::write_buffer(
            stream,
            Some(
                value
                    .chars()
                    .into_iter()
                    .map(|c| c as u8)
                    .collect::<Vec<u8>>()
                    .as_slice(),
            ),
            256,
        )
    }
}
