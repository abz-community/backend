use anyhow::Context;
use dotenv::dotenv;
use epub::doc::EpubDoc;
use futures::future::join_all;
use html_parser::{Dom, Node};
use std::{
    fs::{create_dir_all, File},
    path::PathBuf,
    sync::{
        atomic::{AtomicU16, AtomicUsize},
        Arc,
    },
};
use tokio::{fs::File as TokioFile, io::{AsyncReadExt, AsyncSeekExt}};
use tokio::io::AsyncWriteExt;
use tokio::task::JoinHandle;

pub struct FileData {
    pub author: String,
    pub book_name: String,
    pub path: String,
}

pub struct LocalStorage {
    path: String,
    page_length: usize,
}

impl LocalStorage {
    pub fn new() -> Self {
        dotenv().ok();
        let path = std::env::var("LOCAL_STORAGE_PATH").expect("LOCAL_STORAGE_PATH must be set");
        let page_length = std::env::var("PAGE_LENGTH").expect("PAGE_LENGTH must be set");
        Self {
            path,
            page_length: page_length.parse().unwrap(),
        }
    }

    pub async fn parse_to_txt(&self, file: File, ext: String) -> Result<FileData, anyhow::Error> {
        let content_type = ext.split('.').last().context("Filename has no extension")?;
        match content_type {
            "epub" => self.epub_parse(file).await,
            _ => Err(anyhow::anyhow!(
                "Cannot parse this content type: {}",
                content_type
            )),
        }
    }

    async fn epub_parse(&self, file: File) -> anyhow::Result<FileData> {
        let mut doc =
            EpubDoc::from_reader(file).map_err(|e| anyhow::anyhow!("Parsing error: {e}"))?;
        let title = doc.mdata("title").context("No title")?;
        let author = doc.mdata("creator").context("No author")?;
        let path = Arc::new(format!("{}/{} {}", self.path, author, title));
        // TODO add normalization of chunks by summaring last chunk to given page_length
        // let arbitary = String::new();
        let id_counter = AtomicUsize::new(0);

        loop {
            let current = doc.get_current_str();
            let a_path = path.clone();
            match current {
                Some((v, _)) => {
                    tokio::spawn(Self::save_chapter(
                        id_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
                        v,
                        a_path,
                    ));
                }
                None => println!("Not Found\n"),
            }
            if !doc.go_next() {
                break;
            }
        }
        Ok(FileData {
            author,
            book_name: title,
            path: path.to_string(),
        })
    }

    // TODO rewrite to save bytes
    async fn save_chapter(count: usize, text: String, path: Arc<String>) -> anyhow::Result<()> {
        let dom = Dom::parse(&text)?;
        let text = dom
            .children
            .into_iter()
            .map(|n| Self::parse_node(n))
            .collect::<Vec<String>>()
            .join("");
        Self::write_chunk(path.clone(), count, text).await?;
        Ok(())
    }

    fn parse_node(node: Node) -> String {
        match node {
            Node::Text(t) => t,
            Node::Element(e) => e
                .children
                .into_iter()
                .map(|n| Self::parse_node(n))
                .collect::<Vec<String>>()
                .join(""),
            _ => String::new(),
        }
    }

    // TODO rewrite to file.seek()
    pub async fn read_from(
        &self,
        book_path: String,
        chapter: i32,
        from_char: i32,
        amount: usize,
    ) -> anyhow::Result<String> {
        let text = tokio::fs::read_to_string(format!("{}/{}/{}.txt", self.path, book_path, chapter)).await?;
        Ok(text[from_char as usize..amount].to_string())
    }

    async fn write_chunk(path: Arc<String>, id: usize, chunk: String) -> anyhow::Result<()> {
        create_dir_all(path.as_str())?;
        let mut file = TokioFile::create(format!("{}/{}.txt", path, id)).await?;
        file.write_all(chunk.as_bytes()).await?;
        Ok(())
    }
}
