pub fn heap_sort(seq: &mut [impl Ord + std::fmt::Debug]) {
    fn heapify(seq: &mut [impl Ord], begin: usize, end: usize) {
        let mut parent = begin;
        let mut child = 2 * begin + 1;
        while child < end {
            if child + 1 < end && seq[child + 1] > seq[child] {
                child += 1;
            }
            if seq[child] > seq[parent] {
                seq.swap(parent, child);
                parent = child;
                child = 2 * child + 1;
            } else {
                break;
            }
        }
    }

    let len = seq.len();
    for begin in (0..=len / 2).rev() {
        heapify(seq, begin, len);
    }
    for end in (0..len).rev() {
        heapify(seq, 0, end + 1);
        seq.swap(0, end);
    }
}

#[test]
fn test_heap_sort() {
    super::test_sort_simple(heap_sort);
    super::test_sort_complete(heap_sort, 10, 9973);
}
