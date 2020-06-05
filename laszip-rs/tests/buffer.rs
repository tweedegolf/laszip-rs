#[cfg(test)]
mod tests {
    #[test]
    fn it_counts_points_in_buffer() {
        use std::io::prelude::*;
        use std::fs::File;

        let mut f = File::open("../data/building.laz").unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();

        laszip::load_laszip_library();
        let laz = laszip::LazReader::from_vec(buffer);
        assert_eq!(1473, laz.unwrap().get_number_of_points().unwrap());
    }
}
