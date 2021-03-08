pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    let mut res = String::new();
    let last = list.len() - 1;

    for i in 0..=last {
        if i != last {
            res += &format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]);
        } else {
            res += &format!("And all for the want of a {}.", list[0]);
        }
    }

    res
}
