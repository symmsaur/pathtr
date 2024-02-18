use crate::bbox::Leaf;
use crate::bbox::Node;
// Geometric ray tracing
use crate::math::*;

use crate::bbox;
use crate::scene;

pub fn shoot_ray<'a>(
    tree: &'a bbox::BoundingBoxTree,
    ray: &Ray,
) -> Option<(&'a scene::Object, Point, Vector, f32, bool)> {
    // FIXME: Allocation!
    let mut lumber_pile = vec![tree];
    let mut closest_intersection: Option<(&'a scene::Object, Point, Vector, f32, bool)> = None;
    //let mut num_checks = 0u32;
    while let Some(tree) = lumber_pile.pop() {
        match tree {
            bbox::BoundingBoxTree::Node(Node {
                bounding_box,
                left,
                right,
            }) => {
                //num_checks += 1;
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
            bbox::BoundingBoxTree::Leaf(Leaf { object }) => {
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
    //println!("Tried to match {} nodes", num_checks);
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
