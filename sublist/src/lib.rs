#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    }

    let f_len = _first_list.len();
    let s_len = _second_list.len();

    if f_len == 0
        || (f_len < s_len
            && _second_list
                .windows(f_len)
                .any(|w_list| w_list == _first_list))
    {
        return Comparison::Sublist;
    }

    if s_len == 0
        || (f_len > s_len
            && _first_list
                .windows(s_len)
                .any(|w_list| w_list == _second_list))
    {
        return Comparison::Superlist;
    }

    Comparison::Unequal
}
