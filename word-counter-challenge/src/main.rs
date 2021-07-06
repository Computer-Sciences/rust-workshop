use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs;

#[derive(Debug)]
struct MostCommonWord {
    value: String,
    occurrence: u32,
}

impl fmt::Display for MostCommonWord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\"{}\" is the most common word with {} occurrences",
            self.value, self.occurrence
        )
    }
}
impl MostCommonWord {
    fn new(value: &str, occurrence: u32) -> MostCommonWord {
        MostCommonWord {
            value: value.to_string(),
            occurrence,
        }
    }
}

fn main() {
    if env::args().len() != 2 {
        println!("this program requires one argument");
        return;
    }

    let filepath = match env::args().nth(1) {
        Some(text) => text,
        None => panic!("Failed to get file path"),
    };

    // println!("filepath is {}", filepath);

    let file_content = match fs::read_to_string(filepath) {
        Ok(content) => content,
        Err(e) => panic!("Err: {}", e),
    };

    // println!("file_content is:\n {}", file_content);

    let mut word_count: HashMap<String, u32> = HashMap::new();

    for word in file_content.split_whitespace() {
        let lowercase_word = word.to_lowercase();

        match word_count.get(&lowercase_word) {
            Some(count) => {
                let increment = *count + 1;
                word_count.insert(lowercase_word, increment);
            }
            None => {
                word_count.insert(lowercase_word, 1);
            }
        };
    }

    let mut most_common = MostCommonWord::new("", 0);
    let mut most_common_words = vec![];

    for (word, count) in &word_count {
        if most_common.occurrence < *count {
            most_common.value = word.to_string();
            most_common.occurrence = *count;
        }
    }

    most_common_words.push(most_common);

    for (word, count) in &word_count {
        if most_common_words[0].value != word.to_string()
            && most_common_words[0].occurrence == *count
        {
            let other_most_common = MostCommonWord::new(word, *count);
            most_common_words.push(other_most_common);
        }
    }

    if most_common_words.len() > 1 {
        println!("The most common words in the text are;");
        for most_common in most_common_words {
            println!(
                "\"{}\" with {} occurrences",
                most_common.value, most_common.occurrence
            );
        }
    } else if most_common_words.len() == 1 {
        println!("{}", most_common_words[0]);
    }
}

// alternative

// use std::env;
// use std::fs;
// use std::collections::HashMap;

// fn main() {
//     // read file and build vector of individual words
//     let contents = match env::args().nth(1) {
//         Some(f) => match fs::read_to_string(f) {
//             Ok(s) => s.to_lowercase(),
//             Err(e) => {
//                 eprintln!("Could not read file: {}", e);
//                 std::process::exit(1);
//             }
//         },
//         None => {
//             eprintln!("Program requires an argument: <file path>");
//             std::process::exit(2);
//         }
//     };
//     let all_words = contents.split_whitespace().collect::<Vec<&str>>();

//     // count how many times each unique word occurs
//     let mut word_counts: HashMap<&str, u32> = HashMap::new();
//     for word in all_words.iter() {
//         *word_counts.entry(word).or_insert(1) += 1;
//     }
//     // determine the most commonly used word(s)
//     let mut top_count = 0u32;
//     let mut top_words: Vec<&str> = Vec::new();
//     for (&key, &val) in word_counts.iter() {
//         if val > top_count {
//             top_count = val;
//             top_words.clear();
//             top_words.push(key);
//         } else if val == top_count {
//             top_words.push(key);
//         }
//     }

//     // display results
//     println!("Top word(s) occurred {} times:", top_count);
//     for word in top_words.iter() {
//         println!("{}", word);
//     }
// }
