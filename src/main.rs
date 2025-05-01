use std::ptr;

#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub left:  Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

#[derive(Copy, Clone)]
struct Raw(*const Node);
unsafe impl Send for Raw {}
unsafe impl Sync for Raw {}

/// Post-order DFS with no recursion / stack.
/// SAFETY: the tree must not be mutated while this runs.
pub unsafe fn dfs_no_stack<F>(root: &Node, mut visit: F)
where
    F: FnMut(&Node),
{
    let root_ptr = Raw(root as *const _);
    let mut last = Raw(ptr::null());
    let mut prev = Raw(ptr::null());
    let mut curr = Raw(ptr::null());

    // child-selection helper = C++ ternary
    let child = |n: Raw, last: Raw| -> Raw {
        if n.0.is_null() {
            return Raw(ptr::null());
        }
        let node = &*n.0;

        let l: *const Node = node
            .left
            .as_deref()
            .map_or(ptr::null::<Node>(), |x| x as *const Node);
        let r: *const Node = node
            .right
            .as_deref()
            .map_or(ptr::null::<Node>(), |x| x as *const Node);

        if !r.is_null() && r == last.0 {
            Raw(ptr::null())
        } else if l == last.0 {
            Raw(r)
        } else {
            Raw(l)
        }
    };

    while last.0 != root_ptr.0 {
        curr = if curr.0 == prev.0 { root_ptr } else { prev };
        prev = curr;                       // keep prev one step behind

        let mut next = child(curr, last);
        while !next.0.is_null() {
            prev = curr;
            curr = next;
            next = child(curr, last);
        }

        visit(&*curr.0);
        last = curr;
    }
}

fn build_test2() -> Box<Node> {
    Box::new(Node {
        value: 1,
        left: Some(Box::new(Node {
            value: 2,
            left: Some(Box::new(Node {
                value: 4,
                left: Some(Box::new(Node { value: 8, left: None, right: None })),
                right: None,
            })),
            right: Some(Box::new(Node {
                value: 5,
                left: Some(Box::new(Node { value: 9,  left: None, right: None })),
                right: Some(Box::new(Node {
                    value: 10,
                    left: Some(Box::new(Node { value: 11, left: None, right: None })),
                    right: Some(Box::new(Node { value: 12, left: None, right: None })),
                })),
            })),
        })),
        right: Some(Box::new(Node {
            value: 3,
            left: Some(Box::new(Node { value: 6, left: None, right: None })),
            right: Some(Box::new(Node { value: 7, left: None, right: None })),
        })),
    })
}

fn main() {
    let tree = build_test2();
    unsafe {
        dfs_no_stack(&tree, |n| println!("{}", n.value));
    }
}
