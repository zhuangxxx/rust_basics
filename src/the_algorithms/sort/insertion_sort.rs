pub fn insertion_sort(seq: &mut [impl Ord]) {
    for i in 0..seq.len() - 1 {
        let mut j = i + 1;
        while j > 0 && seq[j - 1] > seq[j] {
            seq.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[test]
fn test_insertion_sort() {
    super::test_sort_simple(insertion_sort);
    super::test_sort_complete(insertion_sort, 10, 997);
}
