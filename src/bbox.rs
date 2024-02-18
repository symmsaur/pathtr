use std::fmt::Debug;
use std::ops::Add;

use crate::math::*;
use crate::scene;

#[derive(Default, Debug, Copy, Clone)]
pub struct BoundingBox {
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
    min_z: f32,
    max_z: f32,
}

impl BoundingBox {
    // FIXME: Could use a better norm?
    fn norm(&self) -> f32 {
        f32::max(
            f32::max(self.max_x - self.min_x, self.max_y - self.min_y),
            self.max_z - self.min_z,
        )
    }
    fn center(&self) -> Point {
        Point {
            x: (self.max_x + self.min_x) / 2.0,
            y: (self.max_y + self.min_y) / 2.0,
            z: (self.max_z + self.min_z) / 2.0,
        }
    }
}

impl Add for BoundingBox {
    type Output = BoundingBox;

    fn add(self, other: BoundingBox) -> BoundingBox {
        BoundingBox {
            min_x: f32::min(self.min_x, other.min_x),
            min_y: f32::min(self.min_y, other.min_y),
            min_z: f32::min(self.min_z, other.min_z),
            max_x: f32::max(self.max_x, other.max_x),
            max_y: f32::max(self.max_y, other.max_y),
            max_z: f32::max(self.max_z, other.max_z),
        }
    }
}

impl BoundingBox {
    pub fn check_intersects(&self, ray: &Ray) -> bool {
        // To hit we need to hit one of six sides of the box
        // Start by checking the min_x plane
        let t = (self.min_x - ray.origin.x) / ray.direction.x;
        let p = translate(ray.origin, t * ray.direction);
        if t > 0.0 && p.y > self.min_y && p.y < self.max_y && p.z > self.min_z && p.z < self.max_z {
            return true;
        }
        // max_x
        let t = (self.max_x - ray.origin.x) / ray.direction.x;
        let p = translate(ray.origin, t * ray.direction);
        if t > 0.0 && p.y > self.min_y && p.y < self.max_y && p.z > self.min_z && p.z < self.max_z {
            return true;
        }
        // min_y
        let t = (self.min_y - ray.origin.y) / ray.direction.y;
        let p = translate(ray.origin, t * ray.direction);
        if t > 0.0 && p.x > self.min_x && p.x < self.max_x && p.z > self.min_z && p.z < self.max_z {
            return true;
        }
        // max_y
        let t = (self.max_y - ray.origin.y) / ray.direction.y;
        let p = translate(ray.origin, t * ray.direction);
        if t > 0.0 && p.x > self.min_x && p.x < self.max_x && p.z > self.min_z && p.z < self.max_z {
            return true;
        }
        // min_z
        let t = (self.min_z - ray.origin.z) / ray.direction.z;
        let p = translate(ray.origin, t * ray.direction);
        if t > 0.0 && p.x > self.min_x && p.x < self.max_x && p.y > self.min_y && p.y < self.max_y {
            return true;
        }
        // max_z
        let t = (self.max_z - ray.origin.z) / ray.direction.z;
        let p = translate(ray.origin, t * ray.direction);
        if t > 0.0 && p.x > self.min_x && p.x < self.max_x && p.y > self.min_y && p.y < self.max_y {
            return true;
        }
        return false;
    }
}

pub trait Shape: HasBoundingBox + Intersectable + std::fmt::Debug {}

#[derive(Debug)]
pub struct Node {
    pub bounding_box: BoundingBox,
    // Maybe don't need this option? Could use empty leaf nodes instead.
    pub left: Option<Box<BoundingBoxTree>>,
    // Maybe don't need this option?
    pub right: Option<Box<BoundingBoxTree>>,
}

#[derive(Debug)]
pub struct Leaf {
    pub object: scene::Object,
}

#[derive(Debug)]
pub enum BoundingBoxTree {
    Node(Node),
    Leaf(Leaf),
}

