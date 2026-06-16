use crate::utils::read_file;
use crate::utils::chunk;
use crate::utils::embed;
use crate::utils::update_db_batch;



pub fn embed_file(data_path: &str, db_path: &str){
    let data = read_file(data_path).expect("No file found");
    let chunks = chunk(&data, ".");
    let slice = &chunks;
    let embeddings = embed(slice.to_vec()).expect("Embedding error");
    let _ = update_db_batch(&chunks, &embeddings, db_path).expect("file should save");
    println!("Created embeddings from `{}` into `{}`", data_path.split("/").last().unwrap(), db_path.split("/").last().unwrap())
}