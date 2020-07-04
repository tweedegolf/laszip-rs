use laszip::error;
use laszip::lafi::IntensityFilter;
use laszip::lapi::LazPipeline;

fn main() -> Result<(), error::LaszipError> {
    const INTENSITY_MAX: u16 = 200;

    // define input and output files
    let mut pipeline = LazPipeline::from_file(
        String::from("./data/building.laz"),
        String::from("./data/out/filtered.laz"),
        true,
    )?;

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
