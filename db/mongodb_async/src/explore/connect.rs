use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}

fn rand_word() -> String {
    let mut rng = rand::thread_rng();
    let n = rng.gen::<u8>() % 20 + 1;
    let mut v = vec![0_u8; n as usize];
    for v in v.iter_mut() {
        let cur = rng.gen::<u8>() % 26;
        *v = 'a' as u8 + cur;
    }
    String::from_utf8(v).unwrap()
}
fn rand_age() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen::<u8>() % 128 + 1
}
#[cfg(test)]
mod tests {
    use futures::TryStreamExt;
    use mongodb::bson::{doc, Document};
    use mongodb::Client;
    use mongodb::options::{ClientOptions, FindOneOptions, FindOptions, UpdateOptions};
    use rand::Rng;
    use crate::explore::connect::{Book, rand_age, rand_word};

    #[actix_rt::test]
    async fn test_connect() -> Result<(), mongodb::error::Error> {
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
    async fn test_cursor_try_next() -> Result<(), mongodb::error::Error> {
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
        client_options.app_name = Some("My App".to_string());
        let client = Client::with_options(client_options)?;
        let db = client.database("mydb");
        let typed_collection = db.collection::<Book>("books");
        let filter = doc! { "author": "George Orwell" };
        let find_options = FindOptions::builder()
            .sort(doc! { "title": 1 }).limit(20).build();

        let mut cursor = typed_collection.find(filter, find_options).await?;
        while let Some(book) = cursor.try_next().await? {
            println!("title: {:?}", book);
        }
        Ok(())
    }

    #[actix_rt::test]
    async fn test_insert_many() -> Result<(), mongodb::error::Error> {
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
        client_options.app_name = Some("My App".to_string());
        let client = Client::with_options(client_options)?;
        let db = client.database("mydb");
        let food = db.collection::<Document>("food");
        let arr = vec![
            doc! {"fruit": ["apple", "banana", "peach"]},
            doc! {"fruit": ["apple", "kumquat", "orange"]},
            doc! {"fruit": ["cherry", "banana", "apple"]},
        ];
        food.insert_many(arr, None).await?;
        Ok(())
    }

    #[actix_rt::test]
    async fn test_find_one() -> Result<(), mongodb::error::Error> {
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
        client_options.app_name = Some("My App".to_string());
        let client = Client::with_options(client_options)?;
        let db = client.database("mydb");
        let food = db.collection::<Document>("food");
        let find_options = FindOneOptions::builder()
            .return_key(false)
            .projection(doc! {"_id": 0})
            .build();
        let res = food.find_one(doc! {"fruit": "banana"}, find_options)
            .await?;
        if let Some(d) = &res {
            println!("{}", d);
        }
        Ok(())
    }

    #[actix_rt::test]
    async fn test_update_one() -> Result<(), mongodb::error::Error> {
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
        client_options.app_name = Some("My App".to_string());
        let client = Client::with_options(client_options)?;
        let db = client.database("mydb");
        let food = db.collection::<Document>("food");
        let options = UpdateOptions::builder()
            .upsert(true)
            .build();
        let d = food.update_one(doc! {
            "fruit": ["apple", "banana", "peach2"]
        }, doc! {
            "$set": {
                "fruit": "watermelon"
            }
        }, options).await?;
        println!("{:?}", d);
        Ok(())
    }

    #[actix_rt::test]
    async fn test_update_many() -> Result<(), mongodb::error::Error> {
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
        client_options.app_name = Some("My App".to_string());
        let client = Client::with_options(client_options)?;
        let db = client.database("mydb");
        let food = db.collection::<Document>("food");
        let options = UpdateOptions::builder()
            .upsert(true)
            .build();
        let d = food.update_many(doc! {
            "name": {
                "$regex": "^eagle.*$"
            }
        }, doc! {
            "$set": {
                "name": "eagle",
                "fruit": "watermelon_eagle_update_many"
            }
        }, options).await?;
        println!("{:?}", d);
        Ok(())
    }

    #[test]
    fn test_rand_word() {
        for _i in 0..1000_000 {
            let s = rand_word();
            assert!(s.len() > 0);
            assert!(s.len() <= 20);
            s.as_bytes().into_iter()
                .for_each(|v|assert!(*v >= 'a' as u8 && *v <= 'z' as u8));
        }
    }

    #[test]
    fn test_rand_age() {
        for _ in 0..1000_000 {
            let age = rand_age();
            assert!(age <= 128);
            assert!(age >= 1);
        }
    }
}