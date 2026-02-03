use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Element, HtmlButtonElement, HtmlInputElement, KeyboardEvent};

use calc_core::{eval_expression, format_all};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = window()
        .and_then(|win| win.document())
        .ok_or_else(|| JsValue::from_str("document not available"))?;

    let input = document
        .get_element_by_id("expr")
        .ok_or_else(|| JsValue::from_str("missing #expr"))?
        .dyn_into::<HtmlInputElement>()?;
    let button = document
        .get_element_by_id("eval-btn")
        .ok_or_else(|| JsValue::from_str("missing #eval-btn"))?
        .dyn_into::<HtmlButtonElement>()?;
    let out_bin = document
        .get_element_by_id("out-bin")
        .ok_or_else(|| JsValue::from_str("missing #out-bin"))?;
    let out_dec = document
        .get_element_by_id("out-dec")
        .ok_or_else(|| JsValue::from_str("missing #out-dec"))?;
    let out_hex = document
        .get_element_by_id("out-hex")
        .ok_or_else(|| JsValue::from_str("missing #out-hex"))?;
    let out_error = document
        .get_element_by_id("out-error")
        .ok_or_else(|| JsValue::from_str("missing #out-error"))?;

    let input = Rc::new(input);
    let out_bin = Rc::new(out_bin);
    let out_dec = Rc::new(out_dec);
    let out_hex = Rc::new(out_hex);
    let out_error = Rc::new(out_error);

    let input_for_eval = Rc::clone(&input);
    let eval_action = Rc::new(move || {
        let expr = input_for_eval.value();
        let result = eval_expression(&expr).and_then(format_all);
        match result {
            Ok(formatted) => {
                set_text(&out_bin, &formatted.bin);
                set_text(&out_dec, &formatted.dec);
                set_text(&out_hex, &formatted.hex);
                set_text(&out_error, "");
            }
            Err(err) => {
                set_text(&out_bin, "—");
                set_text(&out_dec, "—");
                set_text(&out_hex, "—");
                set_text(&out_error, &err.to_string());
            }
        }
    });

    let eval_for_click = Rc::clone(&eval_action);
    let click_closure = Closure::wrap(Box::new(move || {
        eval_for_click();
    }) as Box<dyn FnMut()>);
    button.add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref())?;
    click_closure.forget();

    let eval_for_key = Rc::clone(&eval_action);
    let key_closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        if event.key() == "Enter" {
            eval_for_key();
        }
    }) as Box<dyn FnMut(KeyboardEvent)>);
    input.add_event_listener_with_callback("keydown", key_closure.as_ref().unchecked_ref())?;
    key_closure.forget();

    Ok(())
}

fn set_text(element: &Element, text: &str) {
    element.set_text_content(Some(text));
}
