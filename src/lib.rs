use serde::{Serialize, Serializer};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PdfParser {}

#[wasm_bindgen]
impl PdfParser {
    pub fn parse(data: &[u8]) -> Result<JsValue, JsError> {
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
}

#[derive(Serialize, Default)]
struct JsPdfFile {
    pages: Vec<JsPdfPage>,
}

#[derive(Serialize, Default)]
struct JsPdfPage {
    operations: Vec<JsPdfOp>,
}
struct JsPdfOp(pdf::content::Op);

impl Serialize for JsPdfOp {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // TODO
        match self.0 {
            _ => ser.serialize_str(&format!("{:?}", self.0)),
        }
    }
}
