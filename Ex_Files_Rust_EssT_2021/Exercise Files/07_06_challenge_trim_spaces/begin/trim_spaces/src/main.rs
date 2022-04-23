fn main() {
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");
    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");
    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");
    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");
    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");
    let test6 = "";
    assert_eq!(trim_spaces(test6), "");
    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(test7), "🚀");
    println!("Tests passed!");
}

fn trim_spaces(s: &str) -> &str {
    let start;
    let end;
    let mut c = 0;
    while c < s.len() && s.as_bytes()[c] == b' ' {
        c += 1;
    }
    if c == s.len() {
        return "";
    }
    start = c;
    c = s.len() - 1;
    while s.as_bytes()[c] == b' ' && c > 0 {
        c -= 1;
    }
    end = c + 1;
    &s[start..end]
}
