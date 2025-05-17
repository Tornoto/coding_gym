/// https://leetcode.cn/problems/minimum-window-substring/description/
///
/// This solution is a common and efficient way to solve the "Minimum Window Substring" problem, especially when the character set is limited (like ASCII, which `[0; 128]` implies). Let's break down why it works:
///
/// **Core Idea: Sliding Window with Character Counts**
///
/// The algorithm uses a sliding window defined by `start` and `end` pointers. It maintains counts of characters to determine if the current window is valid (contains all characters from `t`).
///
/// **Variables:**
///
/// 1.  **`s_bytes`, `t_bytes`**: Converts strings to byte slices for efficient indexing. This implicitly assumes ASCII characters or that UTF-8 characters relevant to `t` fit within single bytes that map to indices 0-127.
/// 2.  **`map = [0; 128]`**: This is a frequency map.
///     *   **Initialization Phase**: `for &c in t_bytes { map[c as usize] += 1; }`
///         *   For each character `c` in `t`, `map[c]` is incremented.
///         *   So, `map[c]` initially stores the **required count** of character `c` from string `t`.
///     *   **During Window Sliding**:
///         *   When a character `s_bytes[end]` enters the window: `map[s_bytes[end] as usize] -= 1;`
///             *   The value `map[char]` now represents:
///                 *   If `> 0`: We still need more of this `char` to satisfy `t`.
///                 *   If `== 0`: We have exactly enough of this `char` to satisfy `t`'s requirement for it.
///                 *   If `< 0`: We have an excess of this `char` in the window (either it's not in `t`, or we have more than `t` needs).
///         *   When a character `s_bytes[start]` leaves the window: `map[s_bytes[start] as usize] += 1;`
///             *   This effectively "returns" the character. If `map[char]` becomes `> 0` after this, it means this character is now needed again for the window to be valid.
///
/// 3.  **`count = t.len()`**: This is the **total number of characters from `t` that we still need to find in our window**.
///     *   It's not the count of unique characters, but the sum of frequencies of all characters in `t`.
///     *   Each time we encounter a character `s_bytes[end]` that is *still needed* by `t` (i.e., `map[s_bytes[end] as usize] > 0` *before* we decrement it for adding to the window), we decrement `count`. This means one less character from `t` is needed.
///     *   When `count == 0`, it signifies that the current window `s[start..=end]` contains at least as many occurrences of each character as required by `t`.
///
/// 4.  **`start`, `end`**: Define the sliding window. `end` expands the window, `start` contracts it.
///
/// 5.  **`min_start`, `min_len`**: Track the starting position and length of the smallest valid window found so far.
///
/// **Algorithm Flow:**
///
/// 1.  **Initialization:**
///     *   Populate `map` with the character counts from `t`.
///     *   Set `count = t.len()`.
///
/// 2.  **Expand Window (Outer `for end ...` loop):**
///     *   The `end` pointer iterates through `s`.
///     *   `char_at_end = s_bytes[end]`.
///     *   `if map[char_at_end as usize] > 0`: This checks if `char_at_end` is one of the characters we are *still looking for* to satisfy `t`. If `map[char_at_end]` was already `0` or negative, it means we either have enough of it, or it's not in `t`, so adding another one doesn't help satisfy a *remaining* requirement for `count`.
///         *   `count -= 1;`: Decrement the total count of needed characters.
///     *   `map[char_at_end as usize] -= 1;`: Mark that we've "used" one `char_at_end`. (As explained above, `map` value reflects surplus/deficit).
///
/// 3.  **Contract Window (Inner `while count == 0 ...` loop):**
///     *   This loop runs if `count == 0`, meaning the current window `s[start..=end]` is a **candidate solution** because it contains all necessary characters from `t`.
///     *   **Update Minimum:** If this window is smaller than `min_len`, update `min_len` and `min_start`.
///     *   **Try to Shrink:** Now, we try to shrink the window from the `start` to see if we can find an even smaller valid window.
///         *   `char_at_start = s_bytes[start]`.
///         *   `map[char_at_start as usize] += 1;`: We are about to remove `char_at_start` from the window. Incrementing its count in `map` is like "giving it back" or "making it available again."
///         *   `if map[char_at_start as usize] > 0`: This is the crucial part. After incrementing `map[char_at_start]`, if its value becomes positive (`>0`), it means that this specific `char_at_start` was *essential* to keep `count` at `0`. Before this increment, `map[char_at_start]` must have been `0` (meaning the window had *exactly* enough of `char_at_start`). By removing it, the window no longer satisfies `t`'s requirement for `char_at_start`.
///             *   `count += 1;`: So, we increment `count`, indicating the window is no longer fully valid. This will break the `while` loop.
///         *   If `map[char_at_start as usize]` is still `<= 0` after incrementing, it means `char_at_start` was an excess character (either not in `t`, or `t` needed fewer of them than were present even after removing one). So, removing it doesn't invalidate the window with respect to `t`'s requirements, and `count` remains `0`. The `while` loop continues to shrink.
///         *   `start += 1;`: Slide the `start` pointer.
///
/// 4.  **Result:**
///     *   If `min_len` is still `usize::MAX`, no valid window was found.
///     *   Otherwise, extract the substring `s[min_start .. min_start + min_len]`.
///
/// **Why this approach is effective:**
///
/// *   **Efficient Counting:** The `map` array and the single `count` variable provide a very efficient way to track whether the current window satisfies the requirements of `t`.
/// *   **O(N) Time Complexity:** Both `start` and `end` pointers traverse the string `s` at most once. Array lookups/updates are O(1). So, the overall time complexity is O(|S| + |T|) (for initializing `map`).
/// *   **O(1) Extra Space (for fixed alphabet):** The `map` has a fixed size (128 for ASCII). If the alphabet were larger or Unicode, a `HashMap` would be used, making space O(AlphabetSize).
///
/// This solution cleverly uses the `map` values to represent not just frequencies but also the "deficit" or "surplus" of characters, and the `count` variable to quickly check for overall window validity.
pub fn min_window(s: String, t: String) -> String {
    if s.len() < t.len() {
        return String::new();
    }

    let s_bytes = s.as_bytes();
    let t_bytes = t.as_bytes();
    // map initialized to represent the frequency of ch in
    let mut map = [0; 128];
    // This is the total number of characters from t that we still need to find in our window.
    let mut count = t.len();
    let mut start = 0;
    let mut min_start = 0;
    let mut min_len = usize::MAX;

    for &c in t_bytes {
        map[c as usize] += 1;
    }

    for end in 0..s.len() {
        // when a character enters the window
        // if map[ch] > 0 represents we need more ch to satisfy t
        // if map[ch] == 0 represents we have enough ch
        // if map[ch] < 0 represents we have an excess of ch
        if map[s_bytes[end] as usize] > 0 {
            // we only reduce the count needed to satisfy t
            count -= 1;
        }
        map[s_bytes[end] as usize] -= 1;

        // when a leave the window
        while count == 0 {
            // update the min_len and min_start
            if end - start + 1 > min_len {
                min_start = start;
                min_len = end - start + 1;
            }

            // return the ch to map
            map[s_bytes[start] as usize] += 1;
            // if map[ch] > 0 represents we need this ch to make the window valid
            if map[s_bytes[start] as usize] > 0 {
                count += 1;
            }
            start += 1;
        }
    }

    if min_len == usize::MAX {
        String::new()
    } else {
        s[min_start..min_start + min_len].to_string()
    }
}

#[test]
fn test() {
    let s = "abdbadddcdadbb".to_string();
    let t = "abcb".to_string();
    let result = min_window(s, t);
    println!("result: {}", result);

    let s = "a".to_string();
    let t = "aa".to_string();
    let result = min_window(s, t);
    println!("result: {}", result);
}
