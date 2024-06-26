pub mod api;

#[cfg(test)]
mod tests {
    use api::{JlptLevel, API};

    use super::*;

    #[test]
    fn search_kanji_language() {
        let api = API::new();
        let result = api.search_kanji("語".into());

        assert_eq!(result.taught, "grade 2");
        assert_eq!(result.jlpt_level, JlptLevel::N5);
        assert_eq!(result.stroke_count, 14);
        // assert_eq!(result.meaning, "word, speech, language ");
    }
}
