use crate::schema::stock;
use diesel::prelude::*;

#[derive(Debug)]
#[derive(Insertable)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = stock)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StockItem {
    pub name: String,
    pub vendor: String,
    pub quantity: i32,
}
