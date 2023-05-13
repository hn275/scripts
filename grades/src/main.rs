use mongodb;
use serde;
use tokio;

mod database;

#[tokio::main]
async fn main() -> Result<(), mongodb::error::Error> {
    let db = database::new().await?.collection::<Grade>("test");

    let test_grade: Grade = Grade {
        title: "tes",
        weight: 20.0,
        score: 12.0,
        sub: "seng265",
    };

    let result = db
        .insert_one(test_grade, None)
        .await
        .map_err(|err| panic!("{}", err));

    Ok(())
}

#[derive(serde::Serialize)]
struct Grade<'a> {
    title: &'a str,
    sub: &'a str,
    weight: f32,
    score: f32,
}
