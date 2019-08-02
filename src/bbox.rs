use std::ops::Add;

use math::*;

// trait Container {
//     fn contains(&self, container: &dyn HasContainer<Container>) -> bool;
// }

// trait HasContainer<T: Container> {
//     fn get_container(&self) -> T;
// }

// // Implemented as binary
// // Leaf nodes?
// struct SpatialTree<T: Container> {
//     container: T,
//     // Option?
//     left: Box<SpatialTree<T>>,
//     // Option?
//     right: Box<SpatialTree<T>>,
// }

// impl <T: Container> SpatialTree<T> {
//     fn add(&mut self,obj: Box<dyn HasContainer<T>>) {
//         if self.left.container.contains(&obj) {
//             self.left.add(obj);
//         } else if self.right.container.contains(&obj) {
//             self.right.add(obj);
//         }
//     }
// }

#[derive(Default, Debug, Copy, Clone)]
struct BoundingBox {
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
    min_z: f64,
    max_z: f64,
}

impl BoundingBox {
    fn norm(&self) -> f64 {
        f64::max(
            f64::max(self.max_x - self.min_x, self.max_y - self.min_y),
            self.max_z - self.min_z,
        )
    }
}

impl Add for BoundingBox {
    type Output = BoundingBox;

    // Union, containing box?
    // Tests
    fn add(self, other: BoundingBox) -> BoundingBox {
        BoundingBox {
            min_x: f64::min(self.min_x, other.min_x),
            min_y: f64::min(self.min_y, other.min_y),
            min_z: f64::min(self.min_z, other.min_z),
            max_x: f64::max(self.max_x, other.max_x),
            max_y: f64::max(self.max_y, other.max_y),
            max_z: f64::max(self.max_z, other.max_z),
        }
    }
}

enum BoundingBoxTree {
    Node {
        bounding_box: BoundingBox,
        // Maybe don't need this option? Could use empty leaf nodes instead.
        left: Option<Box<BoundingBoxTree>>,
        // Maybe don't need this option?
        right: Option<Box<BoundingBoxTree>>,
    },
    Leaf {
        object: Box<dyn HasBoundingBox>,
    },
}

impl BoundingBoxTree {
    fn add(self, obj: Box<dyn HasBoundingBox>) -> BoundingBoxTree {
        match self {
            BoundingBoxTree::Node {
                bounding_box: _,
                left: None,
                right: None,
            } => BoundingBoxTree::Node {
                bounding_box: obj.get_bounding_box(),
                left: Some(Box::new(BoundingBoxTree::Leaf { object: obj })),
                right: None,
            },
            BoundingBoxTree::Node {
                bounding_box,
                left: left @ Some(_),
                right: None,
            } => BoundingBoxTree::Node {
                bounding_box: bounding_box + obj.get_bounding_box(),
                left: left,
                right: Some(Box::new(BoundingBoxTree::Leaf { object: obj })),
            },
            BoundingBoxTree::Node {
                bounding_box,
                left: Some(node_l),
                right: Some(node_r),
            } => {
                // This node is full
                // Put in smaller box
                let bounding_box = bounding_box + obj.get_bounding_box();
                let (node_l, node_r) = if (obj.get_bounding_box() + node_l.get_bounding_box()).norm()
                    < (obj.get_bounding_box() + node_r.get_bounding_box()).norm()
                {
                    (Box::new(node_l.add(obj)), node_r)
                } else {
                    (node_l, Box::new(node_r.add(obj)))
                };
                BoundingBoxTree::Node {
                    bounding_box: bounding_box,
                    left: Some(node_l),
                    right: Some(node_r),
                }
            }
            BoundingBoxTree::Leaf { object } => BoundingBoxTree::Node {
                bounding_box: object.get_bounding_box() + obj.get_bounding_box(),
                left: Some(Box::new(BoundingBoxTree::Leaf { object: object })),
                right: Some(Box::new(BoundingBoxTree::Leaf { object: obj })),
            },
            _ => {
                panic!("Invalid tree state");
            }
        }
    }

    fn create_empty() -> BoundingBoxTree {
        BoundingBoxTree::Node {
            bounding_box: BoundingBox {
                ..Default::default()
            },
            left: None,
            right: None,
        }
    }
}

trait HasBoundingBox {
    fn get_bounding_box(&self) -> BoundingBox;
}

