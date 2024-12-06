use reqwest::{ blocking::get, Error };
use scraper::{ Html, Selector };
use std::collections::HashMap;

pub struct Krtecek {
    odkaz: String,
    ids: Vec<String>, // Vec<String> pro vic; Option<String> pro jeden
    tags: Vec<String>,
    classes: Vec<String>,
}

impl Krtecek {
    pub fn new(odkaz: impl Into<String>) -> Self {
        // pub fn new(odkaz: String) -> Self { (s timhle je nutny u vseho psat .to_string)
        Krtecek {
            odkaz: odkaz.into(), // odkaz,
            ids: Vec::new(), // Vec::new() pro vice hledani najednou (None pro hledani od kazdeho druhu jednou)
            tags: Vec::new(),
            classes: Vec::new(),
        }
    }

    pub fn najit_id(mut self, id: impl Into<String>) -> Self {
        self.ids.push(id.into());
        self
    }

    pub fn najit_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    pub fn najit_class(mut self, class: impl Into<String>) -> Self {
        self.classes.push(class.into());
        self
    }

    pub fn zapnout(self) -> Result<HashMap<String, Vec<String>>, Error> {
        let body = get(&self.odkaz)?.text()?;
        let dokument = Html::parse_document(&body);
        let mut vysledek = HashMap::new();

        //  hleda element podle ID
        for id in self.ids {
            let vybrat = format!("#{}", id);
            let vyb = Selector::parse(&vybrat).unwrap();
            let elementy: Vec<String> = dokument
                .select(&vyb)
                .map(|e| e.inner_html())
                .collect();
            vysledek.insert(format!("#{}", id), elementy);
        }

        //  hleda element podle tagu
        for tag in self.tags {
            let vyb = Selector::parse(&tag).unwrap();
            let elementy: Vec<String> = dokument
                .select(&vyb)
                .map(|e| e.inner_html())
                .collect();
            vysledek.insert(tag, elementy);
        }

        //  hleda element podle classy
        for class in self.classes {
            let vybrat = format!(".{}", class);
            let vyb = Selector::parse(&vybrat).unwrap();
            let elementy: Vec<String> = dokument
                .select(&vyb)
                .map(|e| e.inner_html())
                .collect();
            vysledek.insert(format!(".{}", class), elementy);
        }

        Ok(vysledek)
    }
}
