use std::io;
fn main() {
    
    let vowels = vec!['a','e','i','o','u','A','E','I','O','U',];

    println!("Pig Latin Program.");
    println!("Input a string:");

    let mut sent = String::new();
    io::stdin()
        .read_line(&mut sent)
        .expect("Failed to read line");

    sent = sent + ".";

    let mut new_string = String::new();
    let mut temp_word = String::new();
    let mut temp_char: char = '\0'; 
    let mut counter = 1;
    for c in sent.chars(){
        if c != ' ' && c != '.' {
            if counter == 1 {
                if vowels.contains(&c) {
                    new_string.push(c);
                    temp_char = '1';
                }else{                    
                    temp_char = c;
                }
            }else{
                new_string.push(c);
            }
        }else{
            new_string.push('-');

            if temp_char == '1' {
                new_string.push_str("hay");
            }else{
                new_string.push(temp_char);
                new_string.push_str("ay");
            } 
                 
            if c!= '.' {
                new_string.push(' ');
            }
            counter = 0;
            temp_char = '\0';
            temp_word.clear();
        }
        counter += 1;
    }

    println!("Output: {}", new_string);
}
