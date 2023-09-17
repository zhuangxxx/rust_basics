pub fn selection_sort(seq: &mut [impl Ord]) {
    for i in 0..seq.len() - 1 {
        let mut m = i;
        for j in i + 1..seq.len() {
            if seq[j] < seq[m] {
                m = j;
            }
        }
        seq.swap(i, m);
    }
}

#[test]
fn test_selection_sort() {
    super::test_sort_simple(selection_sort);
    super::test_sort_complete(selection_sort, 10, 997);
}
