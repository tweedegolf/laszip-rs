pub type LazHeader = laszip_sys::laszip_header_struct;

pub trait LazHeaderWriter {
    fn set_scale(&mut self, scale: &crate::geo::Point3D);
    fn set_offset(&mut self, offset: &crate::geo::Point3D);
    fn set_version(&mut self, major: u8, minor: u8);
}

impl LazHeaderWriter for LazHeader {
    fn set_scale(&mut self, scale: &crate::geo::Point3D) {
        self.x_scale_factor = scale.x;
        self.y_scale_factor = scale.y;
        self.z_scale_factor = scale.z;
    }

    fn set_offset(&mut self, offset: &crate::geo::Point3D) {
        self.x_offset = offset.x;
        self.y_offset = offset.y;
        self.z_offset = offset.z;
    }

    fn set_version(&mut self, major: u8, minor: u8) {
        self.version_major = major;
        self.version_minor = minor;
    }
}
