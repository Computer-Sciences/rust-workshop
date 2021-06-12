fn main() {
    let test1 = "This is test1.        ";
    assert_eq!(trim_space(&test1), "This is test1.");

    let test2 = "        This is test2.";
    assert_eq!(trim_space(&test2), "This is test2.");

    let test3 = "        This is test3.               ";
    assert_eq!(trim_space(&test3), "This is test3.");

    let test4 = "This is test4.";
    assert_eq!(trim_space(&test4), "This is test4.");

    let test5 = "        ";
    assert_eq!(trim_space(&test5), "");

    let test6 = "";
    assert_eq!(trim_space(&test6), "");

    let test7 = "    😸   ";
    assert_eq!(trim_space(&test7), "😸");

    let test8 = "     😸😸 test8.   ";
    assert_eq!(trim_space(&test8), "😸😸 test8.");

    let test9 = "     😸😸 test9. 😸😸   ";
    assert_eq!(trim_space(&test9), "😸😸 test9. 😸😸");

    let test10 = "     😸test10.  ";
    assert_eq!(trim_space_also(&test10), "😸test10.");

    // note: strings in Rust have a trim() method

    let test11 = " 😸This is the last test😸.😸      ";
    let trimmed_message = test11.trim();
    assert_eq!(trimmed_message, "😸This is the last test😸.😸");
}

fn trim_space(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut result = "";

    // trim leading spaces by finding first non-space character
    for (index, &item) in bytes.iter().enumerate() {
        if item != b' ' {
            result = &s[index..];
            break;
        }
    }

    let bytes = result.as_bytes();

    // trim trailing spaces by finding last non-pace character
    for t in bytes.iter().enumerate() {
        let index = t.0;
        let reverse_index = result.len() - index;
        if bytes[reverse_index - 1] != b' ' {
            result = &result[..reverse_index];
            break;
        }
    }

    result
}

fn trim_space_also(s: &str) -> &str {
    let mut start = 0;
    let mut end = 0;

    for (index, character) in s.chars().enumerate() {
        if character != ' ' {
            start = index;
            break;
        }
    }

    for (index, character) in s.chars().rev().enumerate() {
        if character != ' ' {
            end = s.len() - index;
            break;
        }
    }

    &s[start..end]
}
