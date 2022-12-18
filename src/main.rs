#![feature(test)]

extern crate test;

use serde_derive::Serialize;
use serde_derive::Deserialize;

pub const PACKET_DATA_SIZE: usize = 1280 - 40 - 8;

#[bench]
fn bench_serialize_vanilla(bencher: &mut test::Bencher) {
    #[derive(Default, Clone, Serialize)]
    pub struct Packet {
        buffer: [u8; 32],
        flags: u64,
    }

    let mut output_binary = vec![];
    let mut input_packets: Vec<_> = std::iter::repeat(Packet::default()).take(32).collect();

    bencher.iter(|| {
        test::black_box(bincode::serialize_into(&mut output_binary, &input_packets).unwrap());
        output_binary.clear();
    })
}

#[bench]
fn bench_serialize_serde_as(bencher: &mut test::Bencher) {
    use serde_with::serde_as;

    #[serde_as]
    #[derive(Default, Clone, Serialize)]
    pub struct Packet {
        #[serde_as(as = "[_; 32]")]
        buffer: [u8; 32],
        flags: u64,
    }

    let mut output_binary = vec![];
    let mut input_packets: Vec<_> = std::iter::repeat(Packet::default()).take(32).collect();

    bencher.iter(|| {
        test::black_box(bincode::serialize_into(&mut output_binary, &input_packets).unwrap());
        output_binary.clear();
    })
}

#[bench]
fn bench_serialize_serde_arrays(bencher: &mut test::Bencher) {
    #[derive(Default, Clone, Serialize)]
    pub struct Packet {
        #[serde(with = "serde_arrays")]
        buffer: [u8; 32],
        flags: u64,
    }

    let mut output_binary = vec![];
    let mut input_packets: Vec<_> = std::iter::repeat(Packet::default()).take(32).collect();

    bencher.iter(|| {
        test::black_box(bincode::serialize_into(&mut output_binary, &input_packets).unwrap());
        output_binary.clear();
    })
}

fn bench_serialize_serde_bytes(bencher: &mut test::Bencher) {
    mod serde_bytes_array {
        use {
            core::convert::TryInto,
            serde::{de::Error, Deserializer, Serializer},
        };

        pub(crate) fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serde_bytes::serialize(bytes, serializer)
        }

        pub(crate) fn deserialize<'de, D, const N: usize>(deserializer: D) -> Result<[u8; N], D::Error>
        where
            D: Deserializer<'de>,
        {
            let slice: &[u8] = serde_bytes::deserialize(deserializer)?;
            let array: [u8; N] = slice.try_into().map_err(|_| {
                let expected = format!("[u8; {}]", N);
                D::Error::invalid_length(slice.len(), &expected.as_str())
            })?;
            Ok(array)
        }
    }

    #[derive(Default, Clone, Serialize)]
    pub struct Packet {
        #[serde(with = "serde_bytes_array")]
        buffer: [u8; 32],
        flags: u64,
    }

    let mut output_binary = vec![];
    let mut input_packets: Vec<_> = std::iter::repeat(Packet::default()).take(32).collect();

    bencher.iter(|| {
        test::black_box(bincode::serialize_into(&mut output_binary, &input_packets).unwrap());
        output_binary.clear();
    })
}

#[bench]
fn bench_serialize_serde_arrays_normal(bencher: &mut test::Bencher) {
    #[derive(Clone, Serialize)]
    pub struct Packet {
        #[serde(with = "serde_arrays")]
        buffer: [u8; PACKET_DATA_SIZE],
        flags: u64,
    }
    impl Default for Packet {
        fn default() -> Self {
            Self {
                buffer: [0; PACKET_DATA_SIZE],
                flags: 3,
            }
        }
    }

    let mut output_binary = vec![];
    let mut input_packets: Vec<_> = std::iter::repeat(Packet::default()).take(512).collect();

    bencher.iter(|| {
        test::black_box(bincode::serialize_into(&mut output_binary, &input_packets).unwrap());
        output_binary.clear();
    })
}

