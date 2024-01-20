use epub::doc::EpubDoc;
use scraper::Html;

fn main() -> anyhow::Result<()> {
    let mut doc = EpubDoc::new("./files/Hamlet.epub")?;
    let title = doc.mdata("title");
    let author = doc.mdata("creator");
    println!("Title  {:?}", title);
    println!("Author  {:?}", author);
    println!("pages  {:?}", doc.get_num_pages());

    while doc.go_next() {
        let id = doc.get_current_id().unwrap();
        println!("ID: {}", id);
        let current = doc.get_current_str();
        match current {
            Some((v, m)) => {
                let document = Html::parse_document(&v);
                let res = document
                    .tree
                    .into_iter()
                    .map(|text| {
                        if let scraper::node::Node::Text(text) = text {
                            text.text.to_string()
                        } else {
                            String::new()
                        }
                    })
                    .collect::<String>();
                if id != String::from("pg-footer")
                    && id != String::from("pg-header")
                    && id != String::from("item9")
                {
                    println!("Value {:?}, Mime {:?}\n", res, m)
                }
                // println!("Value {:?}, Mime {:?}\n", res, m)
            }
            None => println!("Not Found\n"),
        }
    }

    Ok(())
}
