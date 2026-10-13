/// Unconditionally removes first and last character from a string.
/// Will not check if the first and last character match any condition.
///
/// Unconditional unwrap operator.
///
/// Given the input: "'\\''" a unwrap operation hopes to remove
/// the leading and trailing single quote character.
///
/// The operation: trim_matches('\''); has a downside.
/// It will remove more than a single character from the border
/// of the string. For the example input, the unwrapped string
/// will look like this: "\\". The remaining single quote is removed.
///
/// To provide a valid unwrap operation, this solution has been
/// taken from stack overflow:
/// https://stackoverflow.com/questions/65976432/how-to-remove-first-and-last-character-of-a-string-in-rust
///
pub fn rem_first_and_last(value: &str) -> &str {

    let mut chars = value.chars();
    chars.next();
    chars.next_back();

    chars.as_str()
}

/// Conditionally removes the first and last character from a string.
/// The condition is that both the first and the last character need to match
/// the match_character.
///
/// Removes first and last character from a string and returns the modified string
/// only if the first and last character both match the match_character!
/// Otherwise, the original input string is returned unchanged.
///
/// e.g.
/// "test" and match_character = " -> test
/// "test' and match_character = " -> "test'
pub fn rem_first_and_last_matches(value: &str, match_character: char) -> &str {

    let first_character = value.chars().nth(0);
    if let Some(char) = first_character {
        if char != match_character {
            return value;
        }
    }
    let last_character = value.chars().last();
    if let Some(char) = last_character {
        if char != match_character {
            return value;
        }
    }

    rem_first_and_last(value)
}