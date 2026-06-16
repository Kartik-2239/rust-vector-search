use rust_rag::dblessquery::db_less_query;
use rust_rag::query::query_table;
use rust_rag::embed::embed_file;
use clap::Parser;

#[derive(Parser)]
#[command(disable_help_flag = true)]
struct Cli {
    #[arg(long)]
    embed: Option<String>,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(short, long)]
    query: Option<String>,

    #[arg(long)]
    path: Option<String>,

    #[arg(long)]
    k: Option<usize>,

    #[arg(short, long)]
    help: bool,
}

fn print_help() {
    println!("Rust Vector Search");
    println!();
    println!("Usage:");
    println!("  Embed/Index a document file:");
    println!("    cargo run -- --embed <FILE_PATH> --output <DB_PATH>");
    println!();
    println!("  Query the indexed database:");
    println!("    cargo run -- --query <QUERY_STRING> --path <DB_PATH> [--k <LIMIT>]");
    println!();
    println!("  Query without saving the indexed database:");
    println!("    cargo run -- --query <QUERY_STRING> --path <DB_PATH> [--k <LIMIT>]");
    println!();
    println!("Options:");
    println!("  --embed <FILE_PATH>    Path to the text source file to be embedded");
    println!("  --output <DB_PATH>     Path to output SQLite database file where embeddings are saved");
    println!("  --query <QUERY>        The question/query search text");
    println!("  --path <DB_PATH>       Path to input/existing SQLite database file to search");
    println!("  --k <LIMIT>           The number of top matching results to retrieve (default: 10)");
    println!("  -h, --help             Show this help message");
}

fn main() {
    let cli = Cli::parse();
    let mut top_k: Option<usize> = None;

    if cli.help || std::env::args().len() == 1 {
        print_help();
        return;
    }
    if let Some(k) = cli.k {
        top_k = Some(k);
    }

    if let Some(output) = cli.output && let Some(embed) = cli.embed {
        embed_file(&embed, &output);
        return;
    }
    if let Some(query) = cli.query.as_deref() && let Some(path) = cli.path {
        if top_k.is_none(){
            top_k = Some(10);
        }
        query_table(&query, top_k.expect("value of k"), &path);
        return;
    }
    if let Some(query) = cli.query.as_deref() && let Some(embed) = cli.embed {
        if top_k.is_none(){
            top_k = Some(10);
        }
        let _ = db_less_query(&embed, query, top_k.expect(""));
        return;
    }
}
