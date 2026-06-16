use crate::utils::read_file;
use crate::utils::chunk;
use crate::utils::embed;
use crate::utils::cosine_similarity;



pub fn db_less_query(data_path: &str, query: &str, k: usize)-> Result<(), Box<dyn std::error::Error>>{
    let data = read_file(data_path).expect("No file found");
    let chunks = chunk(&data, ".");
    let slice = &chunks;
    let embeddings = embed(slice.to_vec()).expect("Embedding error");

    let mut returned_stuff: Vec<(i64, String, Vec<f32>, f32)> = vec![];
    let embedded_q = embed(vec![query.to_string()])?;
    for (idx, embedding) in embeddings.iter().enumerate(){
        let similarity = cosine_similarity(&embedded_q[0], &embedding);
        returned_stuff.push((1, (&chunks[idx]).to_string(), embedding.to_owned(), similarity))
    }
    returned_stuff.sort_by_key(|r| - (r.3*1000000.0) as i32);
    println!("Top K results for query `{}`", query);
    for (idx, (_,t,_,_)) in returned_stuff[0..k].iter().enumerate() {
        println!("rank {} -> {}", idx+1, t.trim());
    }
    Ok(())
}