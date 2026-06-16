use std::{fs, vec};
use anyhow::Result;
use fastembed::{TextEmbedding, InitOptions, EmbeddingModel};
use rusqlite::{Connection, params};

pub fn read_file(path: &str) -> Result<String, std::io::Error>{
    let data = fs::read_to_string(path)?;
    Ok(data)
}

pub fn chunk(data: &str, pattern: &str) -> Vec<String>{
    let mut v = Vec::new();
    let sentences = data.split(pattern);
    for sentence in sentences {
        if sentence.len() > 0{
            v.push(sentence.to_string());
        }

    }
    return v
}

pub fn embed(chunk: Vec<String>) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>>{
    let mut model = TextEmbedding::try_new(
        InitOptions::new(EmbeddingModel::BGESmallENV15)
            .with_show_download_progress(true)
    )?;
    let embeddings = model.embed(chunk, None)?;
    Ok(embeddings)
}

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let mut numerator: f32 = 0.0;
    let mut first: f32 = 0.0;
    let mut second: f32 = 0.0;
    for i in 0..a.len(){
        numerator += a[i]*b[i];
        first += a[i]*a[i];
        second += b[i]*b[i];
    }
    let denominator = first.sqrt() * second.sqrt();

    if denominator == 0.0{
        return 0.0;
    }

    return  numerator/denominator;
}

pub fn update_db_batch(chunks: &[String], embeddings: &[Vec<f32>], db_path: &str)->Result<()>{
    let mut conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS chunks (
            id INTEGER PRIMARY KEY,
            text TEXT NOT NULL,
            embedding BLOB NOT NULL
        );",
        [],
    )?;
    let tx = conn.transaction()?;
    {
        let mut stmt = tx.prepare("INSERT INTO chunks (text, embedding) VALUES (?1, ?2)")?;
        for (text, embedding) in chunks.iter().zip(embeddings.iter()) {
            let embedding_bytes = bincode::encode_to_vec(embedding, bincode::config::standard()).unwrap();
            stmt.execute(params![text, embedding_bytes])?;
        }
    }
    tx.commit()?;
    Ok(())
}

pub fn get_all_rows(db_path: &str)->Result<Vec<(i64, String, Vec<f32>)>, Box<dyn std::error::Error>>{
    let conn = Connection::open(db_path)?;

    let mut stmt = conn.prepare(
        "SELECT id, text, embedding FROM chunks"
    )?;

    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, Vec<u8>>(2)?,
        ))
    })?;

    let mut ret: Vec<(i64, String, Vec<f32>)> = vec![];

    for row in rows {
        let (id, text, embedding_bytes) = row?;
        let embeds = bincode::decode_from_slice(&embedding_bytes, bincode::config::standard())?.0;
        ret.push((id,text,embeds));
    }

    Ok(ret)
}


pub fn search_db(query: &str, db_path: &str) -> Result<Vec<(i64, String, Vec<f32>, f32)>, Box<dyn std::error::Error>>{
    let q = embed(vec![query.to_string()])?;
    let all_vecs = get_all_rows(db_path)?;
    let mut ret = vec![];
    for (id, text, embedding) in all_vecs {
        let similarity = cosine_similarity(&q[0], &embedding);
        ret.push((id, text, embedding, similarity));
    }
    Ok(ret)
}