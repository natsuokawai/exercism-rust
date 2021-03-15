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
    if contain(_first_list, _second_list) {
        return Comparison::Superlist;
    }
    if contain(_second_list, _first_list) {
        return Comparison::Sublist;
    }

    Comparison::Unequal
}

// return true if sjb ⊇ obj
fn contain<T: PartialEq>(sbj: &[T], obj: &[T]) -> bool {
    let s_len = sbj.len();
    let o_len = obj.len();
    o_len == 0 || (s_len > o_len && sbj.windows(o_len).any(|w| w == obj))
}
