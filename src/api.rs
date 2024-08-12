/*
 * This module encapsulates the official Jisho.org API
 * Permission to scrape granted by Jisho's admin Kimtaro:
 *     https://jisho.org/forum/54fefc1f6e73340b1f160000-is-there-any-kind-of-search-api
 */

use std::collections::HashMap;

use regex::Regex;
use reqwest::Error;
use scraper::{ElementRef, Html, Selector};
use soup::{NodeExt, QueryBuilderExt, Soup};

const SCRAPE_BASE_URI: &'static str = "https://jisho.org/search/";

#[derive(PartialEq, Debug)]
pub enum JlptLevel {
    N1,
    N2,
    N3,
    N4,
    N5,
}

#[derive(Debug)]
pub struct YomiExample {
    pub kanji: String,
    pub hiragana: String,
    pub meaning: String,
}

impl YomiExample {
    pub fn new(e: &ElementRef) -> Self {
        let text = e.inner_html();
        let text = text.trim();
        let mut examples: Vec<String> = text.split("\n").map(|e| String::from(e.trim())).collect();

        let kanji = examples.remove(0);
        let hiragana = examples.remove(0);
        let hiragana = hiragana.replace("【", "").replace("】", "");
        let meaning = examples.remove(0);

        Self {
            kanji,
            hiragana,
            meaning,
        }
    }
}

#[derive(Debug)]
pub struct Piece {
    pub lifted: String,
    pub unlifted: String,
}

#[derive(Debug)]
pub struct KanjiExample {
    pub english: String,
    pub kanji: String,
    pub kana: String,
    pub pieces: Vec<Piece>,
}

impl KanjiExample {
    fn new(div: &ElementRef) -> Self {
        let ul_selector = Selector::parse("ul").unwrap();
        let ul = div.select(&ul_selector).collect::<Vec<_>>()[0];

        let english_selector = Selector::parse("span.english").unwrap();
        let english = div.select(&english_selector).collect::<Vec<_>>()[0].inner_html();
        let (kanji, kana, pieces) = Self::get_kanji_kana_and_pieces(&ul);

        Self {
            english,
            kanji,
            kana,
            pieces,
        }
    }

    fn get_kanji_kana_and_pieces(ul: &ElementRef) -> (String, String, Vec<Piece>) {
        let kanji_regex = Regex::new(r#"/[\u4e00-\u9faf\u3400-\u4dbf々]/g"#).unwrap();
        let soup = Soup::new(&ul.inner_html());

        // That's unefficient... It returns the whole phrase with furigana and kanji together...
        // Again... Unefficient. But I wasn't able to think in a more efficent way :(
        let mut results = soup
            .tag(true)
            .find_all()
            .map(|tag| tag.text())
            .collect::<Vec<_>>();

        let result = results.remove(0);
        let kanji_to_furigana = Self::get_kanji_furigana_hash(&ul);

        let mut kanji = result.clone();
        for (_, value) in &kanji_to_furigana {
            kanji = kanji.replace(value, "");
        }

        let mut hiragana = kanji.clone();
        for (key, value) in &kanji_to_furigana {
            hiragana = hiragana.replace(key, value);
        }

        let pieces = Self::get_pieces(&kanji_to_furigana);

        (kanji.trim().into(), hiragana.trim().into(), pieces)
    }

    fn get_kanji_furigana_hash(ul: &ElementRef) -> HashMap<String, String> {
        let furigana_selector = Selector::parse("span.furigana").unwrap();
        let kanji_selector = Selector::parse("span.unlinked").unwrap();

        let mut kanji_to_furigana = HashMap::new();

        let furigana = ul.select(&furigana_selector).collect::<Vec<_>>();
        let kanji = ul.select(&kanji_selector).collect::<Vec<_>>();

        for index in 0..kanji.len() {
            kanji_to_furigana.insert(kanji[index].inner_html(), furigana[index].inner_html());
        }

        kanji_to_furigana
    }

    fn get_pieces(map: &HashMap<String, String>) -> Vec<Piece> {
        let mut p = Vec::new();
        for (key, value) in map {
            let piece = Piece {
                unlifted: key.clone(),
                lifted: value.clone(),
            };
            p.push(piece)
        }
        p
    }
}

// I'm wondering... Should this struct use 'String' or '&str'?
// (Guess I'll have to study more about strings...)
#[derive(Debug)]
pub struct Kanji {
    pub taught: String,
    pub jlpt_level: JlptLevel,
    pub stroke_count: u32,
    pub meaning: String,
    pub kunyomi: Vec<String>,
    pub kunyomi_examples: Vec<YomiExample>,
    pub onyomi: Vec<String>,
    pub onyomi_examples: Vec<YomiExample>,
    pub parts: Vec<String>,
    pub url: String,
}

pub struct JishoAPI;

impl JishoAPI {
    pub fn search_kanji(kanji: String) -> Result<Kanji, Error> {
        let url = format!("{}/{} %23kanji", SCRAPE_BASE_URI, kanji);
        let html = reqwest::blocking::get(&url)?.text()?;

        let html = Html::parse_document(&html);

        let (onyomi_example, kunyomi_example) = Self::extract_examples(&html);

        Ok(Kanji {
            taught: Self::taught_in(&html),
            jlpt_level: Self::jlpt_level(&html),
            stroke_count: Self::stroke_count(&html),
            meaning: Self::meaning(&html),
            kunyomi: Self::kunyomi(&html),
            onyomi: Self::onyomi(&html),
            kunyomi_examples: kunyomi_example,
            onyomi_examples: onyomi_example,
            parts: Self::kanji_parts(&html),
            url,
        })
    }