#[bench]
fn bench_serialize_serde_as_normal(bencher: &mut test::Bencher) {
    use serde_with::serde_as;

    #[serde_as]
    #[derive(Clone, Serialize)]
    pub struct Packet {
        #[serde_as(as = "[_; PACKET_DATA_SIZE]")]
        buffer: [u8; PACKET_DATA_SIZE],
        flags: u64,
    }
    impl Default for Packet {
        fn default() -> Self {
            Self {
                buffer: [0; PACKET_DATA_SIZE],
                flags: 3,
            }
        }
    }

    let mut output_binary = vec![];
    let mut input_packets: Vec<_> = std::iter::repeat(Packet::default()).take(512).collect();

    bencher.iter(|| {
        test::black_box(bincode::serialize_into(&mut output_binary, &input_packets).unwrap());
        output_binary.clear();
    })
}

mod serde_bytes_vec {
    use super::*;

    mod serde_bytes_array {
        use {
            core::convert::TryInto,
            serde::{de::Error, Deserializer, Serializer},
        };

        #[inline(always)]
        pub(crate) fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serde_bytes::serialize(bytes, serializer)
        }

        #[inline(always)]
        pub(crate) fn deserialize<'de, D, const N: usize>(deserializer: D) -> Result<[u8; N], D::Error>
        where
            D: Deserializer<'de>,
        {
            let vec: Vec<u8> = serde_bytes::deserialize(deserializer)?;
            let vec_len = vec.len();
            let array: [u8; N] = vec.try_into().map_err(|_| {
                let expected = format!("[u8; {}]", N);
                D::Error::invalid_length(vec_len, &expected.as_str())
            })?;
            Ok(array)
        }
    }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct Packet {
        #[serde(with = "serde_bytes_array")]
        buffer: [u8; PACKET_DATA_SIZE],
        flags: u64,
    }
    impl Default for Packet {
        #[inline(always)]
        fn default() -> Self {
            Self {
                buffer: [0; PACKET_DATA_SIZE],
                flags: 3,
            }
        }
    }

    #[bench]
    fn bench_serialize_serde_bytes_normal(bencher: &mut test::Bencher) {
        let mut output_binary = vec![];
        let mut input_packets: Vec<_> = std::iter::repeat(Packet::default()).take(512).collect();

        bencher.iter(|| {
            test::black_box(bincode::serialize_into(&mut output_binary, &input_packets).unwrap());
            output_binary.clear();
        })
    }

    #[bench]
    fn bench_serialize_serde_bytes_normal_debug_writes(bencher: &mut test::Bencher) {
        use std::io::Write;
        struct DummyWriter;
        impl Write for DummyWriter {
            fn write(&mut self, s: &[u8]) -> Result<usize, std::io::Error> { 
                dbg!("write: {} bytes", s.len());
                Ok(s.len())
            }
            fn flush(&mut self) -> Result<(), std::io::Error> {
                Ok(())
            }
        }
        let mut input_packets: Vec<_> = std::iter::repeat(Packet::default()).take(512).collect();

        bencher.iter(|| {
            test::black_box(bincode::serialize_into(&mut DummyWriter, &input_packets).unwrap());
        })
    }

    #[bench]
    fn bench_deserialize_serde_bytes_normal(bencher: &mut test::Bencher) {
        use std::io::Write;
        let mut input_packets: Vec<_> = std::iter::repeat(Packet::default()).take(512).collect();
        let mut stream = std::io::BufWriter::new(std::fs::OpenOptions::new().append(true).create(true).open("./out").unwrap());
        test::black_box(bincode::serialize_into(&mut stream, &input_packets).unwrap());
        stream.flush();

        let mut stream = std::io::BufReader::new(std::fs::File::open("./out").unwrap());
        let mut s: Vec<u8> = vec![];
        use std::io::Read;
        stream.read_to_end(&mut s).unwrap();

        bencher.iter(|| {
            test::black_box(bincode::deserialize::<Vec<Packet>>(&s).unwrap());
        })
    }

    #[bench]
    fn bench_deserialize_from_serde_bytes_normal(bencher: &mut test::Bencher) {
        use std::io::Write;
        let mut input_packets: Vec<_> = std::iter::repeat(Packet::default()).take(512).collect();
        let mut stream = std::io::BufWriter::new(std::fs::OpenOptions::new().append(true).create(true).open("./out").unwrap());
        test::black_box(bincode::serialize_into(&mut stream, &input_packets).unwrap());
        stream.flush();

        let mut stream = std::io::BufReader::new(std::fs::File::open("./out").unwrap());
        let mut s: Vec<u8> = vec![];
        use std::io::Read;
        stream.read_to_end(&mut s).unwrap();

        bencher.iter(|| {
            let mut reader = &s[..];
            bincode::deserialize_from::<_, Vec<Packet>>(&mut reader).unwrap();
        })
    }
}

