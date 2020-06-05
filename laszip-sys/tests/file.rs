#[cfg(test)]
mod file_tests {
    use std::sync::Once;
    static LOAD: Once = Once::new();

    #[test]
    fn it_loads_lib() {
        LOAD.call_once(|| {
            let result = unsafe { laszip_sys::laszip_load_dll() };
            assert_eq!(result, 0);
        });
    }

    #[test]
    fn it_counts_points_in_file() {
        use std::ffi::CString;
        use std::ptr;

        LOAD.call_once(|| {
            let result = unsafe { laszip_sys::laszip_load_dll() };
            assert_eq!(result, 0);
        });

        let file_name = "../data/building.laz";
        unsafe {
            let mut reader = ptr::null_mut();
            assert_eq!(0, laszip_sys::laszip_create(&mut reader));

            let mut is_compressed = 0;
            assert_eq!(
                0,
                laszip_sys::laszip_open_reader(
                    reader,
                    CString::new(file_name).unwrap().as_ptr(),
                    &mut is_compressed,
                )
            );

            let mut header = ptr::null_mut();
            assert_eq!(
                0,
                laszip_sys::laszip_get_header_pointer(reader, &mut header)
            );

            let npoints = (*header).number_of_point_records as u64;
            assert_eq!(1473, npoints);
        }
    }
}
