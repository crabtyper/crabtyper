use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement};

pub struct CharWidthRequest {
    pub char: Option<char>,
    pub width: u8,
}

impl Default for CharWidthRequest {
    fn default() -> Self {
        CharWidthRequest {
            char: None,
            width: 0,
        }
    }
}

pub struct DomCharWidthReader {
    requests: Vec<CharWidthRequest>,

    container: HtmlElement,
    test_elements: Vec<HtmlElement>,
}

impl DomCharWidthReader {
    pub fn read(&mut self) {
        self.create_dom_elements();
        todo!();
    }

    fn create_dom_elements(&mut self) {
        let document: Document = Document::new().unwrap();

        let container: HtmlElement = Document::create_element(&document, "div")
            .unwrap()
            .dyn_ref::<HtmlElement>()
            .unwrap()
            .clone();
        container.set_attribute("style", "position:absolute;top:-50000px:width:500000px");

        let dom_node: HtmlElement = Document::create_element(&document, "div")
            .unwrap()
            .dyn_ref::<HtmlElement>()
            .unwrap()
            .clone();
        container.append_child(&dom_node);

        let test_elements: &mut Vec<HtmlElement> = &mut vec![];

        for request in &self.requests {
            let parent: &HtmlElement = &dom_node;

            parent.append_child(&Document::create_element(&document, "br").unwrap());

            let test_element: HtmlElement = Document::create_element(&document, "span")
                .unwrap()
                .dyn_ref::<HtmlElement>()
                .unwrap()
                .clone();
            DomCharWidthReader::render(&test_element, request);

            test_elements.push(test_element);
        }

        self.container = container;
        self.test_elements = test_elements.to_vec();
    }

    fn render(test_element: &HtmlElement, request: &CharWidthRequest) {
        if let Some(chr) = request.char {
            if chr == ' ' {
                let mut html_string = String::from("\u{00a0}");
                for i in 0..8 {
                    html_string.push_str(&html_string.clone());
                }
                &test_element.set_inner_text(&html_string);
            } else {
                let mut test_string = String::from(chr);
                for i in 0..8 {
                    test_string.push_str(&test_string.clone());
                }
                &test_element.set_text_content(Some(&test_string));
            }
        }
    }

    fn read_from_dom_elements() {
        todo!();
    }
}
