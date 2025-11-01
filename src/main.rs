use std::io::{self, Write};

fn main() {
    // get user input
    print!("Please enter a sentence: ");
    io::stdout().flush().unwrap();

    // create a mutable string in scope
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("User provided invalid input");

    // removing leading and trailing whitespace and newlines
    let s: String = s.trim().to_string();

    // getting the first word from the string of words
    let word = first_word(&s[..]);

    // getting the second word from the string of words
    let second_word = second_word(&s);

    // empty the string making it equal to ""
    // we can't do this because s is borrowed as immutable
    // this will throw compile error as a mutable borrow occurs,
    // within the lifetime of immutable borrowers like word and second_word
    // s.clear();

    // word still has the value of end of the first word here,
    // also the start and end index of the second word are still available,
    // but s no longer has any content
    // that we could meaningfully use with the value of end of the first word,
    // and with the start and end index of the second word,
    // so word in now totally invalid! and so are the start and end indices of the second word
    // the variables are not in sync with the original string

    // check if string is empty
    if !word.is_empty() {
        // print the first word
        println!("The first word: {word}");
    } else {
        println!("The string is empty");
    }

    // check if second word is available
    if !second_word.is_empty() {
        // print the second word
        println!("The second word: {second_word}");
    } else {
        println!("The second word is not available");
    }
}

/// Returns the slice of the first word in a string of words
///
/// # Arguments
///
/// * `s` - A string slice that holds the string of words
///
/// # Returns
///
/// A string slice that holds the first word
///
/// # Examples
///
/// let s = String::from("hello world");
/// let word = first_word(&s[..]);
/// assert_eq!(word, "hello");
///
fn first_word(s: &str) -> &str {
    // convert string to bytes
    let bytes = s.as_bytes();

    // iterate over the bytes in the string
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    s
}

/// Returns the slice of the second word in a string of words
///
/// # Arguments
///
/// * `s` - A string slice that holds the string of words
///
/// # Returns
///
/// A string slice that holds the second word
///
/// # Examples
///
/// let s = String::from("hello world");
/// let word = second_word(&s[..]);
/// assert_eq!(word, "world");
///
fn second_word(s: &str) -> &str {
    // count the word
    let mut word_count: usize = 0;

    // start and end index of the second word
    let (mut start, mut end) = (0usize, 0usize);

    // convert string to bytes
    let bytes = s.as_bytes();

    // iterate over the bytes in the string
    for (i, &item) in bytes.iter().enumerate() {
        // find the start and end of the second word
        match word_count {
            1 => {
                if item == b' ' {
                    end = i;
                    break;
                }
                // if the second word is the last word
                end = i + 1;
            }
            _ => {
                if item == b' ' {
                    start = i + 1;
                    word_count += 1;
                    continue;
                }
            }
        };
    }
    &s[start..end]
}
