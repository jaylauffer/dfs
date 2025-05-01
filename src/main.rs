use std::ptr;

#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub left:  Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

/// Helper that lets us compare / copy raw pointers without sprinkling `*const _` everywhere.
#[derive(Copy, Clone)]
struct Raw(*const Node);
unsafe impl Send for Raw {}
unsafe impl Sync for Raw {}

/// Post-order DFS **without** a stack / recursion – identical to the original C++.
pub unsafe fn dfs_no_stack<F>(root: &Node, mut visit: F)
where
    F: FnMut(&Node),
{
    let root_ptr   = Raw(root as *const _);
    let mut last   = Raw(ptr::null());
    let mut prev   = Raw(ptr::null());
    let mut curr   = Raw(ptr::null());
    let mut next: Raw;

    // --- helper closure: given `curr` and `last`, pick the next child exactly as in the C++ ternary ---
    let mut child = |curr: Raw, last: Raw| -> Raw {
        if curr.0.is_null() {
            return Raw(ptr::null());
        }
        let n = &*curr.0;
        match (&n.left, &n.right) {
            (Some(l), Some(r)) => {
                if (&**r as *const _) == last.0 {
                    Raw(ptr::null())
                } else if (&**l as *const _) == last.0 {
                    Raw(&**r)
                } else {
                    Raw(&**l)
                }
            }
            (Some(l), None) => {
                if (&**l as *const _) == last.0 { Raw(ptr::null()) } else { Raw(&**l) }
            }
            (None, Some(r)) => {
                if (&**r as *const _) == last.0 { Raw(ptr::null()) } else { Raw(&**r) }
            }
            (None, None) => Raw(ptr::null()),
        }
    };

    // ---------------- main loop (verbatim logic of the C++ original) ----------------
    while last.0 != root_ptr.0 {
        if curr.0 == prev.0 {
            curr = root_ptr;
        } else {
            curr = prev;
        }

        next = child(curr, last);

        // descend as far as possible
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
