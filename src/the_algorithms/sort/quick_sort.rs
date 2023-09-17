use rand::Rng;

pub fn quick_sort(seq: &mut [impl Ord + Copy]) {
    fn partition(seq: &mut [impl Ord + Copy], begin: usize, end: usize) -> usize {
        let pivot = seq[begin];
        let mut index = begin + 1;
        for i in begin + 1..end {
            if seq[i] < pivot {
                seq.swap(i, index);
                index += 1;
            }
        }
        seq.swap(begin, index - 1);

        index - 1
    }
    fn sort(seq: &mut [impl Ord + Copy], begin: usize, end: usize) {
        if begin < end {
            let m = partition(seq, begin, end);
            sort(seq, begin, m);
            sort(seq, m + 1, end);
        }
    }

    sort(seq, 0, seq.len());
}

pub fn random_pivot_quick_sort(seq: &mut [impl Ord + Copy]) {
    fn partition(seq: &mut [impl Ord + Copy], begin: usize, end: usize) -> usize {
        seq.swap(begin, rand::thread_rng().gen_range(begin..end));
        let pivot = seq[begin];
        let mut index = begin + 1;
        for i in begin + 1..end {
            if seq[i] < pivot {
                seq.swap(i, index);
                index += 1;
            }
        }
        seq.swap(begin, index - 1);

        index - 1
    }
    fn sort(seq: &mut [impl Ord + Copy], begin: usize, end: usize) {
        if begin < end {
            let m = partition(seq, begin, end);
            sort(seq, begin, m);
            sort(seq, m + 1, end);
        }
    }

    sort(seq, 0, seq.len());
}

pub fn dual_pivot_quick_sort(seq: &mut [impl Ord + Copy]) {
    fn partition(seq: &mut [impl Ord + Copy], begin: usize, end: usize) -> (usize, usize) {
        if seq[begin] > seq[end] {
            seq.swap(begin, end);
        }
        let (less_pivot, great_pivot) = (seq[begin], seq[end]);
        let (mut l, mut r) = (begin + 1, end - 1);
        let mut i = begin;
        while l <= r {
            if seq[l] < less_pivot {
                seq.swap(l, i);
                l += 1;
                i += 1;
            } else if seq[l] >= great_pivot {
                while l < r && seq[r] > great_pivot {
                    r -= 1;
                }
                seq.swap(l, r);
                r -= 1;
            } else {
                l += 1;
            }
        }
        r += 1;
        seq.swap(begin, i);
        seq.swap(r, end);

        (l, r)
    }
    fn sort(seq: &mut [impl Ord + Copy], begin: usize, end: usize) {
        if begin < end {
            let (l, r) = partition(seq, begin, end);
            sort(seq, begin, l - 1);
            sort(seq, l + 1, r - 1);
            sort(seq, r + 1, end);
        }
    }

    sort(seq, 0, seq.len() - 1);
}

#[test]
fn test_quick_sort() {
    super::test_sort_simple(quick_sort);
    super::test_sort_complete(quick_sort, 10, 9973);
}

#[test]
fn test_random_pivot_quick_sort() {
    super::test_sort_simple(random_pivot_quick_sort);
    super::test_sort_complete(random_pivot_quick_sort, 10, 9973);
}

#[test]
fn test_dual_pivot_quick_sort() {
    super::test_sort_simple(dual_pivot_quick_sort);
    super::test_sort_complete(dual_pivot_quick_sort, 10, 9973);
}

#[test]
fn test_quick_sort_worst_case() {
    {
        let mut seq = (1..=9973).collect::<Vec<_>>();
        let time = std::time::SystemTime::now();
        quick_sort(&mut seq);
        println!(
            "quick_sort worst case used: {} ms",
            time.elapsed().unwrap().as_millis()
        );
    }
    {
        let mut seq = (1..=9973).collect::<Vec<_>>();
        let time = std::time::SystemTime::now();
        random_pivot_quick_sort(&mut seq);
        println!(
            "random_pivot_quick_sort worst case used: {} ms",
            time.elapsed().unwrap().as_millis()
        );
    }
    {
        let mut seq = (1..=9973).collect::<Vec<_>>();
        let time = std::time::SystemTime::now();
        dual_pivot_quick_sort(&mut seq);
        println!(
            "dual_pivot_quick_sort worst case used: {} ms",
            time.elapsed().unwrap().as_millis()
        );
    }
}
