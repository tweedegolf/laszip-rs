# laszip-rs
Provides bindings to LASzip, the open-source LiDAR compressor, as well as wrappers to read and write both files and memory.

## Setup

``` git submodule update --init ```

The laszip-sys crate included in this repository needs to access our customized
version of laszip-sys. When running using `cargo` the dynamic library path is 
automatically included. 
When you want to run a binary directly you will need to install the library yourself, 
or add the library path to your environment, the latter can be done using this command, 
where you need to find the hash for your laszip-sys build yourself:

```bash
export LD_LIBRARY_PATH="$(pwd)/target/release/build/laszip-sys-<HASH>/out/lib"
```

## Usage example

You can use `LazReader` and `LazWriter` to have complete control over data processing using files or in memory.
For example, to read data from file, filter on height (z) and write to output file:

```rust
let reader = laszip::LazReader::from_file("../data/building.laz").unwrap();
let (scale, offset) = (reader.get_header()?.scale(), reader.get_header()?.offset());
let mut writer =
    laszip::LazWriter::from_file("../data/out/roof.laz", true, &scale, &offset).unwrap();
//!
for p in reader.iter_points()?.filter(|p| {
    if let Ok(p) = p {
        let z = p.Z as f64 * scale.z + offset.z;
        z > 6.0
    } else {
        false
    }
}) {
    writer.push(p.unwrap())?;
}
```

laszip-rs also provides an optional layer to make filtering more convenient, using `LazPipeline`:

```rust
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
```


## See also
- [Original LASzip](https://github.com/LASzip/LASzip)
- [Memory enabled LASzip](https://github.com/tweedegolf/LASzip)
- [laz-rs, Rust implementation of LAZ](https://github.com/tmontaigu/laz-rs)
