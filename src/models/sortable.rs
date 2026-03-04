use std::cmp::Ordering;

#[derive(Clone)]
pub enum Sortable {
    Int(i64),
    Float(f64),
    Bool(bool),
    StrIdx(u32),
    Null,
}

impl Sortable {
    #[inline]
    pub fn run_cmp(&self, other: &Self, strings_pool: &[&str]) -> Ordering {
        match (self, other) {
            (Sortable::Int(a), Sortable::Int(b)) => a.cmp(b),
            (Sortable::Float(a), Sortable::Float(b)) => a.total_cmp(b),
            (Sortable::Bool(a), Sortable::Bool(b)) => a.cmp(b),
            (Sortable::StrIdx(a), Sortable::StrIdx(b)) => {
                if a == b {
                    return Ordering::Equal;
                }

                strings_pool[*a as usize].cmp(&strings_pool[*b as usize])
            }
            (Sortable::Null, Sortable::Null) => std::cmp::Ordering::Equal,
            (Sortable::Null, _) => std::cmp::Ordering::Greater,
            (_, Sortable::Null) => std::cmp::Ordering::Less,
            _ => std::cmp::Ordering::Equal,
        }
    }
}
