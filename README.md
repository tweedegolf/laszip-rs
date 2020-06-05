# laszip-rs
Provides bindings to LASzip, the open-source LiDAR compressor, as well as wrappers to read and write both files and memory.

## Usage

``` git submodule update --init ```

The laszip-sys crate included in this repository needs to access our customized
version of laszip-sys. When running using `cargo run` or `cargo run --release`
the dynamic library path is automatically included. When you want to run a
binary directly you will need to install the library yourself, or add the library path to your environment, the latter can be done using this command, where you need to find
the hash for your laszip-sys build yourself:

```bash
export LD_LIBRARY_PATH="$(pwd)/target/release/build/laszip-sys-<HASH>/out/lib"
```

See also:
- [Original LASzip](https://github.com/LASzip/LASzip)
- [Memory enabled LASzip](https://github.com/tweedegolf/LASzip)
- [laz-rs, Rust implementation of LAZ](https://github.com/tmontaigu/laz-rs)
