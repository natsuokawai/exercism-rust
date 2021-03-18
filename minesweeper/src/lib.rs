use std::cmp;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    let row_len = minefield.len();

    if row_len == 0 {
        return res;
    }

    let col_len = minefield[0].len();

    for row in 0..row_len {
        let mut row_str = String::new();
        for col in 0..col_len {
            if minefield[row].chars().nth(col).unwrap() == '*' {
                row_str += "*";
                continue;
            }

            // count mines around empty square
            let mut count = 0;
            let r_start = cmp::max(0, (row as i32) - 1);
            let r_end = cmp::min((row + 1) as i32, (row_len - 1) as i32);
            for r in r_start..=r_end {
                let c_start = cmp::max(0, (col as i32) - 1);
                let c_end = cmp::min((col + 1) as i32, (col_len - 1) as i32);
                for c in c_start..=c_end {
                    if minefield[r as usize].chars().nth(c as usize).unwrap() == '*' {
                        count += 1;
                    }
                }
            }

            if &count.to_string() == "0" {
                row_str += " ";
            } else {
                row_str += &count.to_string();
            }
        }
        res.push(row_str);
    }

    res
}
