use crate::lapoint::LazPoint;

pub trait LazFilter {
    fn apply(&self, point: &LazPoint) -> bool;
}

pub struct IntensityFilter {
    pub min: Option<u16>,
    pub max: Option<u16>,
}

impl LazFilter for IntensityFilter {
    fn apply(&self, point: &LazPoint) -> bool {
        if let Some(imax) = self.max {
            if point.intensity > imax {
                return false;
            }
        }
        if let Some(imin) = self.min {
            if point.intensity < imin {
                return false;
            }
        }
        true
    }
}
