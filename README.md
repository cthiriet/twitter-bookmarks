# Twitter Bookmarks Indexer 🐦‍

If you are seeing this page, you likely have many Twitter bookmarks and are seeking a way to search through them.

This tool is a simple indexer for your Twitter bookmarks.

It uses:

- A Rust script for retrieving your Twitter bookmarks
- [Meiliseach](https://github.com/meilisearch/meilisearch) for indexing your bookmarks and offering a web interface for searching them

## How to use

1. Clone this repository

2. Fill env variables

Copy `.env.example` to `.env` and fill the variables.

In these screenshots, you will see the 4 env variables that need to be set and their location within the Chrome Network Inspector.

![env1](assets/1.png)

![env2](assets/2.png)

3. Launch meilisearch

```bash
docker compose up -d
```

4. Index your bookmarks

```bash
cargo run --release
```

The script will also save the bookmarks in a `bookmarks.json` file.

5. Search your bookmarks

Open [`http://localhost:7700`](http://localhost:7700) in your browser.
