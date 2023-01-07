use gloo_utils::format::JsValueSerdeExt;
use pdf::content::Op;
use pdf_render::{
    render_page,
    tracer::{DrawItem, TraceCache, Tracer},
};
use schemars::JsonSchema;
use serde::{Serialize, Serializer};
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
                js_page.operations = ops.iter().map(|op| JsPdfOp::new(op.clone())).collect();
            }

            js_file.pages.push(js_page);
        }

        Ok(JsValue::from_serde(&js_file)?.into())
    }

    pub fn parse_trace(data: &[u8]) -> Result<FileTraceDts, JsError> {
        let file = pdf::file::File::from_data(data)?;
        let mut cache = TraceCache::new_embedded();
        let mut result = FileTrace::default();
        for page in file.pages() {
            let page = page?;
            let mut backend = Tracer::new(&mut cache);
            render_page(&mut backend, &file, &page, Default::default()).unwrap();
            let items = backend.finish();
            result.pages.push(PageTrace { items });
        }
        Ok(JsValue::from_serde(&result)?.into())
    }
}

#[derive(Serialize, JsonSchema, Default)]
struct FileTrace {
    pages: Vec<PageTrace>,
}

#[derive(Serialize, JsonSchema, Default)]
struct PageTrace {
    items: Vec<DrawItem>,
}

#[derive(Serialize, JsonSchema, Default)]
struct JsPdfFile {
    pages: Vec<JsPdfPage>,
}

#[derive(Serialize, JsonSchema, Default)]
struct JsPdfPage {
    operations: Vec<JsPdfOp>,
}
#[derive(Serialize, JsonSchema)]
struct JsPdfOp {
    #[serde(skip)]
    #[allow(unused)]
    inner: Op,
    debug: String,
}

impl JsPdfOp {
    fn new(inner: Op) -> Self {
        let debug = format!("{:?}", inner);
        Self { inner, debug }
    }
}

//
// export typing via json-schema
//

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "JsPdfFile")]
    pub type JsPdfFileDts;

    #[wasm_bindgen(typescript_type = "FileTrace")]
    pub type FileTraceDts;
}

#[wasm_bindgen(typescript_custom_section)]
const TYPESCRIPT_EXTRA: &'static str = r#"
/* __TYPESCRIPT_EXTRA__START__ */

import { JsPdfFile, FileTrace } from "./types";
export { JsPdfFile, FileTrace }

/* __TYPESCRIPT_EXTRA__END__ */
"#;

#[cfg(test)]
pub mod tests {
    use schemars::schema_for;
    use wasm_bindgen_test::*;
    use web_sys::console;

    #[wasm_bindgen_test]
    fn export_json_schema() {
        let schema = schema_for!(super::FileTrace);
        let schema_str = serde_json::to_string_pretty(&schema).unwrap();
        if cfg!(feature = "export_json_schema") {
            console::log_1(&"__JSON_SCHEMA_START__".into());
            console::log_1(&schema_str.into());
            console::log_1(&"__JSON_SCHEMA_END__".into());
        }
    }
}
