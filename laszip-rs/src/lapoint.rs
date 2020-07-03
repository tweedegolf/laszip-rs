pub type LazPoint = laszip_sys::laszip_point_struct;

pub trait LazNormalizedPoint {
    fn x(&self, scale: f64, offset: f64) -> f64;
    fn y(&self, scale: f64, offset: f64) -> f64;
    fn z(&self, scale: f64, offset: f64) -> f64;
}

impl LazNormalizedPoint for LazPoint {
    fn x(&self, scale: f64, offset: f64) -> f64 {
        self.X as f64 * scale + offset
    }

    fn y(&self, scale: f64, offset: f64) -> f64 {
        self.Y as f64 * scale + offset
    }

    fn z(&self, scale: f64, offset: f64) -> f64 {
        self.Z as f64 * scale + offset
    }
}

pub trait LazPointCompare {
    fn within_2d(
        &self,
        bounds: &crate::geo::BBox2D,
        offset: &crate::geo::Point3D,
        scale: &crate::geo::Point3D,
    ) -> bool;

    fn within_polygon_2d(
        &self,
        polygon: &crate::geo::Polygon2D,
        offset: &crate::geo::Point3D,
        scale: &crate::geo::Point3D,
    ) -> bool;
}

impl LazPointCompare for LazPoint {
    fn within_2d(
        &self,
        bounds: &crate::geo::BBox2D,
        offset: &crate::geo::Point3D,
        scale: &crate::geo::Point3D,
    ) -> bool {
        let x = self.X as f64 * scale.x + offset.x;
        let y = self.Y as f64 * scale.y + offset.y;
        x >= bounds.min.x && x <= bounds.max.x && y >= bounds.min.y && y <= bounds.max.y
    }

    fn within_polygon_2d(
        &self,
        polygon: &crate::geo::Polygon2D,
        offset: &crate::geo::Point3D,
        scale: &crate::geo::Point3D,
    ) -> bool {
        use crate::geo::Point2D;

        let mut inside = false;
        let p = Point2D::new(
            self.X as f64 * scale.x + offset.x,
            self.Y as f64 * scale.y + offset.y,
        );

        let points: Vec<&Point2D> = polygon
            .points
            .iter()
            .chain(polygon.points.iter().take(1))
            .collect();
        for win in points[..].windows(2) {
            let p1 = win.get(0).unwrap();
            let p2 = win.get(1).unwrap();

            if (p2.y > p.y) != (p1.y > p.y)
                && p.x < (p1.x - p2.x) * (p.y - p2.y) / (p1.y - p2.y) + p2.x
            {
                inside = !inside;
            }
        }
        inside
    }
}
