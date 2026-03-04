pub use crate::models::Sortable;
pub use crate::quick_sort::parallel::quick_sort;
use std::cmp::Ordering;

pub fn sort_by_priority(
    data: Vec<Sortable>,
    mut ids: Vec<u32>,
    width: usize,
    priorities: Vec<usize>,
    directions: Vec<bool>,
    strings_pool: Vec<String>,
) -> Vec<u32> {
    let pool: Vec<&str> = strings_pool.iter().map(|s| s.as_str()).collect();

    quick_sort(&mut ids, |&id_a, &id_b| {
        let start_a = id_a as usize * width;
        let start_b = id_b as usize * width;

        for i in 0..priorities.len() {
            let col_idx = priorities[i];
            let is_asc = directions[i];

            let val_a = &data[start_a + col_idx];
            let val_b = &data[start_b + col_idx];

            let res = val_a.run_cmp(val_b, &pool);

            if res != Ordering::Equal {
                return if is_asc { res } else { res.reverse() };
            }
        }
        Ordering::Equal
    });

    ids
}
