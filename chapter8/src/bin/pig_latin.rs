fn main() {
    println!("{}", pig_latin("first"));
}

fn pig_latin(original: &str) -> String {
    let mut s = String::from(original);
    let letter = s.remove(0);
    if is_vowel(letter) {
        let return_string = String::from(original.to_owned() + "-hay");
        return_string
    }
    else {
        let return_string = String::from(s + "-" + letter.to_string().as_str() + "hay");
        return_string
    }
}

fn is_vowel(c: char) -> bool {
    let c = c.to_lowercase().to_string();
    c == "a" || c == "e" || c == "i" || c == "o" || c == "u"
}