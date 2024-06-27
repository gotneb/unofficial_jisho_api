pub mod api;

#[cfg(test)]
mod tests {
    use api::{JlptLevel, JishoAPI};

    use super::*;

    #[test]
    fn search_language_kanji() {
        let api = JishoAPI::new();
        let result = api.search_kanji("語".into());

        assert_eq!(result.taught, "grade 2");
        assert_eq!(result.jlpt_level, JlptLevel::N5);
        assert_eq!(result.stroke_count, 14);
        assert_eq!(result.meaning, "word, speech, language");
        assert_eq!(result.kunyomi, vec!["かた.る", "かた.らう"]);
        assert_eq!(result.onyomi, vec!["ゴ"]);
        assert_eq!(result.parts, vec!["口", "五", "言"]);

        // Now it must test the kunyomis examples
        // -------------------------------------------
        assert_eq!(result.kunyomi_examples[0].kanji, "語る");
        assert_eq!(result.kunyomi_examples[0].hiragana, "かたる");
        assert_eq!(result.kunyomi_examples[0].meaning, "to talk about, to speak of, to tell, to narrate, to recite, to chant, to indicate, to show");

        assert_eq!(result.kunyomi_examples[1].kanji, "語るに落ちる");
        assert_eq!(result.kunyomi_examples[1].hiragana, "かたるにおちる");
        assert_eq!(result.kunyomi_examples[1].meaning, "to let slip a secret, to let the cat out of the bag");

        assert_eq!(result.kunyomi_examples[2].kanji, "語らう");
        assert_eq!(result.kunyomi_examples[2].hiragana, "かたらう");
        assert_eq!(result.kunyomi_examples[2].meaning, "to talk, to tell, to recite, to pledge, to conspire with");

        // Now it must test the onyomis examples
        // -------------------------------------------
        assert_eq!(result.onyomi_examples[0].kanji, "語");
        assert_eq!(result.onyomi_examples[0].hiragana, "ゴ");
        assert_eq!(result.onyomi_examples[0].meaning, "word, language, speech");

        assert_eq!(result.onyomi_examples[1].kanji, "語学");
        assert_eq!(result.onyomi_examples[1].hiragana, "ゴガク");
        assert_eq!(result.onyomi_examples[1].meaning, "study of foreign languages, linguistics");

        assert_eq!(result.onyomi_examples[2].kanji, "標語");
        assert_eq!(result.onyomi_examples[2].hiragana, "ヒョウゴ");
        assert_eq!(result.onyomi_examples[2].meaning, "motto, slogan, catchword");

        assert_eq!(result.onyomi_examples[3].kanji, "造語");
        assert_eq!(result.onyomi_examples[3].hiragana, "ゾウゴ");
        assert_eq!(result.onyomi_examples[3].meaning, "coinage, invention of a new word, coined word");
    }
}
