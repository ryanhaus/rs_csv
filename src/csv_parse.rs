/*
    Takes in the contents of a CSV file (data: String)
    Outputs a Vec<Vec<String>>, a 2D string vector that holds our parsed CSV values

    Procedure
        Create an empty mut Vec<Vec<String>> (2D string vector) called 'entries' to hold our parsed CSV values
        Add an empty Vec<String> to entries

        Create a variable (mut entries_index: i32 or u32) to hold the current index of our current Vec<String> in the entries vector
        Create a variable (mut current_str: String) to hold our current string
        Create a variable (mut backslash_mode: bool) to indicate whether or not 'backslash mode' is enabled (when enabled, add the next character without checking any rules)

        Go through each character in the supplied string 'data':
            If 'backslash_mode' is true:
                Push the current character to 'current_str'
                Set 'backslash_mode' to false
                continue (skip current character)
            Match the input character:
                If ',' or '\n':
                    Take 'current_str' and push it to 'entries[entries_index]'
                    Clear 'current_str'
                    If '\n':
                        Add a new empty Vec<String> to 'entries'
                        Increase 'entries_index' by 1
                If '\':
                    Set 'backslash_mode' to true
                If '\r':
                    Do nothing
                Otherwise:
                    Push the current character to 'current_str'
        Return 'entries'
 */

pub fn parse_csv(data: String) -> Vec<Vec<String>> {
    let mut entries = Vec::<Vec<String>>::new(); // 2D string vector to hold our parsed data
    entries.push(Vec::<String>::new()); // push an empty Vec<String>

    let mut entries_index = 0; // hold current index of our working Vec<String> in 'entries'
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
                // take the current string, add it to our Vec<String>, and clear the string
                entries[entries_index].push(current_str.clone());
                current_str.clear();

                // if it's a new line, create a new working Vec<String> and change the entries_index accordingly
                if c == '\n' {
                    entries.push(Vec::<String>::new());
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
    
    entries
}
