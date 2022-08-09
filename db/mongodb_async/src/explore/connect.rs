use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}

#[cfg(test)]
mod tests {
    use futures::TryStreamExt;
    use mongodb::bson::{doc, Document};
    use mongodb::Client;
    use mongodb::options::{ClientOptions, FindOptions};
    use crate::explore::connect::Book;

    #[actix_rt::test]
    async fn test_connect() -> Result<(), mongodb::error::Error>{
        // Parse a connection string into an options struct.
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

        // Manually set an option.
        client_options.app_name = Some("My App".to_string());

        // Get a handle to the deployment.
        let client = Client::with_options(client_options)?;

        // List the names of the databases in that deployment.
        for db_name in client.list_database_names(None, None).await? {
            println!("{}", db_name);
        }

        // Get a handle to a database.
        let db = client.database("mydb");

        // List the names of the collections in that database.
        for collection_name in db.list_collection_names(None).await? {
            println!("{}", collection_name);
        }

        let collection = db.collection::<Document>("books");

        let docs = vec![
            doc! { "title": "1984", "author": "George Orwell" },
            doc! { "title": "Animal Farm", "author": "George Orwell" },
            doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
        ];

        // Insert some documents into the "mydb.books" collection.
        collection.insert_many(docs, None).await?;

        let typed_collection = db.collection::<Book>("books");

        let books = vec![
            Book {
                title: "The Grapes of Wrath".to_string(),
                author: "John Steinbeck".to_string(),
            },
            Book {
                title: "To Kill a Mockingbird".to_string(),
                author: "Harper Lee".to_string(),
            },
        ];
        // Insert the books into "mydb.books" collection, no manual conversion to BSON necessary.
        let a = typed_collection.insert_many(books, None).await?;
        println!("{:?}", a);

        // This trait is required to use `try_next()` on the cursor
        use futures::stream::TryStreamExt;
        use mongodb::{bson::doc, options::FindOptions};

        let filter = doc! { "author": "George Orwell" };
        let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
        let mut cursor = typed_collection.find(filter, find_options).await?;

// Iterate over the results of the cursor.
        while let Some(book) = cursor.try_next().await? {
            println!("title: {}", book.title);
        }
        Ok(())
    }

    #[actix_rt::test]
    async fn test_cursor_try_next() -> Result<(), mongodb::error::Error>{
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
        client_options.app_name = Some("My App".to_string());
        let client = Client::with_options(client_options)?;
        let db = client.database("mydb");
        let typed_collection = db.collection::<Book>("books");
        let filter = doc! { "author": "George Orwell" };
        let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
        let mut cursor = typed_collection.find(filter, find_options).await?;
        while let Some(book) = cursor.try_next().await? {
            println!("title: {:?}", book);
        }
        Ok(())
    }
}