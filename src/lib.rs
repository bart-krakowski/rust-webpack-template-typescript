use wasm_bindgen::prelude::*;
use web_sys::Document;
use web_sys::HtmlElement;

#[wasm_bindgen]
pub fn run() {
    // get window/document/body
    let window = web_sys::window().expect("Could not get window");
    let document = window.document().expect("Could not get document");
    let body = document.body().expect("Could not get body");

    mount_app(&document, &body);
}

fn mount_app(document: &Document, body: &HtmlElement) {
    mount_title(&document, &body);
}

// Create a title
fn mount_title(document: &Document, body: &HtmlElement) {
    // create title element
    let title = document
        .create_element("h1")
        .expect("Could not create element");
    let title_text = document.create_text_node("DOT");
    title
        .append_child(&title_text)
        .expect("Could not append child to title");

    // append to body
    body.append_child(&title)
        .expect("Could not append title to body");
}