mod serde_bytes_cow {
    use super::*;

    mod serde_bytes_array {
        use {
            core::convert::TryInto,
            serde::{de::Error, Deserializer, Serializer},
        };

        #[inline(always)]
        pub(crate) fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serde_bytes::serialize(bytes, serializer)
        }

        #[inline(always)]
        pub(crate) fn deserialize<'de, D, const N: usize>(deserializer: D) -> Result<[u8; N], D::Error>
        where
            D: Deserializer<'de>,
        {
            let slice: std::borrow::Cow<'de, [u8]> = serde_bytes::deserialize(deserializer)?;
            //dbg!(&slice);
            let array: [u8; N] = (&*slice).try_into().map_err(|_| {
                let expected = format!("[u8; {}]", N);
                D::Error::invalid_length(slice.len(), &expected.as_str())
            })?;
            Ok(array)
        }
    }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct Packet {
        #[serde(with = "serde_bytes_array")]
        buffer: [u8; PACKET_DATA_SIZE],
        flags: u64,
    }
    impl Default for Packet {
        #[inline(always)]
        fn default() -> Self {
            Self {
                buffer: [0; PACKET_DATA_SIZE],
                flags: 3,
            }
        }
    }

    #[bench]
    fn bench_serialize_serde_bytes_normal(bencher: &mut test::Bencher) {
        let mut output_binary = vec![];
        let mut input_packets: Vec<_> = std::iter::repeat(Packet::default()).take(512).collect();

        bencher.iter(|| {
            test::black_box(bincode::serialize_into(&mut output_binary, &input_packets).unwrap());
            output_binary.clear();
        })
    }

    #[bench]
    fn bench_serialize_serde_bytes_normal_debug_writes(bencher: &mut test::Bencher) {
        use std::io::Write;
        struct DummyWriter;
        impl Write for DummyWriter {
            fn write(&mut self, s: &[u8]) -> Result<usize, std::io::Error> { 
                dbg!("write: {} bytes", s.len());
                Ok(s.len())
            }
            fn flush(&mut self) -> Result<(), std::io::Error> {
                Ok(())
            }
        }
        let mut input_packets: Vec<_> = std::iter::repeat(Packet::default()).take(512).collect();

        bencher.iter(|| {
            test::black_box(bincode::serialize_into(&mut DummyWriter, &input_packets).unwrap());
        })
    }

    #[bench]
    fn bench_deserialize_serde_bytes_normal(bencher: &mut test::Bencher) {
        use std::io::Write;
        let mut input_packets: Vec<_> = std::iter::repeat(Packet::default()).take(512).collect();
        let mut stream = std::io::BufWriter::new(std::fs::OpenOptions::new().append(true).create(true).open("./out").unwrap());
        test::black_box(bincode::serialize_into(&mut stream, &input_packets).unwrap());
        stream.flush();

        let mut stream = std::io::BufReader::new(std::fs::File::open("./out").unwrap());
        let mut s: Vec<u8> = vec![];
        use std::io::Read;
        stream.read_to_end(&mut s).unwrap();

        bencher.iter(|| {
            //let mut reader = &s[..];
            //bincode::deserialize_from::<_, Vec<Packet>>(&mut stream).unwrap();
            //test::black_box(bincode::deserialize_from::<_, Vec<Packet>>(&mut stream).unwrap());
            //bincode::deserialize_from::<_, Vec<Packet>>(&mut stream).unwrap();
            test::black_box(bincode::deserialize::<Vec<Packet>>(&s).unwrap());
            //bincode::deserialize::<Vec<Packet>>(&s).unwrap();
        })
    }

    #[bench]
    fn bench_deserialize_from_serde_bytes_normal(bencher: &mut test::Bencher) {
        use std::io::Write;
        let mut input_packets: Vec<_> = std::iter::repeat(Packet::default()).take(512).collect();
        let mut stream = std::io::BufWriter::new(std::fs::OpenOptions::new().append(true).create(true).open("./out").unwrap());
        test::black_box(bincode::serialize_into(&mut stream, &input_packets).unwrap());
        stream.flush();

        let mut stream = std::io::BufReader::new(std::fs::File::open("./out").unwrap());
        let mut s: Vec<u8> = vec![];
        use std::io::Read;
        stream.read_to_end(&mut s).unwrap();

        bencher.iter(|| {
            let mut reader = &s[..];
            bincode::deserialize_from::<_, Vec<Packet>>(&mut reader).unwrap();
        })
    }
}

