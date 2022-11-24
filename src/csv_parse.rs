/*
   Takes in the contents of a CSV file (data: String)
   Outputs a Vec<Vec<T>>, a 2D vector that holds our parsed CSV values

   Procedure
       Create an empty mut Vec<Vec<T>> (2D T vector) called 'entries' to hold our parsed CSV values
       Add an empty Vec<T> to entries

       Create a variable (mut entries_index: i32 or u32) to hold the current index of our current Vec<T> in the entries vector
       Create a variable (mut current_str: String) to hold our current string
       Create a variable (mut backslash_mode: bool) to indicate whether or not 'backslash mode' is enabled (when enabled, add the next character without checking any rules)

       Go through each character in the supplied string 'data':
           If 'backslash_mode' is true:
               Push the current character to 'current_str'
               Set 'backslash_mode' to false
               continue (skip current character)
           Match the input character:
               If ',' or '\n':
                   Take 'current_str', convert it to T, and push it to 'entries[entries_index]'
                   Clear 'current_str'
                   If '\n':
                       Add a new empty Vec<T> to 'entries'
                       Increase 'entries_index' by 1
               If '\':
                   Set 'backslash_mode' to true
               If '\r':
                   Do nothing
               Otherwise:
                   Push the current character to 'current_str'
       Take 'current_str', convert it to T, and push it to 'entries[entries_index]'
       Return 'entries'
*/

use std::fmt;
use std::str::FromStr;

pub fn parse_csv<T: FromStr + fmt::Debug>(data: String) -> Vec<Vec<T>>
where
    <T as FromStr>::Err: fmt::Debug,
{
    let mut entries = Vec::<Vec<T>>::new(); // 2D vector to hold our parsed data
    entries.push(Vec::<T>::new()); // push an empty Vec<T>

    let mut entries_index = 0; // hold current index of our working Vec<T> in 'entries'
    let mut current_str = String::new(); // our current string
    let mut backslash_mode = false; // when true, ignore the rules and just add the current character to the string

    // go through each character
    for c in data.chars() {
        // if we're in backslash mode, just add the character and don't check any rules
        if backslash_mode {
            current_str.push(c);
            backslash_mode = false;

            continue;
        }

        // if we're not in backslash mode
        match c {
            ',' | '\n' => {
                // take the current string, convert it, add it to our Vec<T>, and clear the string
                let parsed = current_str
                    .clone()
                    .parse::<T>()
                    .unwrap_or_else(|_| panic!("Could not parse \"{:?}\" as supplied type T", &current_str));
                entries[entries_index].push(parsed);

                current_str.clear();

                // if it's a new line, create a new working Vec<T> and change the entries_index accordingly
                if c == '\n' {
                    entries.push(Vec::<T>::new());
                    entries_index += 1;
                }
            }

            // if the character is a backslash, enable backslash mode
            '\\' => backslash_mode = true,

            // if it's \r, just do nothing
            '\r' => {}

            // otherwise, just add the character to the string
            _ => current_str.push(c),
        }
    }

    // take the current string, convert it, and add it to our Vec<T>
    let parsed = current_str
        .clone()
        .parse::<T>()
        .unwrap_or_else(|_| panic!("Could not parse \"{:?}\" as supplied type T", &current_str));
    entries[entries_index].push(parsed);

    // return entries
    entries
}
