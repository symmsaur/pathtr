// Geometric ray tracing
use crate::math::*;

use crate::bbox;
use crate::scene;

pub fn shoot_ray<'a>(
    tree: &'a bbox::BoundingBoxTree,
    ray: &Ray,
) -> Option<(&'a scene::Object, Point, Vector, f64, bool)> {
    let mut lumber_pile = vec![tree];
    let mut closest_intersection: Option<(&'a scene::Object, Point, Vector, f64, bool)> = None;
    while let Some(tree) = lumber_pile.pop() {
        //println!("lumber_pile Height: {}", lumber_pile.len());
        match tree {
            bbox::BoundingBoxTree::Node {
                bounding_box,
                left,
                right,
            } => {
                if bounding_box.check_intersects(&ray) {
                    match left {
                        Some(l) => lumber_pile.push(l),
                        None => {}
                    }
                    match right {
                        Some(r) => lumber_pile.push(r),
                        None => {}
                    }
                }
            }
            bbox::BoundingBoxTree::Leaf { object } => {
                let new_intersection = object.shape.intersect(&ray);
                match new_intersection {
                    Some(TraceResult {
                        intersection: p,
                        normal: n,
                        parameter: t,
                        backside: i,
                    }) => match closest_intersection {
                        Some((_, _, _, t_old, _)) => {
                            if t < t_old {
                                closest_intersection = Some((object, p, n, t, i));
                            }
                        }
                        None => {
                            closest_intersection = Some((object, p, n, t, i));
                        }
                    },
                    None => {}
                }
            }
        };
    }
    // if closest_intersection.is_some() {
    //     println!("closest_intersection: {:?}", closest_intersection.is_some());
    // }
    closest_intersection
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test1() {
//         panic!("unimplemented test");
//     }
// }