impl HasBoundingBox for Sphere {
    fn get_bounding_box(&self) -> BoundingBox {
        BoundingBox {
            min_x: self.center.x - self.radius,
            min_y: self.center.y - self.radius,
            min_z: self.center.z - self.radius,
            max_x: self.center.x + self.radius,
            max_y: self.center.y + self.radius,
            max_z: self.center.z + self.radius,
        }
    }
}

impl HasBoundingBox for BoundingBoxTree {
    fn get_bounding_box(&self) -> BoundingBox {
        match self {
            BoundingBoxTree::Node { bounding_box, .. } => bounding_box.clone(),
            BoundingBoxTree::Leaf { object } => object.get_bounding_box(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    trait AlmostEquality {
        fn almost_equals(&self, other: Self) -> bool;
    }

    const EPSILON: f64 = 1e-13;

    impl AlmostEquality for f64 {
        fn almost_equals(&self, other: f64) -> bool {
            (self - other).abs() < EPSILON * self.abs()
        }
    }

    impl BoundingBox {
        fn almost_equals(&self, other: &BoundingBox) -> bool {
            self.min_x.almost_equals(other.min_x)
                && self.min_y.almost_equals(other.min_y)
                && self.min_z.almost_equals(other.min_z)
                && self.max_x.almost_equals(other.max_x)
                && self.max_y.almost_equals(other.max_y)
                && self.max_z.almost_equals(other.max_z)
        }
    }

    #[test]
    fn create_empty() {
        let tree = BoundingBoxTree::create_empty();
        match tree {
            BoundingBoxTree::Node {
                bounding_box,
                left,
                right,
            } => {
                assert!(bounding_box.min_y == 0.0);
                assert!(left.is_none());
                assert!(right.is_none());
            }
            _ => {
                assert!(false);
            }
        }
    }

    #[test]
    fn add_item() {
        let tree = BoundingBoxTree::create_empty();
        let sphere = Sphere {
            center: Point::origin(),
            radius: 1.0,
        };
        let tree = tree.add(Box::new(sphere.clone()));
        match tree {
            BoundingBoxTree::Node {
                bounding_box,
                left,
                right,
            } => {
                println!("bbox: {:?}", bounding_box);
                println!("sphere bbox: {:?}", sphere.get_bounding_box());
                assert!(left.is_some() ^ right.is_some());
                assert!(sphere.get_bounding_box().almost_equals(&bounding_box));
            }
            _ => {
                assert!(false);
            }
        }
    }

    #[test]
    fn add_2_items() {
        let tree = BoundingBoxTree::create_empty();
        let sphere1 = Sphere {
            center: Point::origin(),
            radius: 1.0,
        };
        let sphere2 = Sphere {
            center: Point {
                x: 3.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 1.0,
        };
        let tree = tree.add(Box::new(sphere1.clone()));
        let tree = tree.add(Box::new(sphere2.clone()));
        match tree {
            BoundingBoxTree::Node {
                bounding_box,
                left,
                right,
            } => {
                assert!(bounding_box
                    .almost_equals(&(sphere1.get_bounding_box() + sphere2.get_bounding_box())));
                assert!(left.is_some() && right.is_some()); // This is somewhat debatable.
            }
            _ => {
                assert!(false);
            }
        }
    }

    #[test]
    fn add_3_items() {
        let tree = BoundingBoxTree::create_empty();
        let sphere1 = Sphere {
            center: Point::origin(),
            radius: 1.0,
        };
        let sphere2 = Sphere {
            center: Point {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            radius: 2.0,
        };
        let sphere3 = Sphere {
            center: Point {
                x: 4.0,
                y: 5.0,
                z: 6.0,
            },
            radius: 3.0,
        };
        let tree = tree.add(Box::new(sphere1.clone()));
        let tree = tree.add(Box::new(sphere2.clone()));
        let tree = tree.add(Box::new(sphere3.clone()));
        match tree {
            BoundingBoxTree::Node {
                bounding_box,
                left,
                right,
            } => {
                assert!(bounding_box.almost_equals(
                    &(sphere1.get_bounding_box()
                        + sphere2.get_bounding_box()
                        + sphere3.get_bounding_box())
                ));
                assert!(left.is_some() && right.is_some()); // This is somewhat debatable.
            }
            _ => {
                assert!(false);
            }
        }
    }
}
