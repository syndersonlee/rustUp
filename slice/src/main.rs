fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);
    //borrow reference instance is immutable, so cannot revise 's'

    s.clear();
    // s가 clear되었는데, word의 길이가 안 맞음 -> 싱크가 안 맞음

    let s2 = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}

fn first_word(s : &String) -> usize {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_rev(s : &String) -> usize {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    s[..]
}

// In Project, if you want to slice string before change, rust prevent to lose existed lose.
