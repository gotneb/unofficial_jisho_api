use thirtyfour_sync::{http::reqwest_sync::ReqwestDriverSync, prelude::*, GenericWebDriver};

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
    fn new(element: &WebElement) -> Self {
        let text = element.inner_html().unwrap();
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

// Should they really use `String`? Or should they use `&str`
// I don't know the difference :(
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

pub struct JishoAPI {
    driver: GenericWebDriver<ReqwestDriverSync>,
}

// impl Drop for API {
//     fn drop(&mut self) {
//         self.driver.quit().unwrap();
//     }
// }

impl JishoAPI {
    pub fn new() -> Self {
        let caps = DesiredCapabilities::chrome();
        let driver = WebDriver::new("http://localhost:4444", &caps).unwrap();
        Self { driver }
    }

    pub fn search_kanji(&self, kanji: String) -> Kanji {
        let url = format!("{}/{} %23kanji", SCRAPE_BASE_URI, kanji);

        self.driver.get(&url).unwrap();

        let (on_example, kun_example) = self.extract_examples();

        Kanji {
            taught: self.taught_in(),
            jlpt_level: self.jlpt_level(),
            stroke_count: self.stroke_count(),
            meaning: self.meaning(),
            kunyomi: self.kunyomi(),
            kunyomi_examples: kun_example,
            onyomi: self.onyomi(),
            onyomi_examples: on_example,
            parts: self.kanji_parts(),
            url,
        }
    }

    /// Scrapes `grade`
    fn taught_in(&self) -> String {
        let elem = self
            .driver
            .find_element(By::Css("div.grade strong"))
            .unwrap();

        elem.inner_html().unwrap()
    }

    /// Scrapes `JLPT level`
    fn jlpt_level(&self) -> JlptLevel {
        let elem = self
            .driver
            .find_element(By::Css("div.jlpt strong"))
            .unwrap();

        match elem.inner_html().unwrap().as_str() {
            "N1" => JlptLevel::N1,
            "N2" => JlptLevel::N2,
            "N3" => JlptLevel::N3,
            "N4" => JlptLevel::N4,
            "N5" => JlptLevel::N5,
            _ => panic!("Incorrect JLPT level"),
        }
    }

    /// Scrapes number of `strokes`
    fn stroke_count(&self) -> u32 {
        let elem = self
            .driver
            .find_element(By::Css("div.kanji-details__stroke_count strong"))
            .unwrap();

        elem.inner_html().unwrap().parse().unwrap()
    }

    /// Scrapes `meaning`
    fn meaning(&self) -> String {
        let elem = self
            .driver
            .find_element(By::Css("div.kanji-details__main-meanings"))
            .unwrap();

        elem.inner_html().unwrap().replace("\n", "").trim().into()
    }

    /// Scrapes `kunyomi`
    fn kunyomi(&self) -> Vec<String> {
        self.extract_yomi("div.kanji-details__main-readings dl.dictionary_entry.kun_yomi a")
    }

    /// Scrapes `onyomi` and `kunyomi` examples. Returns in that order.
    fn extract_examples(&self) -> (Vec<YomiExample>, Vec<YomiExample>) {
        let elems = self
            .driver
            .find_elements(By::Css(
                "div.small-12.columns div.row.compounds div.small-12.large-6.columns",
            ))
            .unwrap();

        // There's always 2 columns
        let onyomi_column = elems.get(0).unwrap();
        let kunyomi_column = elems.get(1).unwrap();

        // TODO: Refactor it later. I'll be ashamed if someone else see this...
        let elems = onyomi_column
            .find_elements(By::Css("ul.no-bullet li"))
            .unwrap();
        let onyomi = elems.iter().map(|e| YomiExample::new(e)).collect();

        let elems = kunyomi_column
            .find_elements(By::Css("ul.no-bullet li"))
            .unwrap();
        let kunyomi = elems.iter().map(|e| YomiExample::new(e)).collect();

        (onyomi, kunyomi)
    }

    /// Scrapes `onyomi`
    fn onyomi(&self) -> Vec<String> {
        self.extract_yomi("div.kanji-details__main-readings dl.dictionary_entry.on_yomi a")
    }

    /// Scrapes parts that kanji is build up
    fn kanji_parts(&self) -> Vec<String> {
        self.extract_yomi("div.radicals dl.dictionary_entry.on_yomi dd a")
    }

    /// Helper function to extract `kunyomi` and `onyomi`
    fn extract_yomi(&self, css_locator: &str) -> Vec<String> {
        let elems = self.driver.find_elements(By::Css(css_locator)).unwrap();

        elems
            .iter()
            .map(|e| String::from(e.inner_html().unwrap()))
            .collect()
    }
}
