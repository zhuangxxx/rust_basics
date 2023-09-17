pub fn bubble_sort(seq: &mut [impl Ord]) {
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 1..seq.len() {
            if seq[i - 1] > seq[i] {
                seq.swap(i - 1, i);
                swapped = true;
            }
        }
    }
}

#[test]
fn test_bubble_sort() {
    super::test_sort_simple(bubble_sort);
    super::test_sort_complete(bubble_sort, 10, 997);
}
