#![feature(test)]
extern crate test;
extern crate byteio;
extern crate byteorder;

#[bench]
fn bench_byteio_vec(b: &mut test::Bencher) {
    use byteio::ReadBytesExt;
    let vec = vec![0u8; 1_000_000];
    b.iter(|| {
        let data = test::black_box(&vec[..]);
        for mut val in data.chunks(2) {
            let _: Result<u16, _> = val.read::<byteio::LittleEndian>();
        }
    });
    b.bytes = vec.len() as u64;
}

#[bench]
fn bench_byteio(b: &mut test::Bencher) {
    use byteio::ByteIo;
    const NITER: i32 = 100_000;
    b.iter(|| {
        for _ in 1..NITER {
            let data = test::black_box([1, 2]);
            let _: u16 = byteio::LittleEndian::from_bytes(data);
        }
    });
    b.bytes = 2 * NITER as u64;
}

#[bench]
fn bench_byteorder_vec(b: &mut test::Bencher) {
    use byteorder::ReadBytesExt;
    let vec = vec![0u8; 1_000_000];
    b.iter(|| {
        let data = test::black_box(&vec[..]);
        for mut val in data.chunks(2) {
            let _: Result<u16, _> = val.read_u16::<byteorder::LittleEndian>();
        }
    });
    b.bytes = vec.len() as u64;
}

#[bench]
fn bench_byteorder(b: &mut test::Bencher) {
    use byteorder::ByteOrder;
    const NITER: i32 = 100_000;
    b.iter(|| {
        for _ in 1..NITER {
            let data = test::black_box([1, 2]);
            let _: u16 = byteorder::LittleEndian::read_u16(&data);
        }
    });
    b.bytes = 2 * NITER as u64;
}