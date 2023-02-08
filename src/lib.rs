/* Check file difference */

/* Accepts a string with a name.
If the name is "Marco", returns "Polo".
If the name is "any other value", it returns "Marco".
*/

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut chars = s.chars().collect::<Vec<char>>();
    let mut set = std::collections::HashSet::new();
    let mut ans = 0;
    let mut i = 0;
    let mut j = 0;
    while i < chars.len() && j < chars.len() {
        if !set.contains(&chars[j]) {
            set.insert(chars[j]);
            j += 1;
            ans = ans.max(j - i);
        } else {
            set.remove(&chars[i]);
            i += 1;
        }
    }
    ans as i32
}