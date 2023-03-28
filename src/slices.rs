fn main() {
    let mut s = String::from("Hellojohn World");
    let word = first_word(&s);

    println!("{}", s);
    println!("{}", word);

    let arr = [1, 2, 3, 4, 5, 6, 7];
    let slice = &arr[..4];

    for i in slice {
        println!("{}", i);
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
