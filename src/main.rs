use std::ptr;

/* ---------- tree definition ---------- */
#[derive(Debug)]
pub struct Node {
    pub value:  i32,
    pub left:   Option<Box<Node>>,
    pub right:  Option<Box<Node>>,
}

/* ---------- tiny helper for raw-ptr compares ---------- */
#[derive(Copy, Clone)]
struct Raw(*const Node);
unsafe impl Send for Raw {}
unsafe impl Sync for Raw {}

/* ---------- post-order DFS, zero stack / recursion ---------- *
 * Exactly the logic of your original C++ implementation.       *
 * SAFETY: the tree must not be mutated concurrently.           */
pub unsafe fn dfs_no_stack<F>(root: &Node, mut visit: F)
where
    F: FnMut(&Node),
{
    let root_ptr = Raw(root as *const _);
    let mut last = Raw(ptr::null());
    let mut prev = Raw(ptr::null());
    let mut curr = Raw(ptr::null());

    /* ternary child selector */
    let pick_child = |n: Raw, last: Raw| -> Raw {
        if n.0.is_null() {
            return Raw(ptr::null());
        }
        let nd = &*n.0;

        let l = nd
            .left
            .as_ref()
            .map_or(ptr::null::<Node>(), |b| &**b as *const Node);
        let r = nd
            .right
            .as_ref()
            .map_or(ptr::null::<Node>(), |b| &**b as *const Node);

        if !r.is_null() && r == last.0 {
            Raw(ptr::null())
        } else if l == last.0 {
            Raw(r)
        } else {
            Raw(l)
        }
    };

    /* -------------- main loop -------------- */
    while last.0 != root_ptr.0 {
        curr = if curr.0 == prev.0 { root_ptr } else { prev };

        /* walk down */
        let mut next = pick_child(curr, last);
        while !next.0.is_null() {
            prev = curr;
            curr = next;
            next = pick_child(curr, last);
        }

        /* visit */
        visit(&*curr.0);
        last = curr;

        /* detach visited node from its parent */
        if !prev.0.is_null() {
            let p = &mut *(prev.0 as *mut Node);
            if let Some(ref c) = p.left {
                if &**c as *const _ == curr.0 {
                    p.left = None;
                }
            }
            if let Some(ref c) = p.right {
                if &**c as *const _ == curr.0 {
                    p.right = None;
                }
            }
        }
    }
}

/* ---------- test harness ---------- */
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
                left: Some(Box::new(Node { value: 9, left: None, right: None })),
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
        dfs_no_stack(&tree, |n| print!("{} ", n.value));
    }
    println!();
}
