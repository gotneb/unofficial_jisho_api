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

pub struct Kanji {
    pub taught: String,
    pub jlpt_level: JlptLevel,
    pub stroke_count: u32,
    pub meaning: Vec<String>,
    // kunyomi: Vec<String>,
    // onyomi: Vec<String>,
    // uri: String,
}

pub struct API {
    driver: GenericWebDriver<ReqwestDriverSync>,
}

impl API {
    pub fn new() -> Self {
        let caps = DesiredCapabilities::chrome();
        let driver = WebDriver::new("http://localhost:4444", &caps).unwrap();
        Self { driver }
    }

    pub fn search_kanji(&self, kanji: String) -> Kanji {
        self.driver
            .get(format!("{}/{} %23kanji", SCRAPE_BASE_URI, kanji))
            .unwrap();

        Kanji {
            taught: self.taught_in(),
            jlpt_level: self.jlpt_level(),
            stroke_count: self.stroke_count(),
            meaning: self.meaning(),
        }
    }

    fn taught_in(&self) -> String {
        let elem = self
            .driver
            .find_element(By::Css("div.grade strong"))
            .unwrap();

        elem.inner_html().unwrap()
    }

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

    fn stroke_count(&self) -> u32 {
        let elem = self
            .driver
            .find_element(By::Css("div.kanji-details__stroke_count strong"))
            .unwrap();

        elem.inner_html().unwrap().parse().unwrap()
    }

    fn meaning(&self) -> Vec<String> {
        todo!();

        // Borrow checker errror
        // let elem = self
        //     .driver
        //     .find_element(By::Css("div.kanji-details__main-meanings"))
        //     .unwrap();

        // let meanings: Vec<&str> = elem
        //     .inner_html()
        //     .unwrap()
        //     .replace("\n", "")
        //     .replace(" ", "")
        //     .split("'")
        //     .collect();
    }
}
