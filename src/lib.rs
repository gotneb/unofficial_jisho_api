pub mod api;

#[cfg(test)]
mod tests {
    use api::{JishoAPI, JlptLevel};

    use super::*;

    #[test]
    fn search_language_kanji() {
        let result = JishoAPI::search_kanji("語".into()).unwrap();

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
        assert_eq!(
            result.kunyomi_examples[1].meaning,
            "to let slip a secret, to let the cat out of the bag"
        );

        assert_eq!(result.kunyomi_examples[2].kanji, "語らう");
        assert_eq!(result.kunyomi_examples[2].hiragana, "かたらう");
        assert_eq!(
            result.kunyomi_examples[2].meaning,
            "to talk, to tell, to recite, to pledge, to conspire with"
        );

        // Now it must test the onyomis examples
        // -------------------------------------------
        assert_eq!(result.onyomi_examples[0].kanji, "語");
        assert_eq!(result.onyomi_examples[0].hiragana, "ゴ");
        assert_eq!(result.onyomi_examples[0].meaning, "word, language, speech");

        assert_eq!(result.onyomi_examples[1].kanji, "語学");
        assert_eq!(result.onyomi_examples[1].hiragana, "ゴガク");
        assert_eq!(
            result.onyomi_examples[1].meaning,
            "study of foreign languages, linguistics"
        );

        assert_eq!(result.onyomi_examples[2].kanji, "標語");
        assert_eq!(result.onyomi_examples[2].hiragana, "ヒョウゴ");
        assert_eq!(
            result.onyomi_examples[2].meaning,
            "motto, slogan, catchword"
        );

        assert_eq!(result.onyomi_examples[3].kanji, "造語");
        assert_eq!(result.onyomi_examples[3].hiragana, "ゾウゴ");
        assert_eq!(
            result.onyomi_examples[3].meaning,
            "coinage, invention of a new word, coined word"
        );
    }

    #[test]
    fn search_tall_kanji() {
        let result = JishoAPI::search_kanji("高".into()).unwrap();

        assert_eq!(result.taught, "grade 2");
        assert_eq!(result.jlpt_level, JlptLevel::N5);
        assert_eq!(result.stroke_count, 10);
        assert_eq!(result.meaning, "tall, high, expensive");
        assert_eq!(
            result.kunyomi,
            vec!["たか.い", "たか", "-だか", "たか.まる", "たか.める"]
        );
        assert_eq!(result.onyomi, vec!["コウ"]);
        assert_eq!(result.parts, vec!["亠", "冂", "口", "高"]);

        // Now it must test the kunyomis examples
        // -------------------------------------------
        assert_eq!(result.kunyomi_examples[0].kanji, "高い");
        assert_eq!(result.kunyomi_examples[0].hiragana, "たかい");
        assert_eq!(result.kunyomi_examples[0].meaning, "high, tall, expensive, high-priced, high (level), above average (in degree, quality, etc.), loud, high-pitched, shrill");

        assert_eq!(result.kunyomi_examples[1].kanji, "高い高い");
        assert_eq!(result.kunyomi_examples[1].hiragana, "たかいたかい");
        assert_eq!(
            result.kunyomi_examples[1].meaning,
            "lifting (a child) high up in the air"
        );

        assert_eq!(result.kunyomi_examples[2].kanji, "高");
        assert_eq!(result.kunyomi_examples[2].hiragana, "たか");
        assert_eq!(
            result.kunyomi_examples[2].meaning,
            "quantity, amount, volume, number, amount of money"
        );

        assert_eq!(result.kunyomi_examples[3].kanji, "高い");
        assert_eq!(result.kunyomi_examples[3].hiragana, "たかい");
        assert_eq!(result.kunyomi_examples[3].meaning, "high, tall, expensive, high-priced, high (level), above average (in degree, quality, etc.), loud, high-pitched, shrill");

        assert_eq!(result.kunyomi_examples[4].kanji, "威高");
        assert_eq!(result.kunyomi_examples[4].hiragana, "いたか");
        assert_eq!(result.kunyomi_examples[4].meaning, "arrogant");

        assert_eq!(result.kunyomi_examples[5].kanji, "背高");
        assert_eq!(result.kunyomi_examples[5].hiragana, "せいたか");
        assert_eq!(result.kunyomi_examples[5].meaning, "tallness, tall person");

        assert_eq!(result.kunyomi_examples[6].kanji, "高まる");
        assert_eq!(result.kunyomi_examples[6].hiragana, "たかまる");
        assert_eq!(
            result.kunyomi_examples[6].meaning,
            "to rise, to swell, to be promoted"
        );

        assert_eq!(result.kunyomi_examples[7].kanji, "高める");
        assert_eq!(result.kunyomi_examples[7].hiragana, "たかめる");
        assert_eq!(
            result.kunyomi_examples[7].meaning,
            "to raise, to lift, to boost, to enhance"
        );

        // Now it must test the onyomis examples
        // -------------------------------------------
        assert_eq!(result.onyomi_examples[0].kanji, "高");
        assert_eq!(result.onyomi_examples[0].hiragana, "コウ");
        assert_eq!(result.onyomi_examples[0].meaning, "high, high school");

        assert_eq!(result.onyomi_examples[1].kanji, "高圧");
        assert_eq!(result.onyomi_examples[1].hiragana, "コウアツ");
        assert_eq!(
            result.onyomi_examples[1].meaning,
            "high voltage, high pressure"
        );

        assert_eq!(result.onyomi_examples[2].kanji, "中高");
        assert_eq!(result.onyomi_examples[2].hiragana, "チュウコウ");
        assert_eq!(
            result.onyomi_examples[2].meaning,
            "middle and high school, medium and high (level)"
        );

        assert_eq!(result.onyomi_examples[3].kanji, "激昂");
        assert_eq!(result.onyomi_examples[3].hiragana, "ゲッコウ");
        assert_eq!(
            result.onyomi_examples[3].meaning,
            "excitement, exasperation, indignation, rage, fury"
        );
    }

    #[test]
    fn search_example_kanji_day() {
        let results = JishoAPI::search_for_examples("日".into()).unwrap();
        let result = results.get(0).unwrap();

        assert_eq!(result.english, "Today is the night of the full moon. The bright, clear, full moon that appears in the autumn night sky is beautiful.");
    }
}
