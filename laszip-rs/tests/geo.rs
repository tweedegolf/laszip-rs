#[cfg(test)]
mod tests {
    use laszip::geo;

    #[test]
    fn box_extend_correctly_with_points() {
        let p1 = geo::Point3D {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };

        let p2 = geo::Point3D {
            x: 3.0,
            y: 2.0,
            z: 1.0,
        };

        let mut b = geo::BBox3D::universe();
        b.extend(&p1);
        b.extend(&p2);

        assert_eq!(
            b,
            geo::BBox3D {
                min: geo::Point3D {
                    x: 3.0,
                    y: 2.0,
                    z: 1.0,
                },
                max: geo::Point3D {
                    x: 4.0,
                    y: 5.0,
                    z: 6.0,
                }
            }
        );
    }

    #[test]
    fn box_extends_correctly_with_box() {
        let mut b = geo::BBox3D {
            min: geo::Point3D {
                x: 4.0,
                y: 3.0,
                z: 2.0,
            },
            max: geo::Point3D {
                x: 4.0,
                y: 5.0,
                z: 6.0,
            },
        };
        let other = geo::BBox3D {
            min: geo::Point3D {
                x: 3.0,
                y: 6.0,
                z: 1.0,
            },
            max: geo::Point3D {
                x: 7.0,
                y: 8.0,
                z: 9.0,
            },
        };
        b.extend_box(&other);

        assert_eq!(
            b,
            geo::BBox3D {
                min: geo::Point3D {
                    x: 3.0,
                    y: 3.0,
                    z: 1.0,
                },
                max: geo::Point3D {
                    x: 7.0,
                    y: 8.0,
                    z: 9.0,
                }
            }
        );
    }
}