impl BoundingBoxTree {
    pub fn add(self, object: scene::Object) -> BoundingBoxTree {
        match self {
            // Node is empty => put in left
            // terminal
            BoundingBoxTree::Node(Node {
                bounding_box: _,
                left: None,
                right: None,
            }) => BoundingBoxTree::Node(Node {
                bounding_box: object.shape.get_bounding_box(),
                left: Some(Box::new(BoundingBoxTree::Leaf(Leaf { object }))),
                right: None,
            }),
            // Node has something in left => put in right
            // terminal
            BoundingBoxTree::Node(Node {
                bounding_box,
                left: left @ Some(_),
                right: None,
            }) => BoundingBoxTree::Node(Node {
                bounding_box: bounding_box + object.shape.get_bounding_box(),
                left,
                right: Some(Box::new(BoundingBoxTree::Leaf(Leaf { object }))),
            }),
            // Node is full => combine with smaller node recursively
            // XXX: This is basically a sift down?
            // XXX: Can only do log(n) steps here and preserve ok scaling.
            // recursive
            BoundingBoxTree::Node(Node {
                bounding_box,
                left: Some(node_l),
                right: Some(node_r),
            }) => {
                // FIXME: This strategy does not work
                // This node is full. Put new object in smaller box or merge boxes
                let bounding_box = bounding_box + object.shape.get_bounding_box();

                // FIXME: Should really check which grows the bounding boxes more
                // Combined bounding box sizes
                let lo_bb_size =
                    (object.shape.get_bounding_box() + node_l.get_bounding_box()).norm();
                let ro_bb_size =
                    (object.shape.get_bounding_box() + node_r.get_bounding_box()).norm();
                let lr_bb_size = (node_l.get_bounding_box() + node_r.get_bounding_box()).norm();

                let (node_l, node_r) = if lo_bb_size < f32::min(ro_bb_size, lr_bb_size) {
                    (Box::new(node_l.add(object)), node_r)
                } else if lr_bb_size < f32::max(lo_bb_size, ro_bb_size) {
                    (
                        Box::new(node_l.merge(*node_r)),
                        Box::new(BoundingBoxTree::Leaf(Leaf { object })),
                    )
                } else {
                    (node_l, Box::new(node_r.add(object)))
                };
                BoundingBoxTree::Node(Node {
                    bounding_box,
                    left: Some(node_l),
                    right: Some(node_r),
                })
            }
            // Leaf => upgrade to Node
            BoundingBoxTree::Leaf(Leaf {
                object: existing_object,
            }) => BoundingBoxTree::Node(Node {
                bounding_box: object.shape.get_bounding_box()
                    + existing_object.shape.get_bounding_box(),
                left: Some(Box::new(BoundingBoxTree::Leaf(Leaf {
                    object: existing_object,
                }))),
                right: Some(Box::new(BoundingBoxTree::Leaf(Leaf { object }))),
            }),
            _ => {
                panic!("Invalid tree state");
            }
        }
    }

    //pub fn add(self, object: scene::Object) -> BoundingBoxTree {
    pub fn merge(self, other: BoundingBoxTree) -> BoundingBoxTree {
        BoundingBoxTree::Node(Node {
            bounding_box: self.get_bounding_box() + other.get_bounding_box(),
            left: Some(Box::new(self)),
            right: Some(Box::new(other)),
        })
    }

    pub fn create_empty() -> BoundingBoxTree {
        BoundingBoxTree::Node(Node {
            bounding_box: BoundingBox {
                ..Default::default()
            },
            left: None,
            right: None,
        })
    }
}

impl Default for BoundingBoxTree {
    fn default() -> BoundingBoxTree {
        BoundingBoxTree::create_empty()
    }
}

