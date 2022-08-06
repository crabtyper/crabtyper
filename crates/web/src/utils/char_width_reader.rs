/*
* In order to render the cursor correctly on the screen we need to know the pixel width of each
* character.
* This module is based on vscode it's implementation for creating a cursor in the browser.
* https://github.com/microsoft/vscode
*/

use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement};

#[derive(Default)]
pub struct CharWidthRequest {
    pub char: Option<char>,
    pub width: Option<i32>,
}

impl CharWidthRequest {
    pub fn fulfill(&mut self, width: i32) {
        self.width = Some(width);
    }
}

pub struct DomCharWidthReader<'a> {
    // requests: Rc<RefCell<Vec<CharWidthRequest>>>,
    requests: &'a mut Vec<CharWidthRequest>,

    container: Option<HtmlElement>,
    test_elements: Option<Vec<HtmlElement>>,
}

impl DomCharWidthReader<'_> {
    pub fn new(requests: &mut Vec<CharWidthRequest>) -> DomCharWidthReader {
        DomCharWidthReader {
            requests,
            container: None,
            test_elements: None,
        }
    }

    pub fn read(&mut self) {
        self.create_dom_elements();

        if let Some(container) = &self.container {
            gloo::utils::document()
                .body()
                .unwrap()
                .append_child(container)
                .unwrap();
        }

        self.read_from_dom_elements();
    }

    fn create_dom_elements(&mut self) {
        let document = gloo::utils::document();

        let container: HtmlElement = Document::create_element(&document, "div")
            .unwrap()
            .dyn_ref::<HtmlElement>()
            .unwrap()
            .clone();

        container
            .set_attribute("style", "position:absolute;top:-50000px;width:500000px")
            .unwrap();

        let dom_node: HtmlElement = Document::create_element(&document, "div")
            .unwrap()
            .dyn_ref::<HtmlElement>()
            .unwrap()
            .clone();
        container.append_child(&dom_node).unwrap();

        let test_elements: &mut Vec<HtmlElement> = &mut vec![];

        for request in self.requests.iter() {
            let parent: &HtmlElement = &dom_node;

            parent
                .append_child(&Document::create_element(&document, "br").unwrap())
                .unwrap();

            let test_element: HtmlElement = Document::create_element(&document, "span")
                .unwrap()
                .dyn_ref::<HtmlElement>()
                .unwrap()
                .clone();
            DomCharWidthReader::render(&test_element, request);

            parent.append_child(&test_element).unwrap();

            test_elements.push(test_element);
        }

        self.container = Some(container);
        self.test_elements = Some(test_elements.to_vec());
    }

    fn render(test_element: &HtmlElement, request: &CharWidthRequest) {
        if let Some(chr) = request.char {
            if chr == ' ' {
                let mut html_string = String::from("\u{00a0}");
                for _ in 0..8 {
                    html_string.push_str(&html_string.clone());
                }
                test_element.set_inner_text(&html_string);
            } else {
                let mut test_string = String::from(chr);
                for _ in 0..8 {
                    test_string.push_str(&test_string.clone());
                }
                test_element.set_text_content(Some(&test_string));
            }
        }
    }

    fn read_from_dom_elements(&mut self) {
        if let Some(test_elements) = &self.test_elements {
            (0..self.requests.len()).for_each(|i| {
                let request = &mut self.requests[i];
                let test_element = &test_elements[i];

                request.fulfill(test_element.offset_width() / 256)
            });
        }
    }
}

pub fn read_char_widths(requests: &mut Vec<CharWidthRequest>) {
    let mut reader = DomCharWidthReader::new(requests);

    reader.read();
}
