use bgg_rs::{client::BggClient, query::{QueryParams, make_reqwest_query}, response::Item};

fn println(items: &Vec<Item>) {
    for item in items {
        println!("{:?}", item);
    }
}

#[tokio::main]
async fn main() {
    let client = BggClient::new();

    let query = make_reqwest_query(QueryParams::new().title("root").yearpublished_min(2000));
    let res = client.adv_search(&query).await;
    match res {
        Ok(r) => println(&r["items"]),
        Err(err) => eprintln!("{:?}", err),
    }
}
