use wasm_bindgen::prelude::*;
use pulldown_cmark::{Options, Parser};

#[wasm_bindgen]
pub fn parse_markdown(input: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(input, options);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    html_output
}