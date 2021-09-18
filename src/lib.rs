 use wasm_bindgen::prelude::*;
 use wasm_bindgen::JsCast;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
  // Use `web_sys`'s global `window` function to get a handle on the global
  // window object.
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let body = document.body().expect("document should have a body");

  let button_element = document.create_element("button").expect("cant make button");
  let button = button_element.dyn_ref::<web_sys::HtmlElement>().expect("fail dyn_ref");
  button.set_text_content(Some("click me"));
  body.append_child(&button)?;

  let button_onclick_doc = document.clone();
  let button_onclick = Closure::wrap(Box::new(move || {
    let body = button_onclick_doc.body().expect("document should have a body");
    // Manufacture the element we're gonna append
    let val = button_onclick_doc.create_element("p").expect("cannot create p");
    val.set_text_content(Some("Hello from Rust!"));
    body.append_child(&val).expect("cannot append child");
  }) as Box<dyn Fn()>);
  button.set_onclick(Some(button_onclick.as_ref().unchecked_ref()));
  button_onclick.forget();

  Ok(())
}

