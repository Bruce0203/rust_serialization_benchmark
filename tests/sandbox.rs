#![feature(new_zeroed_alloc)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(specialization)]

use std::{fs::File, io::Write, str::FromStr};

use fastbuf::{Buf, Buffer, ReadBuf};
use rand_pcg::Lcg64Xsh32;
use rust_serialization_benchmark::datasets::log::{Address, Log, Logs};
use serialization::{Decode, Encode};
use serialization_minecraft::{PacketDecoder, PacketEncoder};

#[test]
fn asdf90() {
    type T = Logs;
    const STATE: u64 = 3141592653;
    const STREAM: u64 = 5897932384;
    const LOGS: usize = 10_000;
    let mut rng = Lcg64Xsh32::new(STATE, STREAM);
    let value = Logs {
        logs: rust_serialization_benchmark::generate_vec::<_, Log>(&mut rng, LOGS..LOGS + 1),
    };
    const BUFFER_LEN: usize = 50_000_000;

    let mut buf = unsafe { Box::<Buffer<BUFFER_LEN>>::new_zeroed().assume_init() };

    let mut enc = PacketEncoder::new(&mut buf);

    println!("break point 1");
    let result = value.encode(&mut enc);
    result.unwrap();
    let mut file = File::create("testtemp").unwrap();
    file.write_all(buf.get_continuous(buf.remaining())).unwrap();
    println!("break point 2");
    {
        let mut dec = PacketDecoder::new(&mut buf);
        let decoded = T::decode(&mut dec).unwrap();
        assert!(value == decoded);
        unsafe { buf.set_pos(0) };
    }
    {
        let mut dec = PacketDecoder::new(&mut buf);
        let decoded = T::decode(&mut dec).unwrap();
        assert!(value == decoded);
        unsafe { buf.set_pos(0) };
    }
    {
        let mut dec = PacketDecoder::new(&mut buf);
        let decoded = T::decode(&mut dec).unwrap();
        assert!(value == decoded);
        unsafe { buf.set_pos(0) };
    }
}
