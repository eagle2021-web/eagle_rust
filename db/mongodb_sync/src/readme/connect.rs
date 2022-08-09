use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}

#[cfg(test)]
mod tests {
    use mongodb::bson::doc;
    use mongodb::sync::Client;
    use crate::readme::connect::Book;

    #[test]
    fn test_connect() -> Result<(), mongodb::error::Error>{
        let client = Client::with_uri_str("mongodb://localhost:27017")?;
        let database = client.database("mydb");
        let collection = database.collection::<Book>("books");

        let docs = vec![
            Book {
                title: "1984".to_string(),
                author: "George Orwell".to_string(),
            },
            Book {
                title: "Animal Farm".to_string(),
                author: "George Orwell".to_string(),
            },
            Book {
                title: "The Great Gatsby".to_string(),
                author: "F. Scott Fitzgerald".to_string(),
            },
        ];
        collection.insert_many(docs, None)?;

        let cursor = collection
            .find(doc! { "author": "George Orwell" }, None)?;
        for result in cursor {
            println!("title: {}", result?.title);
        }


        Ok(())
    }
}