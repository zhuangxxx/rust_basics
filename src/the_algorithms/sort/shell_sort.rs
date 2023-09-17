pub fn shell_sort(seq: &mut [impl Ord]) {
    let mut g = seq.len() / 3;
    while g > 0 {
        for i in g..seq.len() {
            let mut j = i;
            while j >= g && seq[j - g] > seq[j] {
                seq.swap(j - g, j);
                j -= g;
            }
        }
        g /= if g < 3 { 2 } else { 3 };
    }
}

#[test]
fn test_shell_sort() {
    super::test_sort_simple(shell_sort);
    super::test_sort_complete(shell_sort, 10, 9973);
}
