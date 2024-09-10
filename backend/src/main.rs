use axum::{
    extract::Query,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Pagination {
    page: usize,
    per_page: usize,
}

#[derive(Serialize)]
struct Item {
    id: usize,
    name: String,
}

async fn list_things(pagination: Query<Pagination>) -> String {
    let Pagination { page, per_page } = pagination.0;

    // Dummy data generation
    let total_items = 100000; // Suppose we have 100 items in total
    let items: Vec<Item> = (1..=total_items)
        .map(|id| Item {
            id,
            name: format!("Item {}", id),
        })
        .collect();

    // Pagination logic
    let start = (page - 1) * per_page;
    let end = start + per_page;
    let paginated_items = &items[start..end.min(total_items)];

    format!(
        "Page: {}, Per Page: {}, Items: {:?}",
        page,
        per_page,
        paginated_items
            .iter()
            .map(|item| &item.name)
            .collect::<Vec<_>>()
    )
}

#[tokio::main]
async fn main() {
    let app: Router = Router::new().route("/list_things", get(list_things));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
