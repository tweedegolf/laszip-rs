#[cfg(test)]
mod file_tests {
    #[test]
    fn it_counts_points_in_file() {
        laszip::load_laszip_library();
        let laz = laszip::LazReader::from_file("../data/building.laz");
        assert_eq!(1473, laz.unwrap().get_number_of_points().unwrap());
    }
}
