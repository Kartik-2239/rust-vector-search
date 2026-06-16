# Rust RAG (Retrieval-Augmented Generation)

A dead simple vector search in rust, stores embeddings in sqlite with text and runs through all the rows when queried 
the database was made with rust through "fastembed".

## Usage

### 1. Show Help
```bash
cargo run -- --help
```

### 2. Embed a Document
```bash
cargo run -- --embed <PATH_TO_TEXT_FILE> --output <PATH_TO_DB_FILE>
```
*Example:*
```bash
cargo run -- --embed documents.txt --output db.sqlite
```

### 3. Query the Database
```bash
cargo run -- --query "<YOUR_QUERY>" --path <PATH_TO_DB_FILE> [--k <LIMIT>]
```

### Query a without creating a Database
```bash
cargo run -- --query "<YOUR_QUERY>" --embed <PATH_TO_TEXT_FILE> [--k <LIMIT>]
```