//! A quick sort

fn main() {
    let mut a: Vec<_> = (0..10_000).map(|x| x % 3).collect();
    quicksort(&mut a);
    for pair in a.windows(2) {
        assert!(pair[0] <= pair[1], "{} <= {}", pair[0], pair[1]);
    }
}

#[inline]
fn quicksort<T: PartialOrd>(a: &mut [T]) {
    if a.len() <= 1 {
        return;
    }
    let pivot = partition(a);
    quicksort(&mut a[..pivot]);
    if pivot < a.len() - 1 {
        quicksort(&mut a[pivot + 1..]);
    }
}

#[inline]
fn partition<T: PartialOrd>(a: &mut [T]) -> usize {
    let mut pivot = a.len() / 2;
    a.swap(0, pivot);
    pivot = 0;
    for i in 1..a.len() {
        if a[i] <= a[0] {
            pivot += 1;
            a.swap(i, pivot);
        }
    }
    a.swap(0, pivot);
    pivot
}
