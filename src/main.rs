use std::ptr;

/// A plain binary-tree node.
#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub left:  Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

/// Post-order depth-first search, Morris version (no stack, no recursion).
///
/// The tree is *temporarily* threaded along some `.right` links;
/// those links are put back before the function returns.
///
/// # Safety
/// We create raw pointers to `Node` and mutate `.right` links even though
/// the caller handed us only an `&mut Node`.  That is fine **iff**
/// the caller does **not** touch the tree (read or write) while this
/// function is running.
pub unsafe fn dfs_post_morris<F>(root: &mut Node, mut visit: F)
where
    F: FnMut(&Node),
{
    // Dummy node whose left child is the real root.
    let mut dummy = Node {
        value: 0,
        left:  Some(Box::from_raw(root as *mut _)),
        right: None,
    };
    // We will use raw pointers everywhere to stay close to the metal.
    let mut cur: *mut Node = &mut dummy;

    // Helper: reverse the `.right` pointers along [from … to] and return `to`.
    unsafe fn reverse(mut from: *mut Node, mut to: *mut Node) {
        if from == to { return; }
        let mut prev  = ptr::null_mut();
        while from != to {
            let next = (*from).right.as_mut().map_or(ptr::null_mut(), |b| &mut **b);
            (*from).right = if prev.is_null() { None } else { Some(Box::from_raw(prev)) };
            prev = from;
            from = next;
        }
        (*to).right = if prev.is_null() { None } else { Some(Box::from_raw(prev)) };
    }

    // Helper: walk the chain [from … to] (inclusive) **after it was reversed**
    unsafe fn walk_chain<F>(mut from: *mut Node, to: *mut Node, mut f: F)
    where
        F: FnMut(&Node),
    {
        while from != ptr::null_mut() {
            f(&*from);
            if from == to { break; }
            from = (*from).right.as_mut().map_or(ptr::null_mut(), |b| &mut **b);
        }
    }

    // ---------------- main loop ----------------
    while !cur.is_null() {
        if let Some(ref mut left_box) = (*cur).left {
            // 1️⃣ find predecessor
            let mut pred: *mut Node = &mut **left_box;
            while let Some(ref mut r) = (*pred).right {
                if &mut **r as *mut _ == cur { break; }
                pred = &mut **r;
            }
            if (*pred).right.is_none() {
                // 1st visit: thread and go left
                (*pred).right = Some(Box::from_raw(cur));
                cur = &mut **left_box;
            } else {
                // 2nd visit: unthread, output the left-subtree in reverse
                (*pred).right.take();
                let from = &mut **left_box as *mut _;
                let to   = pred;
                reverse(from, to);
                walk_chain(to, from, |n| visit(n));
                reverse(to, from);          // restore pointers
                cur = (*cur).right.as_mut().map_or(ptr::null_mut(), |b| &mut **b);
            }
        } else {
            // No left child – straightforward Morris step
            cur = (*cur).right.as_mut().map_or(ptr::null_mut(), |b| &mut **b);
        }
    }

    // Put the real root back under `dummy` so its ownership is unchanged.
    let _ = dummy.left.take();      // drop the Box we created from `root`
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
    let mut tree = build_test2();          // same builder you posted
    unsafe {
        dfs_post_morris(&mut *tree, |n| println!("{}", n.value));
    }
}
