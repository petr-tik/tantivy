extern crate wasm_bindgen;

extern crate tantivy;

use tantivy::collector::TopCollector;
use tantivy::directory::static_directory::{StaticDirectory};
use tantivy::query::QueryParser;
use tantivy::Index;

use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// pub fn instantiate_index(data: &[u8]) -> Index {
//     index
// }


#[wasm_bindgen]
pub fn query(data: &[u8], query: &str) -> String {
    let directory = StaticDirectory::open(data).unwrap();
    let index = Index::open(directory).unwrap();
    index.load_searchers().unwrap();


    let searcher = index.searcher();

    let schema = index.schema();
    let command = schema.get_field("command").unwrap();
    let text = schema.get_field("text").unwrap();

    let query_parser = QueryParser::for_index(&index, vec![command, text]);
    let query = query_parser.parse_query(query).unwrap();

    let mut top_collector = TopCollector::with_limit(10);

    searcher.search(&*query, &mut top_collector).unwrap();

    let doc_addresses = top_collector.docs();

    // The actual documents still need to be

    // retrieved from Tantivy's store.

    //
    // Since the body field was not configured as stored,
    // the document returned will only contain
    // a title.
    let mut docs = Vec::new();

    for doc_address in doc_addresses {
        let retrieved_doc = searcher.doc(doc_address).unwrap();
        docs.push(schema.to_json(&retrieved_doc));
    }

    docs.join(";")
}

#[cfg(test)]
mod tests {}
