use byte_unit::Byte;
use fstd;
use std::{
    env,
    fs::{self, File},
    io::{BufWriter, Write},
    time::SystemTime,
};

const CAP: usize = 64 * 1024;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Require size")
    }

    let target_input = &args[1];
    let target = Byte::from_str(target_input).unwrap();

    // Generate random file name and data.
    let random_filename = format!("{:X}.tmp", fstd::rand::u64(..));
    let mut random_data: [u8; CAP] = [0; CAP];

    for i in random_data.iter_mut() {
        *i = fstd::rand::u64(..) as u8;
    }

    // Open this random file.
    let file = match File::create(random_filename.clone()) {
        Err(err) => panic!("couldn't create {}: {}", random_filename, err),
        Ok(file) => file,
    };

    let mut bf = BufWriter::with_capacity(CAP, file);
    let mut length = 0usize;
    let start = SystemTime::now();
    loop {
        let data = bf.write(random_data[..].try_into().unwrap()).unwrap();
        length = length + data;
        if length > (target.get_bytes() as usize) {
            break;
        }
    }
    bf.get_ref().sync_all().unwrap();

    let end = SystemTime::now()
        .duration_since(start)
        .expect("time went backwards");
    let target_mb = target
        .get_adjusted_unit(byte_unit::ByteUnit::MiB)
        .get_value();

    // Remove temp file.
    fs::remove_file(random_filename).unwrap();

    println!(
        "Total size: {} MiB\nAvg speed: {} MiB/s",
        target_mb,
        target_mb / end.as_secs_f64()
    );
}
