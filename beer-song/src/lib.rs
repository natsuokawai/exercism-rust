pub fn verse(n: u32) -> String {
    let bottle = |num| { if num == 1 { String::from("1 bottle") } else { format!("{} bottles", num) } };

    match n {
        0 => String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        num => format!("{} of beer on the wall, {} of beer.\nTake one down and pass it around, {} of beer on the wall.\n", bottle(num), bottle(num), bottle(num - 1)),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut res = String::new();
    let mut num = start;

    while num >= end {
        res += &verse(num);
        if num != end {
            res += "\n";
        }
        if num == 0 {
            break;
        }
        num -= 1;
    }
    
    res
}
