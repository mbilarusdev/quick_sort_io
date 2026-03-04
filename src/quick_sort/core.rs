use std::cmp::Ordering;

pub fn partition<T, F>(arr: &mut [T], compare: F) -> (usize, usize)
where
    F: Fn(&T, &T) -> Ordering,
{
    let last = arr.len() - 1;
    let mid = arr.len() / 2;
    arr.swap(mid, last);

    let mut lt = 0;
    let mut i = 0;
    let mut gt = last - 1;

    while i <= gt {
        match compare(&arr[i], &arr[last]) {
            Ordering::Less => {
                arr.swap(lt, i);
                lt += 1;
                i += 1;
            }
            Ordering::Greater => {
                arr.swap(i, gt);
                if gt == 0 {
                    break;
                }
                gt -= 1;
            }
            Ordering::Equal => {
                i += 1;
            }
        }
    }

    arr.swap(i, last);

    (lt, i)
}