pub trait HasBoundingBox {
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

impl HasBoundingBox for Plane {
    fn get_bounding_box(&self) -> BoundingBox {
        BoundingBox {
            min_x: f32::NEG_INFINITY,
            min_y: f32::NEG_INFINITY,
            min_z: f32::NEG_INFINITY,
            max_x: f32::INFINITY,
            max_y: f32::INFINITY,
            max_z: f32::INFINITY,
        }
    }
}

impl Shape for Plane {}

impl Shape for Sphere {}

impl HasBoundingBox for BoundingBoxTree {
    fn get_bounding_box(&self) -> BoundingBox {
        match self {
            BoundingBoxTree::Node(Node { bounding_box, .. }) => bounding_box.clone(),
            BoundingBoxTree::Leaf(Leaf { object }) => object.shape.get_bounding_box(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Node, *};

    use itertools::Itertools;

    use crate::material;

    fn wrap_in_object(shape: Box<dyn Shape>) -> scene::Object {
        scene::Object {
            shape,
            material: material::Material::create_colored_1(),
        }
    }

    // FIXME: Extract all the almost-equals stuff
    trait AlmostEquality {
        fn almost_equals(&self, other: Self) -> bool;
    }

    const EPSILON: f32 = 1e-13;

    impl AlmostEquality for f32 {
        fn almost_equals(&self, other: f32) -> bool {
            (self - other).abs() <= EPSILON * self.abs()
        }
    }

    impl AlmostEquality for BoundingBox {
        fn almost_equals(&self, other: BoundingBox) -> bool {
            self.min_x.almost_equals(other.min_x)
                && self.min_y.almost_equals(other.min_y)
                && self.min_z.almost_equals(other.min_z)
                && self.max_x.almost_equals(other.max_x)
                && self.max_y.almost_equals(other.max_y)
                && self.max_z.almost_equals(other.max_z)
        }
    }

    impl AlmostEquality for Point {
        fn almost_equals(&self, other: Point) -> bool {
            println!("{:?} == {:?}", self, other);
            self.x.almost_equals(other.x)
                && self.y.almost_equals(other.y)
                && self.z.almost_equals(other.z)
        }
    }

    #[test]
    fn create_empty() {
        let tree = BoundingBoxTree::create_empty();
        match tree {
            BoundingBoxTree::Node(Node {
                bounding_box,
                left,
                right,
            }) => {
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
    fn bounding_box_center() {
        let bbox = BoundingBox {
            min_x: -1.0,
            max_x: 1.0,
            min_y: 0.0,
            max_y: 2.0,
            min_z: -2.0,
            max_z: 0.0,
        };
        println!("bbox.center() {:?}", bbox.center());
        assert!(bbox.center().almost_equals(Point::new(0.0, 1.0, -1.0)));
    }

    #[test]
    fn add_item() {
        let tree = BoundingBoxTree::create_empty();
        let sphere = Sphere {
            center: Point::origin(),
            radius: 1.0,
        };
        let tree = tree.add(wrap_in_object(Box::new(sphere.clone())));
        match tree {
            BoundingBoxTree::Node(Node {
                bounding_box,
                left,
                right,
            }) => {
                println!("bbox: {:?}", bounding_box);
                println!("sphere bbox: {:?}", sphere.get_bounding_box());
                assert!(left.is_some() ^ right.is_some());
                assert!(sphere.get_bounding_box().almost_equals(bounding_box));
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
        let tree = tree.add(wrap_in_object(Box::new(sphere1.clone())));
        let tree = tree.add(wrap_in_object(Box::new(sphere2.clone())));
        match tree {
            BoundingBoxTree::Node(Node {
                bounding_box,
                left,
                right,
            }) => {
                assert!(bounding_box
                    .almost_equals(sphere1.get_bounding_box() + sphere2.get_bounding_box()));
                assert!(left.is_some() && right.is_some()); // This is somewhat debatable.
            }
            _ => {
                assert!(false);
            }
        }
    }

    #[test]
    fn add_3_items_invariants() {
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
        let tree = tree.add(wrap_in_object(Box::new(sphere1.clone())));
        let tree = tree.add(wrap_in_object(Box::new(sphere2.clone())));
        let tree = tree.add(wrap_in_object(Box::new(sphere3.clone())));
        match tree {
            BoundingBoxTree::Node(Node {
                bounding_box,
                left,
                right,
            }) => {
                assert!(bounding_box.almost_equals(
                    sphere1.get_bounding_box()
                        + sphere2.get_bounding_box()
                        + sphere3.get_bounding_box()
                ));
                assert!(left.is_some() && right.is_some()); // This is somewhat debatable.
            }
            _ => {
                assert!(false);
            }
        }
    }

    #[test]
    fn add_3_items_optimized() {
        let items = vec![
            Sphere {
                center: Point::origin(),
                radius: 1.0,
            },
            Sphere {
                center: Point::new(0.0, 2.0, 0.0),
                radius: 1.0,
            },
            Sphere {
                center: Point::new(5.0, 0.0, 0.0),
                radius: 1.0,
            },
        ];

        for items_permutation in items.clone().into_iter().permutations(3) {
            println!("Permutation {:?}", items_permutation);
            let mut tree = BoundingBoxTree::create_empty();
            for item in items_permutation {
                tree = tree.add(wrap_in_object(Box::new(item)));
            }
            // println!("{:#?}", tree);
            let node = match tree {
                BoundingBoxTree::Node(node) => node,
                _ => panic!("Didn't expect Leaf"),
            };
            for child in [node.left, node.right] {
                match child {
                    Some(node) => {
                        match *node {
                            BoundingBoxTree::Node(Node { bounding_box, .. }) => {
                                println!("Checking bounding box {:?}", bounding_box);
                                assert!(f32::almost_equals(&-1.0, bounding_box.min_x));
                                assert!(f32::almost_equals(&1.0, bounding_box.max_x));
                                assert!(f32::almost_equals(&-1.0, bounding_box.min_y));
                                assert!(f32::almost_equals(&3.0, bounding_box.max_y));
                                // Don't need to check Z as all items are in X/Y plane.
                            }
                            BoundingBoxTree::Leaf(Leaf { object }) => {
                                let bbox = object.shape.get_bounding_box();
                                assert!(Point::new(5.0, 0.0, 0.0).almost_equals(bbox.center()));
                            }
                        }
                    }
                    _ => panic!("Expected there to be a node"),
                }
            }
        }
    }
}
