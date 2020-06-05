use crate::error::Result;
use crate::laheader::{LazHeader, LazHeaderWriter};
use crate::lapoint::LazPoint;
use crate::lautil::ErrorHandler;

pub struct LazWriter {
    ptr: laszip_sys::laszip_POINTER,
    points_written: usize,
}

impl ErrorHandler for LazWriter {
    fn handle_error(&self, res: laszip_sys::laszip_I32) -> Result<()> {
        self.ptr.handle_error(res)
    }
}

impl Drop for LazWriter {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            self.handle_error(unsafe { laszip_sys::laszip_destroy(self.ptr) })
                .unwrap();
        }
    }
}

impl LazWriter {
    pub fn new(
        alloc: usize,
        compress: bool,
        scale: &crate::geo::Point3D,
        offset: &crate::geo::Point3D,
    ) -> Result<LazWriter> {
        let mut writer = LazWriter {
            ptr: crate::lautil::create_laszip(),
            points_written: 0,
        };
        let header = writer.header_mut()?;
        header.set_scale(scale);
        header.set_offset(offset);
        header.set_version(1, 2);

        writer.handle_error(unsafe {
            laszip_sys::laszip_open_writer_array(writer.ptr, alloc as i64, compress as i32)
        })?;
        Ok(writer)
    }

    pub fn push(&mut self, point: &LazPoint) -> Result<()> {
        self.handle_error(unsafe {
            // println!("set point");
            laszip_sys::laszip_set_point(self.ptr, point)
        })?;
        self.handle_error(unsafe {
            // println!("write point");
            laszip_sys::laszip_write_point(self.ptr)
        })?;
        self.handle_error(unsafe {
            // println!("update inventory");
            laszip_sys::laszip_update_inventory(self.ptr)
        })?;

        self.points_written += 1;

        Ok(())
    }

    pub fn into_data(self) -> Result<Vec<u8>> {
        self.handle_error(unsafe { laszip_sys::laszip_close_writer(self.ptr) })?;
        let mut data: *mut laszip_sys::laszip_U8 = std::ptr::null_mut();
        let mut data_size: i64 = 0;
        self.handle_error(unsafe {
            laszip_sys::laszip_writer_get_data(self.ptr, &mut data, &mut data_size)
        })?;

        let vec_data = unsafe { std::slice::from_raw_parts(data, data_size as usize) }.to_vec();
        unsafe {
            libc::free(data as *mut std::ffi::c_void);
        }

        Ok(vec_data)
    }

    pub fn header_mut(&mut self) -> Result<&mut LazHeader> {
        let mut header_ptr = std::ptr::null_mut();
        self.handle_error(unsafe {
            laszip_sys::laszip_get_header_pointer(self.ptr, &mut header_ptr)
        })?;

        Ok(unsafe { &mut *header_ptr })
    }
}
