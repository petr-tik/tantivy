extern crate wasm_bindgen;

extern crate tantivy;

use tantivy::collector::TopCollector;
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
pub fn query(query: &str) -> String {
    let index = instantiate_index(DATA).unwrap();
    let searcher = index.searcher();
    let schema = index.schema();
    let command = schema.get_field("command").unwrap();
    let text = schema.get_field("text").unwrap();

    let query_parser = QueryParser::for_index(&index, vec![command, text]);
    let query = query_parser.parse_query(query).unwrap();

    let mut top_collector = TopCollector::with_limit(10);

    searcher.search(&*query, &mut top_collector).unwrap();

    let doc_addresses = top_collector.docs();

    let mut docs = Vec::new();

    for doc_address in doc_addresses {
        let retrieved_doc = searcher.doc(doc_address).unwrap();
        docs.push(schema.to_json(&retrieved_doc));
    }

    docs.join(";")
}

#[cfg(test)]
mod tests {}