mod serde_bytes_slice {
    use super::*;

    mod serde_bytes_array {
        use {
            core::convert::TryInto,
            serde::{de::Error, Deserializer, Serializer},
        };

        #[inline(always)]
        pub(crate) fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serde_bytes::serialize(bytes, serializer)
        }

        #[inline(always)]
        pub(crate) fn deserialize<'de, D, const N: usize>(deserializer: D) -> Result<[u8; N], D::Error>
        where
            D: Deserializer<'de>,
        {
            let slice: &[u8] = serde_bytes::deserialize(deserializer)?;
            //dbg!(&slice);
            let array: [u8; N] = slice.try_into().map_err(|_| {
                let expected = format!("[u8; {}]", N);
                D::Error::invalid_length(slice.len(), &expected.as_str())
            })?;
            Ok(array)
        }
    }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct Packet {
        #[serde(with = "serde_bytes_array")]
        buffer: [u8; PACKET_DATA_SIZE],
        flags: u64,
    }
    impl Default for Packet {
        #[inline(always)]
        fn default() -> Self {
            Self {
                buffer: [0; PACKET_DATA_SIZE],
                flags: 3,
            }
        }
    }

    #[bench]
    fn bench_serialize_serde_bytes_normal(bencher: &mut test::Bencher) {
        let mut output_binary = vec![];
        let mut input_packets: Vec<_> = std::iter::repeat(Packet::default()).take(512).collect();

        bencher.iter(|| {
            test::black_box(bincode::serialize_into(&mut output_binary, &input_packets).unwrap());
            output_binary.clear();
        })
    }

    #[bench]
    fn bench_serialize_serde_bytes_normal_debug_writes(bencher: &mut test::Bencher) {
        use std::io::Write;
        struct DummyWriter;
        impl Write for DummyWriter {
            fn write(&mut self, s: &[u8]) -> Result<usize, std::io::Error> { 
                dbg!("write: {} bytes", s.len());
                Ok(s.len())
            }
            fn flush(&mut self) -> Result<(), std::io::Error> {
                Ok(())
            }
        }
        let mut input_packets: Vec<_> = std::iter::repeat(Packet::default()).take(512).collect();

        bencher.iter(|| {
            test::black_box(bincode::serialize_into(&mut DummyWriter, &input_packets).unwrap());
        })
    }

    #[bench]
    fn bench_deserialize_serde_bytes_normal(bencher: &mut test::Bencher) {
        use std::io::Write;
        let mut input_packets: Vec<_> = std::iter::repeat(Packet::default()).take(512).collect();
        let mut stream = std::io::BufWriter::new(std::fs::OpenOptions::new().append(true).create(true).open("./out").unwrap());
        test::black_box(bincode::serialize_into(&mut stream, &input_packets).unwrap());
        stream.flush();

        let mut stream = std::io::BufReader::new(std::fs::File::open("./out").unwrap());
        let mut s: Vec<u8> = vec![];
        use std::io::Read;
        stream.read_to_end(&mut s).unwrap();

        bencher.iter(|| {
            test::black_box(bincode::deserialize::<Vec<Packet>>(&s).unwrap());
        })
    }

    #[bench]
    fn bench_deserialize_from_serde_bytes_normal(bencher: &mut test::Bencher) {
        use std::io::Write;
        let mut input_packets: Vec<_> = std::iter::repeat(Packet::default()).take(512).collect();
        let mut stream = std::io::BufWriter::new(std::fs::OpenOptions::new().append(true).create(true).open("./out").unwrap());
        test::black_box(bincode::serialize_into(&mut stream, &input_packets).unwrap());
        stream.flush();

        let mut stream = std::io::BufReader::new(std::fs::File::open("./out").unwrap());
        let mut s: Vec<u8> = vec![];
        use std::io::Read;
        stream.read_to_end(&mut s).unwrap();

        bencher.iter(|| {
            let mut reader = &s[..];
            bincode::deserialize_from::<_, Vec<Packet>>(&mut reader).unwrap();
        })
    }
}

