pub fn get_pig_latin_translation(text: &String) -> String {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let char_vec: Vec<char> = text.to_lowercase().chars().collect();
    let starts_with_vowel = VOWELS.iter().any(|&c| c == char_vec[0]);
    if starts_with_vowel {
        let new_string = format!("{}-{}", &text, "hay");
        return new_string;
    } else {
        let new_string = format!("{}-{}{}", &text[1..], char_vec[0], "ay");
        return new_string;
    }
}