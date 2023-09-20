mod models;
mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use self::schema::stock::dsl::*;
use self::models::*;

pub fn establish_connection() -> PgConnection {
     let database_url = "postgres://postgres:pass@127.0.0.1/postgres";
     PgConnection::establish(&database_url)
         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    let connection = &mut establish_connection();
    
    let new_item1 = StockItem { name: "Toothbrush".to_string(), vendor: "Limo".to_string(), quantity: 1497 };
    let new_item2 = StockItem { name: "Comb".to_string(), vendor: "Takoon".to_string(), quantity: 210 };
    let new_item3 = StockItem { name: "Towel".to_string(), vendor: "Beana".to_string(), quantity: 362 };
    
    diesel::insert_into(self::schema::stock::table)
        .values([new_item1, new_item2, new_item3])
        .returning(StockItem::as_returning())
        .get_results(connection)
        .expect("Error saving new stock item");
    
    let results = stock
        .filter(quantity.ge(300))
        .select(StockItem::as_select())
        .load(connection)
        .expect("Error loading stock items");

    for item in results {
        println!("{:?}", item);
    }
}
