extern crate tantivy;
extern crate tempdir;
use tempdir::TempDir;

use tantivy::directory::static_directory::{write_static_from_directory};
use tantivy::Index;
use std::path::Path;
use tantivy::schema::*;
use std::fs::File;
use std::io::prelude::*;

fn save_test_index(serialized_path: &Path) {
    let index_dir = TempDir::new("index").unwrap();
    // create index on top of content dir

    {
        let mut schema_builder = SchemaBuilder::default();
        schema_builder.add_text_field("title", TEXT | STORED);
        schema_builder.add_text_field("body", TEXT);
        let schema = schema_builder.build();
        let _index = Index::create_in_dir(&index_dir, schema.clone()).unwrap();
    }
    let bytes = write_static_from_directory(index_dir.path()).unwrap();

    {
        let mut file = File::create(serialized_path).unwrap();
        file.write_all(&bytes).unwrap();
    }
}

fn main() {
    save_test_index(Path::new("/L/coding-local/tantivy/wasm/tmp/index"));
}


#[cfg(test)]
mod tests {}
