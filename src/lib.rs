//! # The Mole (Krteček)

//! Library for searching HTML elements using ID, tags, and class in HTML pages.

//! ## Usage

//! ```
//! use krtek::Krtecek;

//! let krtecek = Krtecek::new("https://example.com")
//!     .find_id("username")
//!     .find_tag("h1")
//!     .find_class("container");
//! ```
//! ## Example

//! ![Code example](https://raw.githubusercontent.com/9hb/krtecek/refs/heads/main/assets/showcase-code-2.png)

//! ![Compilation result](https://raw.githubusercontent.com/9hb/krtecek/refs/heads/main/assets/showcase-terminal-2.png)

use reqwest::{ blocking::get, Error };
use scraper::{ Html, Selector };
use std::collections::HashMap;

pub struct Krtecek {
    link: String,
    ids: Vec<String>,
    tags: Vec<String>,
    classes: Vec<String>,
}

impl Krtecek {
    pub fn new(link: impl Into<String>) -> Self {
        Krtecek {
            link: link.into(),
            ids: Vec::new(),
            tags: Vec::new(),
            classes: Vec::new(),
        }
    }

    pub fn find_id(mut self, id: impl Into<String>) -> Self {
        self.ids.push(id.into());
        self
    }

    pub fn find_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    pub fn find_class(mut self, class: impl Into<String>) -> Self {
        self.classes.push(class.into());
        self
    }

    pub fn run(self) -> Result<HashMap<String, Vec<String>>, Error> {
        let body = get(&self.link)?.text()?;
        let document = Html::parse_document(&body);
        let mut result = HashMap::new();

        for id in self.ids {
            let choose = format!("#{}", id);
            let ch = Selector::parse(&choose).unwrap();
            let elements: Vec<String> = document
                .select(&ch)
                .map(|e| e.inner_html())
                .collect();
            result.insert(format!("#{}", id), elements);
        }

        for tag in self.tags {
            let ch = Selector::parse(&tag).unwrap();
            let elements: Vec<String> = document
                .select(&ch)
                .map(|e| e.inner_html())
                .collect();
            result.insert(tag, elements);
        }

        for class in self.classes {
            let choose = format!(".{}", class);
            let ch = Selector::parse(&choose).unwrap();
            let elements: Vec<String> = document
                .select(&ch)
                .map(|e| e.inner_html())
                .collect();
            result.insert(format!(".{}", class), elements);
        }

        Ok(result)
    }
}
