// @generated automatically by Diesel CLI.

diesel::table! {
    stock (name) {
        name -> Varchar,
        vendor -> Varchar,
        quantity -> Int4,
    }
}
