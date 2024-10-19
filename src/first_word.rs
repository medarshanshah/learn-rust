fn main() {
    let sentence: String = String::from("Darshan is my name");
    let first_word: String = get_first_word(sentence);

    let n:i32 = 1000;
    for i in 0..n{
        println!("Hello world! {}", i);
    }

    print!("First word is {}", first_word)
}

fn get_first_word(sentence: String) -> String {
    let ans: String = String::from("");
    for char: char in sentence.chars() {
        ans.push_str(string: char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return ans;
}