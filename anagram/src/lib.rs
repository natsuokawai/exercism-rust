use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut res: HashSet<&'a str> = HashSet::new();
    let word_lower = word.to_lowercase();
    let word_sorted = sort_char(&word_lower);

    for pa in possible_anagrams {
        let pa_lower = pa.to_lowercase();
        if pa.len() == word_lower.len()
            && pa_lower != word_lower
            && word_sorted == sort_char(&pa_lower)
        {
            res.insert(pa);
        }
    }
    res
}

fn sort_char(c: &str) -> Vec<char> {
    let mut res: Vec<char> = c.chars().collect();
    res.sort_unstable();
    res
}
