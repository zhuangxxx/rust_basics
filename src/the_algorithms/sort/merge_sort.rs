pub fn top_down_merge_sort(seq: &mut [impl Ord + Copy]) {
    fn merge(seq: &mut [impl Ord + Copy], l: usize, m: usize, r: usize) {
        if m - l > 1 {
            merge(seq, l, l + (m - l) / 2, m);
        }
        if r - m > 1 {
            merge(seq, m, m + (r - m) / 2, r);
        }

        let mut tmp = Vec::with_capacity(r - l);
        let (mut i, mut j) = (l, m);
        for _ in l..r {
            if j >= r || (i < m && seq[i] <= seq[j]) {
                tmp.push(seq[i]);
                i += 1;
            } else {
                tmp.push(seq[j]);
                j += 1;
            }
        }

        seq[l..r].copy_from_slice(&tmp);
    }

    merge(seq, 0, seq.len() / 2, seq.len());
}

pub fn bottom_up_merge_sort(seq: &mut [impl Ord + Copy]) {
    let mut len = 1;
    while len < seq.len() {
        for m in (len..seq.len()).step_by(2 * len) {
            let (l, r) = (m - len, std::cmp::min(m + len, seq.len()));
            let mut tmp = Vec::with_capacity(r - l);
            let (mut i, mut j) = (l, m);
            for _ in l..r {
                if j >= r || (i < m && seq[i] <= seq[j]) {
                    tmp.push(seq[i]);
                    i += 1;
                } else {
                    tmp.push(seq[j]);
                    j += 1;
                }
            }
            seq[l..r].copy_from_slice(&tmp);
        }
        len *= 2;
    }
}

#[test]
fn test_top_down_merge_sort() {
    super::test_sort_simple(top_down_merge_sort);
    super::test_sort_complete(top_down_merge_sort, 10, 9973);
}

#[test]
fn test_bottom_up_merge_sort() {
    super::test_sort_simple(bottom_up_merge_sort);
    super::test_sort_complete(bottom_up_merge_sort, 10, 9973);
}