    /// Scrapes `grade`
    fn taught_in(html: &Html) -> String {
        let selector = Selector::parse("div.grade strong").unwrap();
        let text = html.select(&selector).collect::<Vec<_>>()[0];

        text.inner_html()
    }

    /// Scrapes `JLPT level`
    fn jlpt_level(html: &Html) -> JlptLevel {
        let selector = Selector::parse("div.jlpt strong").unwrap();
        let tag = html.select(&selector).collect::<Vec<_>>()[0];

        let level = match tag.inner_html().as_str() {
            "N1" => JlptLevel::N1,
            "N2" => JlptLevel::N2,
            "N3" => JlptLevel::N3,
            "N4" => JlptLevel::N4,
            "N5" => JlptLevel::N5,
            _ => panic!("Incorrect JLPT level"),
        };

        level
    }

    /// Scrapes number of `strokes`
    fn stroke_count(html: &Html) -> u32 {
        let selector = Selector::parse("div.kanji-details__stroke_count strong").unwrap();
        let text = html.select(&selector).collect::<Vec<_>>()[0];

        text.inner_html().parse().unwrap()
    }

    /// Scrapes `meaning`
    fn meaning(html: &Html) -> String {
        let selector = Selector::parse("div.kanji-details__main-meanings").unwrap();
        let text = html.select(&selector).collect::<Vec<_>>()[0];

        text.inner_html().replace("\n", "").trim().into()
    }

    /// Scrapes `kunyomi`
    fn kunyomi(html: &Html) -> Vec<String> {
        let selector =
            Selector::parse("div.kanji-details__main-readings dl.dictionary_entry.kun_yomi a")
                .unwrap();
        Self::extract_yomi(html, &selector)
    }

    /// Scrapes `onyomi`
    fn onyomi(html: &Html) -> Vec<String> {
        let selector =
            Selector::parse("div.kanji-details__main-readings dl.dictionary_entry.on_yomi a")
                .unwrap();
        Self::extract_yomi(html, &selector)
    }

    /// Helper function to extract `kunyomi` and `onyomi`
    fn extract_yomi(html: &Html, selector: &Selector) -> Vec<String> {
        let elems = html.select(selector).collect::<Vec<_>>();

        elems.iter().map(|e| String::from(e.inner_html())).collect()
    }

    /// Scrapes `onyomi` and `kunyomi`and returns them, respectively.
    fn extract_examples(html: &Html) -> (Vec<YomiExample>, Vec<YomiExample>) {
        let selector =
            Selector::parse("div.small-12.columns div.row.compounds div.small-12.large-6.columns")
                .unwrap();
        let columns = html.select(&selector).collect::<Vec<_>>();

        // There's always 2 columns
        let onyomi_column = columns.get(0).unwrap();
        let kunyomi_column = columns.get(1).unwrap();

        // TODO: Refactor it later. I'll be ashamed if someone else see this...
        let onyomi_selector = Selector::parse("ul.no-bullet li").unwrap();
        let elems = onyomi_column.select(&onyomi_selector).collect::<Vec<_>>();
        let onyomi = elems.iter().map(|e| YomiExample::new(e)).collect();

        let kunyomi_selector = Selector::parse("ul.no-bullet li").unwrap();
        let elems = kunyomi_column.select(&kunyomi_selector).collect::<Vec<_>>();
        let kunyomi = elems.iter().map(|e| YomiExample::new(e)).collect();

        (onyomi, kunyomi)
    }

    /// Scrapes parts that kanji is build up
    fn kanji_parts(html: &Html) -> Vec<String> {
        let selector = Selector::parse("div.radicals dl.dictionary_entry.on_yomi dd a").unwrap();
        Self::extract_yomi(html, &selector)
    }

    pub fn search_for_examples(kanji: String) -> Result<Vec<KanjiExample>, Error> {
        let url = format!("{}/{}%23sentences", SCRAPE_BASE_URI, kanji);
        let html = reqwest::blocking::get(&url)?.text()?;
        let html = Html::parse_document(&html);
        let divs = Selector::parse("div.sentence_content").unwrap();

        let divs = html.select(&divs);
        Ok(divs.map(|div| KanjiExample::new(&div)).collect())
    }
}
