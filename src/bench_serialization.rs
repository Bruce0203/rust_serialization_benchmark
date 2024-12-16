use std::{any::type_name, fmt::Debug, ops::DerefMut};

use criterion::{black_box, Criterion};
use fastbuf::{Buf, Buffer, ReadBuf};
use serialization::{Decode, Encode, Serializable};
use serialization_minecraft::{PacketDecoder, PacketEncoder};

pub fn bench<'de, T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Encode + Decode<'de> + PartialEq + Debug,
{
    let mut group = c.benchmark_group(format!("{}/serialization", name));

    const BUFFER_LEN: usize = 50_000_000;

    let mut buf = unsafe { Box::<Buffer<BUFFER_LEN>>::new_zeroed().assume_init() };
    group.bench_function("serialize", |b| {
        b.iter(|| {
            unsafe { buf.set_filled_pos(0) };
            let ref mut encoder = PacketEncoder::new(&mut buf);
            let _result = black_box(&data.encode(encoder).unwrap());
        })
    });

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            let ref mut decoder = PacketDecoder::new(&mut buf);
            let _result = black_box(&T::decode(decoder).unwrap());
            unsafe { buf.set_pos(0) };
        })
    });
    unsafe { buf.set_pos(0) };
    crate::bench_size(name, "serialization", buf.get_continuous(buf.remaining()));

    group.finish();
}

#[derive(Serializable)]
pub struct AAA {
    value2: AB,
    value: i32,
    vec: Vec<u8>,
}

#[derive(Serializable)]
pub struct AB {
    value: i32,
}
