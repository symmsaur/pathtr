use crate::bbox::{self, Leaf, Node};

pub fn print_bounding_box_tree_stats(tree: &bbox::BoundingBoxTree) {
    println!("Max depth {}", max_depth(tree));
    println!("Count {}", count(tree));
}

fn max_depth(tree: &bbox::BoundingBoxTree) -> usize {
    max_depth_rec(tree, 0)
}

fn max_depth_rec(tree: &bbox::BoundingBoxTree, depth: usize) -> usize {
    match tree {
        bbox::BoundingBoxTree::Node(Node {
            bounding_box: _,
            left: Some(tree_l),
            right: Some(tree_r),
        }) => std::cmp::max(
            max_depth_rec(tree_l, depth + 1),
            max_depth_rec(tree_r, depth + 1),
        ),
        bbox::BoundingBoxTree::Node(Node {
            bounding_box: _,
            left: Some(tree),
            right: None,
        }) => max_depth_rec(tree, depth + 1),
        bbox::BoundingBoxTree::Leaf(Leaf { object: _ }) => depth + 1,
        _ => {
            panic!("Invalid tree state")
        }
    }
}

fn count(tree: &bbox::BoundingBoxTree) -> usize {
    match tree {
        bbox::BoundingBoxTree::Node(Node {
            bounding_box: _,
            left: Some(tree_l),
            right: Some(tree_r),
        }) => count(tree_l) + count(tree_r),
        bbox::BoundingBoxTree::Node(Node {
            bounding_box: _,
            left: Some(tree),
            right: None,
        }) => count(tree),
        bbox::BoundingBoxTree::Leaf(Leaf { object: _ }) => 1,
        _ => {
            panic!("Invalid tree state")
        }
    }
}
