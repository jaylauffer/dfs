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

/// Post-order DFS without recursion or an auxiliary stack.
/// (⚠️ relies on `unsafe`; tree must stay immutable while walking.)
pub unsafe fn dfs_no_stack<F>(root: &Node, mut visit: F)
where
    F: FnMut(&Node),
{
    let root_ptr = Raw(root as *const _);
    let mut last = Raw(ptr::null());
    let mut prev = Raw(ptr::null());
    let mut curr = Raw(ptr::null());

    // helper reproducing the C++ ternary child selection
    let child = |n: Raw, last: Raw| -> Raw {
        if n.0.is_null() {
            return Raw(ptr::null());
        }
        let node = &*n.0;
        let l = node.left.as_deref().map_or(ptr::null(), |x| x as *const _);
        let r = node.right.as_deref().map_or(ptr::null(), |x| x as *const _);

        if !r.is_null() && r == last.0 {
            Raw(ptr::null())
        } else if l == last.0 {
            Raw(r)
        } else {
            Raw(l)
        }
    };

    while last.0 != root_ptr.0 {
        // pick the node to start the downward sweep
        if curr.0 == prev.0 {
            curr = root_ptr;
        } else {
            curr = prev;
        }

        prev = curr;                     //  ←← **critical refresh**

        // walk as far down as possible
        let mut next = child(curr, last);
        while !next.0.is_null() {
            prev = curr;
            curr = next;
            next = child(curr, last);
        }

        // visit in post-order
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
