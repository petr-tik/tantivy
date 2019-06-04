extern crate wasm_bindgen;

extern crate tantivy;

use tantivy::collector::TopDocs;
use tantivy::directory::static_directory::StaticDirectory;
use tantivy::query::QueryParser;
use tantivy::Index;

use wasm_bindgen::prelude::*;

static DATA: &'static [u8] = include_bytes!("../index.bin");


/// Given an byte-array build and return a tantivy index
///
/// Byte-array will be received by the browser from the server,
/// where it had been built by tantivy-cli
/// Creates an index and loads searchers,
/// the `query` function can call for a searcher
fn instantiate_index(data: &'static [u8]) -> Result<Index, tantivy::Error> {
    let directory = StaticDirectory::open(data)?;
    let index = Index::open(directory)?;
    index.load_searchers()?;
    Ok(index)
}

#[wasm_bindgen]
pub fn query(query: &str) -> Vec<JsValue> {
    let index = instantiate_index(DATA).unwrap();
    let searcher = index.searcher();
    let schema = index.schema();
    let fields = schema.fields();

    let query_parser = QueryParser::for_index(&index, fields);
    let query = query_parser.parse_query(query).unwrap();

    let top_docs = searcher.search(&query, &TopDocs::with_limit(10)).unwrap();
    let mut docs: Vec<JsValue> = Vec::new();
    for (_score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address).unwrap();
        docs.push(JsValue::from_serde(&schema.to_json(&retrieved_doc)).unwrap());
    }
    docs
}