mod serde_bytes_slice_json {
    use super::*;

    mod serde_bytes_array {
        use {
            core::convert::TryInto,
            serde::{de::Error, Deserializer, Serializer},
        };

        #[inline(always)]
        pub(crate) fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serde_bytes::serialize(bytes, serializer)
        }

        #[inline(always)]
        pub(crate) fn deserialize<'de, D, const N: usize>(deserializer: D) -> Result<[u8; N], D::Error>
        where
            D: Deserializer<'de>,
        {
            let slice: &[u8] = serde_bytes::deserialize(deserializer)?;
            //dbg!(&slice);
            let array: [u8; N] = slice.try_into().map_err(|_| {
                let expected = format!("[u8; {}]", N);
                D::Error::invalid_length(slice.len(), &expected.as_str())
            })?;
            Ok(array)
        }
    }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct Packet {
        #[serde(with = "serde_bytes_array")]
        buffer: [u8; PACKET_DATA_SIZE],
        flags: u64,
    }
    impl Default for Packet {
        #[inline(always)]
        fn default() -> Self {
            Self {
                buffer: [0; PACKET_DATA_SIZE],
                flags: 3,
            }
        }
    }

    #[bench]
    fn bench_deserialize_serde_bytes_normal(bencher: &mut test::Bencher) {
        use std::io::Write;
        let mut input_packets: Vec<_> = std::iter::repeat(Packet::default()).take(512).collect();
        let mut stream = std::io::BufWriter::new(std::fs::OpenOptions::new().append(true).create(true).open("./out").unwrap());
        test::black_box(serde_json::to_writer(&mut stream, &input_packets).unwrap());
        stream.flush();

        let mut stream = std::io::BufReader::new(std::fs::File::open("./out").unwrap());
        let mut s: Vec<u8> = vec![];
        use std::io::Read;
        stream.read_to_end(&mut s).unwrap();

        bencher.iter(|| {
            test::black_box(serde_json::from_slice::<Vec<Packet>>(&s).unwrap());
        })
    }

    #[bench]
    fn bench_deserialize_from_serde_bytes_normal(bencher: &mut test::Bencher) {
        use std::io::Write;
        let mut input_packets: Vec<_> = std::iter::repeat(Packet::default()).take(512).collect();
        let mut stream = std::io::BufWriter::new(std::fs::OpenOptions::new().append(true).create(true).open("./out").unwrap());
        test::black_box(serde_json::to_writer(&mut stream, &input_packets).unwrap());
        stream.flush();

        let mut stream = std::io::BufReader::new(std::fs::File::open("./out").unwrap());
        let mut s: Vec<u8> = vec![];
        use std::io::Read;
        stream.read_to_end(&mut s).unwrap();

        bencher.iter(|| {
            let mut reader = &s[..];
            serde_json::from_reader::<_, Vec<Packet>>(&mut reader).unwrap();
        })
    }
}

fn main() {
    println!("Hello, world!");
}
