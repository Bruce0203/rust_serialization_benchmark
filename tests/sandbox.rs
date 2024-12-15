use fastbuf::Buffer;
use rust_serialization_benchmark::datasets::mesh::{Mesh, Triangle, Vector3};
use serialization::{Decode, Encode};
use serialization_minecraft::{PacketDecoder, PacketEncoder};

#[test]
fn asdf90() {
    let value = Mesh {
        triangles: vec![Triangle {
            v0: Vector3 {
                x: 123.,
                y: 234.,
                z: 456.,
            },
            v1: Vector3 {
                x: 123.,
                y: 234.,
                z: 456.,
            },
            v2: Vector3 {
                x: 123.,
                y: 234.,
                z: 456.,
            },
            normal: Vector3 {
                x: 123.,
                y: 234.,
                z: 456.,
            },
        }],
    };
    let mut buf = Buffer::<10000>::new();
    let mut enc = PacketEncoder::new(&mut buf);
    println!("break point 1");
    let result = value.encode(&mut enc);
    result.unwrap();
    println!("break point 2");
    let mut dec = PacketDecoder::new(&mut buf);
    let decoded = Mesh::decode(&mut dec).unwrap();
    println!("break point 3");
    assert_eq!(value, decoded);
    println!("break point 4");
}
