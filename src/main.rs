use noodles::bam as bam;
use noodles::bgzf as bgzf;
use std::env;
use core::num::NonZeroUsize;

fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");
    let ifn = env::args().nth(1).unwrap();
    let worker_count = NonZeroUsize::try_from(4).unwrap();
    let mut reader = bgzf::reader::Builder::default().set_worker_count(worker_count).build_from_path(ifn)?;

    let mut reader = bam::Reader::new(reader);
    reader.read_header()?;
    reader.read_reference_sequences()?;

    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
