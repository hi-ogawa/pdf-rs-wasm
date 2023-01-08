use gloo_utils::format::JsValueSerdeExt;
use pdf_render::{
    render_page,
    tracer::{DrawItem, TraceCache, Tracer},
    RectF,
};
use schemars::JsonSchema;
use serde::Serialize;
use serde_with::serde_as;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PdfParser {}

#[wasm_bindgen]
impl PdfParser {
    pub fn parse_operations(data: &[u8]) -> Result<JsPdfFileDts, JsError> {
        let file = pdf::file::File::from_data(data)?;

        let mut js_file = JsPdfFile::default();

        for page in file.pages() {
            let mut js_page = JsPdfPage::default();

            if let Some(contents) = page?.contents.as_ref() {
                let ops = contents.operations(&file)?;
                js_page.operations = ops.iter().map(|op| format!("{:?}", op)).collect();
            }

            js_file.pages.push(js_page);
        }

        Ok(JsValue::from_serde(&js_file)?.into())
    }

    pub fn parse_trace(data: &[u8]) -> Result<JsPdfFileTraceDts, JsError> {
        let file = pdf::file::File::from_data(data)?;
        let mut cache = TraceCache::new_embedded();
        let mut result = JsPdfFileTrace::default();
        for page in file.pages() {
            let page = page?;
            let mut backend = Tracer::new(&mut cache);
            render_page(&mut backend, &file, &page, Default::default())?;
            let view_box = backend.view_box();
            let items = backend.finish();
            result.pages.push(JsPdfPageTrace { view_box, items });
        }
        Ok(JsValue::from_serde(&result)?.into())
    }
}

#[derive(Serialize, JsonSchema, Default)]
struct JsPdfFile {
    pages: Vec<JsPdfPage>,
}

#[derive(Serialize, JsonSchema, Default)]
struct JsPdfPage {
    operations: Vec<String>,
}

#[derive(Serialize, JsonSchema, Default)]
struct JsPdfFileTrace {
    pages: Vec<JsPdfPageTrace>,
}

#[serde_as]
#[derive(Serialize, JsonSchema, Default)]
struct JsPdfPageTrace {
    #[schemars(with = "pdf_render::serde_utils::LocalRectF")]
    #[serde_as(as = "pdf_render::serde_utils::AsLocalRectF")]
    view_box: RectF,
    items: Vec<DrawItem>,
}

//
// export typing via json-schema
//

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "JsPdfFile")]
    pub type JsPdfFileDts;

    #[wasm_bindgen(typescript_type = "JsPdfFileTrace")]
    pub type JsPdfFileTraceDts;
}

#[wasm_bindgen(typescript_custom_section)]
const TYPESCRIPT_EXTRA: &'static str = r#"
/* __TYPESCRIPT_EXTRA__START__ */
import { JsPdfFile, JsPdfFileTrace } from "./types";
export { JsPdfFile, JsPdfFileTrace }
/* __TYPESCRIPT_EXTRA__END__ */
"#;

#[cfg(test)]
pub mod tests {
    use schemars::{schema_for, JsonSchema};
    use wasm_bindgen_test::*;
    use web_sys::console;

    #[derive(JsonSchema)]
    struct __RootSchema(crate::JsPdfFile, crate::JsPdfFileTrace);

    #[wasm_bindgen_test]
    fn export_json_schema() {
        let schema = schema_for!(__RootSchema);
        let schema_str = serde_json::to_string_pretty(&schema).unwrap();
        if cfg!(feature = "export_json_schema") {
            console::log_1(&"__JSON_SCHEMA_START__".into());
            console::log_1(&schema_str.into());
            console::log_1(&"__JSON_SCHEMA_END__".into());
        }
    }
}
