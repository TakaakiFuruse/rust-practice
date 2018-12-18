fn main() {
    assert_eq!(
        pig_latin(String::from("Welcome")),
        (String::from("Elcomeway"))
    );
    assert_eq!(pig_latin(String::from("trash")), (String::from("ashtray")));
    assert_eq!(pig_latin(String::from("Hello")), (String::from("Ellohay")));
    assert_eq!(pig_latin(String::from("eat")), (String::from("eathay")));
    assert_eq!(
        pig_latin(String::from("omelet")),
        (String::from("omelethay"))
    );
    assert_eq!(pig_latin(String::from("are")), (String::from("arehay")));

    assert_eq!(
        pig_latin(String::from("What is your name")),
        (String::from("Atwhay ishay ouryay amenay"))
    );
    assert_eq!(
        pig_latin(String::from("Where are you from")),
        (String::from("Erewhay arehay ouyay omfray"))
    );
    assert_eq!(
        pig_latin(String::from("Pleased to meet you")),
        (String::from("Easedplay otay eetmay ouyay"))
    );
    assert_eq!(
        pig_latin(String::from("Good luck")),
        (String::from("Oodgay ucklay"))
    );
    assert_eq!(
        pig_latin(String::from("Have a nice day")),
        (String::from("Avehay ahay icenay ayday"))
    );
    assert_eq!(
        pig_latin(String::from("Would you like to dance with me")),
        (String::from("Ouldway ouyay ikelay otay anceday ithway emay"))
    );
}

fn is_vowel(letter: &char) -> bool {
    match &letter {
        'a' => true,
        'e' => true,
        'i' => true,
        'o' => true,
        'u' => true,
        'A' => true,
        'E' => true,
        'I' => true,
        'O' => true,
        'U' => true,
        _ => false,
    }
}

fn capitalize(string: &String) -> String {
    let mut c = string.chars();
    match c.next() {
        None => "".to_string(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn pig_latin(words: String) -> String{
    let mut res = vec![];
    let words_vec: Vec<&str> = words.split(' ').collect();

    for word in &words_vec {
        res.push(pig_latinize_word(word.to_string()));
    }

     return res.join(" ");
}

fn pig_latinize_word(word: String) -> String {
    let index: &usize = &word.find(|c: char| is_vowel(&c)).unwrap();
    let before_the_first_vowel = &word.get(0..*index).unwrap().to_lowercase();
    let after_the_first_vowel = &mut word.get(*index..).unwrap().to_lowercase();

    let first_word = word.chars().next();

    match (index, first_word.unwrap().is_uppercase()) {
        (0, true)  => {
            return format!(
                "{}{}",
                capitalize(&word),
                String::from("hay"),
            );
        },
        (0, false) => {
            return format!(
                "{}{}",
                &word,
                String::from("hay"),
            );
        },
        (&_, true) =>{
            return format!(
                "{}{}{}",
                capitalize(after_the_first_vowel),
                before_the_first_vowel,
                String::from("ay"),
            );
        },
        (&_, false) =>{
            return format!(
                "{}{}{}",
                after_the_first_vowel,
                before_the_first_vowel,
                String::from("ay"),
            );
        },
    }
}
