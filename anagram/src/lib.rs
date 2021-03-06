use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut res: HashSet<&'a str> = HashSet::new();
    let left = word.to_lowercase();
    for s in possible_anagrams {
        let rignt = s.to_lowercase();
        if left != rignt && compare_chars(&left, &rignt) {
            res.insert(s);
        }
    }
    res
}

fn compare_chars(left: &str, right: &str) -> bool {
    let mut l = left.chars().collect::<Vec<_>>();
    let mut r = right.chars().collect::<Vec<_>>();
    l.sort();
    r.sort();

    if l == r {
        return true;
    }

    false
}
