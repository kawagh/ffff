pub fn update_scores(scores: &mut [i32], names: &[String], text_input: &String) {
    for (i, name) in names.iter().enumerate() {
        scores[i] = character_count_distance_score(name, text_input);
    }
}

pub fn character_count_distance_score(name: &str, input: &String) -> i32 {
    let length_diff = input.len().abs_diff(name.len()) as i32;
    -length_diff
}

#[cfg(test)]
mod tests {
    use crate::scoring::character_count_distance_score;
    #[test]
    fn test_character_count_distance_score() {
        let input = "abc";
        let name = "abcde";
        let expected = -2;
        assert_eq!(
            character_count_distance_score(name, &String::from(input)),
            expected
        );
    }
}
