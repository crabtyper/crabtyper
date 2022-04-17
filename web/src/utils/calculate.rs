pub fn calculate_accuracy(correct: &str, remaining: &str, mistakes: u8) -> u8 {
    let text = format!("{}{}", correct, remaining);
    let tabs = text.chars().filter(|c| c == &'\t').count();
    let char_count = (text.len() + tabs) as f32;

    if mistakes > 0 {
        (100.0 - ((mistakes as f32 / char_count) * 100.0)) as u8
    } else {
        100
    }
}

pub fn calculate_progress(correct: &str, remaining: &str) -> u8 {
    let index = correct.len() as f32;
    let max = (format!("{}{}", correct, remaining).len() + 1) as f32;

    ((index / max) * 100.0).floor() as u8
}

pub fn calculate_wpm(time: usize, correct: &str) -> u8 {
    let minutes_past = time as f32 / 60.0;
    let index = correct.len() as f32;

    if minutes_past > 0.0 {
        ((index / 5.0) / minutes_past).floor() as u8
    } else {
        0_u8
    }
}
