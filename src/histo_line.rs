

// See https://en.wikipedia.org/wiki/Block_Elements
//static H_CHARS: [char; 9] = [' ', '▏', '▎', '▍', '▌', '▋', '▊', '▉', '█'];
static V_CHARS: [char; 9] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];


/// will panic if counts aren't consistent with the max or
/// if the max is 0.
/// Use full height only if you don't need to keep a margin with
/// le line above
pub fn histo_line(
    counts: &[u64],
    max: u64,
    full_height: bool,
) -> String {
    let mut h = String::new();
    let m = if full_height { 8f32 } else { 7f32 };
    for &c in counts {
        if max == 0 {
            // I lied: it doesn't panic
            h.push(V_CHARS[0]);
        } else {
            let idx = (m * (c as f32) / (max as f32)).round() as usize;
            h.push(V_CHARS[idx]);
        }
    }
    h
}
