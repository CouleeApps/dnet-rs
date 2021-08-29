# dnet-rs

This is a partial reimplementation of Torque3D's UDP network stack in Rust. Current features include:
- Torque-compatible BitStream implementation
- DNet-compatible raw packet sequences/acks
- Some non-raw packet types (more would be easy to add)
- Huffman string compression with lots of danger

Currently the only client application is a network fuzzer that sends 2000 random bits. The bugs are already just falling out, so I've held off on making anything more advanced yet.
