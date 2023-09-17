pub mod bubble_sort;
pub mod heap_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod quick_sort;
pub mod selection_sort;
pub mod shell_sort;

pub fn is_sorted(seq: &[impl Ord]) -> bool {
    seq.windows(2).all(|w| w[0] <= w[1])
}

pub fn test_sort_simple(sort_fn: fn(&mut [i32])) {
    let mut seq = vec![12, 5, 3, 9, 7, 11, 2, 8, 10, 0, 4, 1, 6];
    sort_fn(&mut seq);
    assert!(is_sorted(&seq));
}

pub fn test_sort_complete(sort_fn: fn(&mut [i32]), num: u8, len: usize) {
    for n in 1..=num {
        let mut seq = (0..len).map(|_| rand::random::<i32>()).collect::<Vec<_>>();
        let time = std::time::SystemTime::now();
        sort_fn(&mut seq);
        println!(
            "No.{n} round used: {} ms",
            time.elapsed().unwrap().as_millis()
        );
        assert!(is_sorted(&seq));
    }
}
