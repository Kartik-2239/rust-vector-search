use crate::utils::search_db;

pub fn query_table(q: &str, k: usize, table_path: &str){
    let mut returned_stuff = search_db(q, table_path).unwrap();
    returned_stuff.sort_by_key(|r| - (r.3*1000000.0) as i32);
    println!("Top K results for query `{}`", q);
    for (idx, (_,t,_,_)) in returned_stuff[0..k].iter().enumerate() {
        println!("rank {} -> {}", idx+1, t.trim());
    }
}