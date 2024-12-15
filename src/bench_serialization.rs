use std::fmt::Debug;

use criterion::{black_box, Criterion};
use fastbuf::{Buf, Buffer, ReadBuf};
use serialization::{Decode, Encode, Serializable};
use serialization_minecraft::{PacketDecoder, PacketEncoder};

pub fn bench<'de, T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Encode + Decode<'de> + PartialEq + Debug,
{
    let mut group = c.benchmark_group(format!("{}/serialization", name));

    group.bench_function("serialize", |b| {
        b.iter(|| {
            let mut buf = Buffer::<1000>::new();
            let ref mut encoder = PacketEncoder::new(&mut buf);
            let _result = black_box(data.encode(encoder));
        })
    });
    let mut buf = Buffer::<1000>::new();
    let ref mut encoder = PacketEncoder::new(&mut buf);
    black_box(data.encode(encoder)).unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            let ref mut decoder = PacketDecoder::new(&mut buf);
            let _result = black_box(T::decode(decoder));
            unsafe { buf.set_pos(0) };
        })
    });

    crate::bench_size(name, "serialization", buf.get_continuous(buf.remaining()));

    let ref mut decoder = PacketDecoder::new(&mut buf);
    assert_eq!(&T::decode(decoder).unwrap(), data);

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
