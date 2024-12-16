use std::{any::type_name, fmt::Debug};

use criterion::{black_box, Criterion};
use fastbuf::{Buf, Buffer, ReadBuf};
use serialization::{Decode, Encode, Serializable};
use serialization_minecraft::{PacketDecoder, PacketEncoder};

pub fn bench<'de, T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Encode + Decode<'de> + PartialEq + Debug,
{
    let mut group = c.benchmark_group(format!("{}/serialization", name));
    println!("serialization bench type = {}", type_name::<T>());

    const BUFFER_LEN: usize = 50_000_000;

    group.bench_function("serialize", |b| {
        b.iter(|| {
            let mut buf = unsafe { Box::<Buffer<BUFFER_LEN>>::new_zeroed().assume_init() };
            let ref mut encoder = PacketEncoder::new(&mut buf);
            let _result = black_box(data.encode(encoder));
        })
    });
    let mut buf = {
        let mut buf = Box::new(Buffer::<BUFFER_LEN>::new());
        let ref mut encoder = PacketEncoder::new(&mut buf);
        let _result = black_box(data.encode(encoder)).unwrap();
        buf
    };
    {
        println!("breakpoint2");
        let ref mut decoder = PacketDecoder::new(&mut buf);
        println!("breakpoint2");
        let result = black_box(T::decode(decoder)).unwrap();
        println!("breakpoint3");
        unsafe { buf.set_pos(0) };
        drop(result);
        println!("breakpoint4");
    }
    println!("bp5");
    {
        println!("breakpoint1");
        let ref mut decoder = PacketDecoder::new(&mut buf);
        println!("breakpoint2");
        let result = black_box(T::decode(decoder));
        println!("breakpoint3");
        let result = result.unwrap();
        println!("breakpoint4");
        unsafe { buf.set_pos(0) };
        drop(result);
        println!("breakpoint5");
    }
    println!("bp5.2");

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            unsafe { buf.set_pos(0) };
            let ref mut decoder = PacketDecoder::new(&mut buf);
            let _result = black_box(T::decode(decoder)).unwrap();
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
