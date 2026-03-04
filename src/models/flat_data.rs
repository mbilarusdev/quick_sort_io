use super::sortable::Sortable;

#[repr(C)]
pub struct FlatData {
    pub values: *const Sortable,
    pub len: usize,
    pub width: usize,
}
