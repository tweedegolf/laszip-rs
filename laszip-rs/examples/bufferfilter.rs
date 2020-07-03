use laszip::error;
use laszip::lafi::IntensityFilter;
use laszip::lapi::LazPipeline;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), error::LaszipError> {
    const INTENSITY_MAX: u16 = 200;

    let mut f = File::open("./data/building.laz").unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();

    // provide laz buffer
    let mut pipeline = LazPipeline::from_vec(buffer, true)?;

    // add a filter
    pipeline.add_filter(Box::new(IntensityFilter {
        min: None,
        max: Some(INTENSITY_MAX),
    }));

    // run the pipeline
    pipeline.run()?;

    // get number of points written
    println!(
        "reader points: {}, writer points: {}",
        pipeline.reader.unwrap().get_number_of_points()?,
        pipeline.writer.unwrap().get_number_of_points_written()
    );

    Ok(())
}
