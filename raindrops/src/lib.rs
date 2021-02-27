pub fn raindrops(n: u32) -> String {
    let mut s = String::from("");

    if n % 3 == 0 {
        s += "Pling";
    }
    if n % 5 == 0 {
        s += "Plang";
    }
    if n % 7 == 0 {
        s += "Plong";
    }

    if s == String::from("") {
        n.to_string()
    } else {
        s
    }
}
