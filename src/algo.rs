/// Computes the [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance)
/// between two strings.
///
/// The Levenshtein distance is a metric for measuring the difference between two
/// sequences by counting the minimum number of single-character edits
/// (insertions, deletions or substitutions) required to transform one string into the other.
///
/// # Performance
///
/// - If both input strings are ASCII, an optimized byte-based implementation is used.
/// - Otherwise, the strings are compared as Unicode scalar values (`char`).
///
/// # Examples
///
/// ```
/// use mylib::levenshtein_distance;
///
/// assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
/// assert_eq!(levenshtein_distance("flaw", "lawn"), 2);
/// assert_eq!(levenshtein_distance("", "abc"), 3);
/// assert_eq!(levenshtein_distance("same", "same"), 0);
/// ```
pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    if a == b {
        return 0;
    }
    if a.is_empty() {
        return b.chars().count();
    }
    if b.is_empty() {
        return a.chars().count();
    }

    if a.is_ascii() && b.is_ascii() {
        return lev_bytes(a.as_bytes(), b.as_bytes());
    }

    let ac: Vec<char> = a.chars().collect();
    let bc: Vec<char> = b.chars().collect();
    lev_chars(&ac, &bc)
}

/// Internal helper for computing Levenshtein distance on ASCII byte slices.
fn lev_bytes(a: &[u8], b: &[u8]) -> usize {
    let (n, m) = (a.len(), b.len());
    let mut prev: Vec<usize> = (0..=m).collect();
    let mut curr: Vec<usize> = vec![0; m + 1];

    for i in 1..=n {
        curr[0] = i;
        let ai = a[i - 1];
        for j in 1..=m {
            let cost = usize::from(ai != b[j - 1]);
            let del = prev[j] + 1;
            let ins = curr[j - 1] + 1;
            let sub = prev[j - 1] + cost;
            curr[j] = del.min(ins).min(sub);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[m]
}

/// Internal helper for computing Levenshtein distance on Unicode scalar values.
fn lev_chars(a: &[char], b: &[char]) -> usize {
    let (n, m) = (a.len(), b.len());
    let mut prev: Vec<usize> = (0..=m).collect();
    let mut curr: Vec<usize> = vec![0; m + 1];

    for i in 1..=n {
        curr[0] = i;
        let ai = a[i - 1];
        for j in 1..=m {
            let cost = usize::from(ai != b[j - 1]);
            let del = prev[j] + 1;
            let ins = curr[j - 1] + 1;
            let sub = prev[j - 1] + cost;
            curr[j] = del.min(ins).min(sub);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[m]
}
