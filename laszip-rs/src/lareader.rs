use crate::error::Result;
use crate::laheader::LazHeader;
use crate::lapoint::LazPoint;
use crate::lautil::ErrorHandler;

pub struct LazReader {
    data: Vec<u8>,
    ptr: laszip_sys::laszip_POINTER,
}

pub struct LazPointReaderIterator<'a> {
    ptr: &'a laszip_sys::laszip_POINTER,
    point_ptr: *mut laszip_sys::laszip_point_struct,
    total_points: usize,
    current_point: usize,
}
impl ErrorHandler for LazReader {
    fn handle_error(&self, res: laszip_sys::laszip_I32) -> Result<()> {
        self.ptr.handle_error(res)
    }
}

impl Drop for LazReader {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            self.handle_error(unsafe { laszip_sys::laszip_close_reader(self.ptr) })
                .unwrap();
            self.handle_error(unsafe { laszip_sys::laszip_destroy(self.ptr) })
                .unwrap();
        }
    }
}

impl LazReader {
    pub fn from_vec(data: Vec<u8>) -> Result<Self> {
        let file = Self {
            data,
            ptr: crate::lautil::create_laszip(),
        };

        let data_ptr = file.data.as_ptr();
        let data_len = file.data.len() as i64;
        let mut is_compressed = 0;
        file.handle_error(unsafe {
            laszip_sys::laszip_open_reader_array(file.ptr, data_ptr, data_len, &mut is_compressed)
        })?;

        Ok(file)
    }

    pub fn get_header(&self) -> Result<&LazHeader> {
        let mut header_ptr = std::ptr::null_mut();
        self.handle_error(unsafe {
            laszip_sys::laszip_get_header_pointer(self.ptr, &mut header_ptr)
        })?;

        Ok(unsafe { &*header_ptr })
    }

    pub fn scale(&self) -> Result<crate::geo::Point3D> {
        let header = self.get_header()?;
        Ok(crate::geo::Point3D::new(
            header.x_scale_factor,
            header.y_scale_factor,
            header.z_scale_factor,
        ))
    }

    pub fn offset(&self) -> Result<crate::geo::Point3D> {
        let header = self.get_header()?;
        Ok(crate::geo::Point3D::new(
            header.x_offset,
            header.y_offset,
            header.z_offset,
        ))
    }

    pub fn get_number_of_points(&self) -> Result<usize> {
        let header = self.get_header()?;
        Ok(header.number_of_point_records as usize)
    }

    pub fn iter_points(&'_ self) -> Result<LazPointReaderIterator> {
        let mut p = std::ptr::null_mut();
        self.handle_error(unsafe { laszip_sys::laszip_get_point_pointer(self.ptr, &mut p) })?;
        Ok(LazPointReaderIterator {
            total_points: self.get_number_of_points()?,
            point_ptr: p,
            current_point: 0,
            ptr: &self.ptr,
        })
    }
}

impl<'a> ErrorHandler for LazPointReaderIterator<'a> {
    fn handle_error(&self, res: laszip_sys::laszip_I32) -> Result<()> {
        self.ptr.handle_error(res)
    }
}

impl<'a> Iterator for LazPointReaderIterator<'a> {
    type Item = Result<&'a LazPoint>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_point >= self.total_points {
            None
        } else {
            match self.handle_error(unsafe { laszip_sys::laszip_read_point(*self.ptr) }) {
                Ok(_) => (),
                Err(e) => return Some(Err(e)),
            }
            self.current_point += 1;
            Some(Ok(unsafe { &*self.point_ptr }))
        }
    }
}
