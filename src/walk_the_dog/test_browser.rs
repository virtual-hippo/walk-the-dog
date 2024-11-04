use anyhow::{anyhow, Result};
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;

pub(super) fn draw_ui(html: &str) -> Result<()> {
    Ok(())
}

pub(super) fn hide_ui() -> Result<()> {
    Ok(())
}

pub(super) fn find_html_element_by_id(id: &str) -> Result<(HtmlElement)> {
    Err(anyhow!("Not implemented yet!"))
}

pub(super) async fn fetch_json(json: &str) -> Result<JsValue> {
    Err(anyhow!("Not implemented yet!"))
}
