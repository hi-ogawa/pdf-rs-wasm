use pdf::content::Op;
use pdf_render::{
    render_page,
    tracer::{TraceCache, Tracer},
};
use serde::{Serialize, Serializer};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PdfParser {}

#[wasm_bindgen]
impl PdfParser {
    pub fn parse_operations(data: &[u8]) -> Result<JsValue, JsError> {
        let file = pdf::file::File::from_data(data)?;

        let mut js_file = JsPdfFile::default();

        for page in file.pages() {
            let mut js_page = JsPdfPage::default();

            if let Some(contents) = page?.contents.as_ref() {
                let ops = contents.operations(&file)?;
                js_page.operations = ops.iter().map(|op| JsPdfOp(op.clone())).collect();
            }

            js_file.pages.push(js_page);
        }

        Ok(serde_wasm_bindgen::to_value(&js_file)?)
    }

    pub fn parse_trace(data: &[u8]) -> Result<JsValue, JsError> {
        let file = pdf::file::File::from_data(data)?;
        let mut cache = TraceCache::new_embedded();
        let mut result: Vec<String> = vec![];
        for page in file.pages() {
            let page = page?;
            let mut backend = Tracer::new(&mut cache);
            render_page(&mut backend, &file, &page, Default::default()).unwrap();
            let items = backend.finish();
            for item in items {
                result.push(format!("{:?}", item));
            }
        }
        Ok(serde_wasm_bindgen::to_value(&result)?)
    }
}

#[derive(Serialize, Default)]
struct JsPdfFile {
    pages: Vec<JsPdfPage>,
}

#[derive(Serialize, Default)]
struct JsPdfPage {
    operations: Vec<JsPdfOp>,
}
struct JsPdfOp(Op);

impl Serialize for JsPdfOp {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let op = &self.0;
        // TODO
        // use serde::ser::{SerializeStruct, SerializeStructVariant};
        // match op {
        //     Op::TextDraw { text } => {
        //         let mut ser = ser.serialize_struct_variant("Op", 0, "TextDraw", 1)?;
        //         ser.serialize_field(
        //             "text",
        //             &text
        //                 .to_string_lossy()
        //                 .map_or("__INVALID__".to_string(), |v| v.clone()),
        //         )?;
        //         ser.end()
        //     }
        //     _ => {
        //         let mut ser = ser.serialize_struct("__OTHER__", 1)?;
        //         ser.serialize_field("__DEBUG__", &format!("{:?}", op))?;
        //         ser.end()
        //     }
        // }
        match op {
            _ => ser.serialize_str(&format!("{:?}", op)),
        }
    }
}
