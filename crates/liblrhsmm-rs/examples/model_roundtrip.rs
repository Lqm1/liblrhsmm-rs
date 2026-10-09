//! Legacy-format interoperability with an external C reference tool.
use liblrhsmm_rs::Model;
use std::{fs::File, io};
fn main() -> io::Result<()> {
    let mut arguments = std::env::args_os().skip(1);
    let input = arguments
        .next()
        .ok_or_else(|| io::Error::other("missing input model"))?;
    let output = arguments
        .next()
        .ok_or_else(|| io::Error::other("missing output model"))?;
    let model = Model::read_from(File::open(input)?)?;
    model.write_to(File::create(output)?)?;
    println!(
        "streams={} durations={}",
        model.streams.len(),
        model.durations.len()
    );
    Ok(())
}
