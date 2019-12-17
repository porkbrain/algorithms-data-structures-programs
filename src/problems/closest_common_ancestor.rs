//! # Problem
//! Given a binary tree, find closest common ancestor of two nodes N, M. A node
//! only knows about its two children if it has any. If the two node equal,
//! return any of them.
//!
//! ## Example
//!
//! ```text
//!                                      n1
//!                                     /  \
//!                                    /    \
//!                                   n2     n3
//!                                  / \
//!                                 /   \
//!                                n4   n5
//!                               / \
//!                              /   \
//!                             n6   n7
//! ```
//!
//! Given nodes `n5` and `n6`, the output of a correct algorithm should be `n2`.

use std::rc::Rc;

pub struct Node {
    left: Option<Rc<Node>>,
    right: Option<Rc<Node>>,
}

impl Node {
    pub fn new(left: &Rc<Node>, right: &Rc<Node>) -> Self {
        Node {
            left: Some(Rc::clone(left)),
            right: Some(Rc::clone(right)),
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Node {
            left: None,
            right: None,
        }
    }
}

///
///
///
/// PS: Implemented the whole module in one go and all tests passed first time.
pub fn closest_common_ancestor(root: &Rc<Node>, a: &Rc<Node>, b: &Rc<Node>) -> Option<Rc<Node>> {
    // If either node is equal to the root node, return the root node.
    if Rc::ptr_eq(root, a) || Rc::ptr_eq(root, b) {
        return Some(Rc::clone(root));
    }

    // If the two nodes are equal, return a pointer to any of them.
    if Rc::ptr_eq(a, b) {
        return Some(Rc::clone(a));
    }

    let (a, b) = index_of_two_nodes(a, b, root, 1);
    let mut a = a?;
    let mut b = b?;

    while a != b {
        if a > b {
            a /= 2;
        } else {
            b /= 2;
        }
    }

    const IS_ODD: usize = 0x1;
    let mut ancestor_index = a;
    let mut path: Vec<bool> = Vec::with_capacity((ancestor_index as f64).sqrt() as usize);

    while ancestor_index != 1 {
        path.push(ancestor_index & IS_ODD == 1);
        ancestor_index /= 2;
    }

    path.reverse();

    let mut node = root;
    for is_right in path {
        if is_right {
            node = node.right.as_ref()?;
        } else {
            node = node.left.as_ref()?;
        }
    }

    Some(Rc::clone(&node))
}

fn index_of_two_nodes(
    a: &Rc<Node>,
    b: &Rc<Node>,
    node: &Rc<Node>,
    index: usize,
) -> (Option<usize>, Option<usize>) {
    if Rc::ptr_eq(a, node) {
        let index_b = node
            .left
            .as_ref()
            .map(|ref child| index_of_one_node(b, child, index * 2))
            .unwrap_or_else(|| {
                node.right
                    .as_ref()
                    .and_then(|ref child| index_of_one_node(b, child, index * 2 + 1))
            });

        (Some(index), index_b)
    } else if Rc::ptr_eq(b, node) {
        let index_a = node
            .left
            .as_ref()
            .map(|ref child| index_of_one_node(a, child, index * 2))
            .unwrap_or_else(|| {
                node.right
                    .as_ref()
                    .and_then(|ref child| index_of_one_node(a, child, index * 2 + 1))
            });

        (index_a, Some(index))
    } else {
        let (left_a, left_b) = if let Some(ref child) = node.left {
            index_of_two_nodes(a, b, child, index * 2)
        } else {
            (None, None)
        };

        let (right_a, right_b) = if let Some(ref child) = node.right {
            index_of_two_nodes(a, b, child, index * 2 + 1)
        } else {
            (None, None)
        };

        (left_a.or(right_a), left_b.or(right_b))
    }
}

fn index_of_one_node(target: &Rc<Node>, node: &Rc<Node>, index: usize) -> Option<usize> {
    if Rc::ptr_eq(&target, &node) {
        Some(index)
    } else {
        let index_left = if let Some(ref child) = node.left {
            index_of_one_node(target, child, index * 2)
        } else {
            None
        };

        let index_right = if let Some(ref child) = node.right {
            index_of_one_node(target, child, index * 2 + 1)
        } else {
            None
        };

        index_left.or(index_right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Graph = [Rc<Node>; 16];

    fn graph() -> Graph {
        let mut h: [Rc<Node>; 16] = Default::default();

        h[4] = Rc::new(Node::new(&h[8], &h[9]));
        h[5] = Rc::new(Node::new(&h[10], &h[11]));
        h[6] = Rc::new(Node::new(&h[12], &h[13]));
        h[7] = Rc::new(Node::new(&h[14], &h[15]));

        h[2] = Rc::new(Node::new(&h[4], &h[5]));
        h[3] = Rc::new(Node::new(&h[6], &h[7]));

        h[1] = Rc::new(Node::new(&h[2], &h[3]));

        h
    }

    #[test]
    fn solves_example() {
        let g = graph();

        let ancestor = closest_common_ancestor(&g[1], &g[12], &g[7]);

        assert!(ancestor.is_some());
        assert!(Rc::ptr_eq(&ancestor.unwrap(), &g[3]));
    }

    #[test]
    fn returns_root_if_descendants_of_n2_and_n3() {
        let g = graph();

        let ancestor = closest_common_ancestor(&g[1], &g[13], &g[9]);

        assert!(ancestor.is_some());
        assert!(Rc::ptr_eq(&ancestor.unwrap(), &g[1]));
    }

    #[test]
    fn if_nodes_equal_then_clone_one() {
        let g = graph();

        let ancestor = closest_common_ancestor(&g[1], &g[2], &g[2]);

        assert!(ancestor.is_some());
        assert!(Rc::ptr_eq(&ancestor.unwrap(), &g[2]));
    }

    #[test]
    fn if_first_node_equal_root_then_clone_it() {
        let g = graph();

        let ancestor = closest_common_ancestor(&g[1], &g[1], &g[2]);

        assert!(ancestor.is_some());
        assert!(Rc::ptr_eq(&ancestor.unwrap(), &g[1]));
    }

    #[test]
    fn if_second_node_equal_root_then_clone_it() {
        let g = graph();

        let ancestor = closest_common_ancestor(&g[1], &g[2], &g[1]);

        assert!(ancestor.is_some());
        assert!(Rc::ptr_eq(&ancestor.unwrap(), &g[1]));
    }
}
