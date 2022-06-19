//! h.rs --- heap
use crate::o::UpValue;
const GC_THRESHOLD: usize = 100;
const GC_GROW_FACTOR: usize = 2;
const _RESET_LIMIT: usize = 5;

#[inline(always)]
fn mark_upvalue(upvalue: &Rc<RefCell<UpValue>>) {
    {
        upvalue.borrow_mut().mark_reachable();
    }

    if let Some(inner) = upvalue.borrow().get_value_if_closed() {
        traverse(inner);
    }
}
