#[derive(Debug, Clone)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Point3D {
        Point3D { x, y, z }
    }
}

impl Default for Point3D {
    fn default() -> Point3D {
        Point3D::new(Default::default(), Default::default(), Default::default())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Point2D {
        Point2D { x, y }
    }
}

impl Default for Point2D {
    fn default() -> Point2D {
        Point2D::new(Default::default(), Default::default())
    }
}

#[derive(Debug, Clone)]
pub struct BBox3D {
    pub min: Point3D,
    pub max: Point3D,
}

impl BBox3D {
    pub fn as_2d(&self) -> BBox2D {
        BBox2D {
            min: Point2D::new(self.min.x, self.min.y),
            max: Point2D::new(self.max.x, self.max.y),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BBox2D {
    pub min: Point2D,
    pub max: Point2D,
}

impl BBox2D {
    pub fn new(min: Point2D, max: Point2D) -> BBox2D {
        BBox2D { min, max }
    }

    pub fn fix_points(&self) -> BBox2D {
        let x1 = self.min.x;
        let x2 = self.max.x;
        let y1 = self.min.y;
        let y2 = self.max.y;

        BBox2D {
            min: Point2D::new(f64::min(x1, x2), f64::min(y1, y2)),
            max: Point2D::new(f64::max(x1, x2), f64::max(y1, y2)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Polygon2D {
    pub points: Vec<Point2D>,
}

impl Polygon2D {
    pub fn new(points: Vec<Point2D>) -> Polygon2D {
        Polygon2D { points }
    }

    pub fn bbox(&self) -> BBox2D {
        let mut minx = std::f64::MAX;
        let mut miny = std::f64::MAX;
        let mut maxx = std::f64::MIN;
        let mut maxy = std::f64::MIN;

        for point in self.points.iter() {
            minx = minx.min(point.x);
            miny = miny.min(point.y);
            maxx = maxx.max(point.x);
            maxy = maxy.max(point.y);
        }

        BBox2D::new(Point2D::new(minx, miny), Point2D::new(maxx, maxy))
    }
}
