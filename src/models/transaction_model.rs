use diesel::prelude::*;

#[derive!(Queryable, Selectable)]
#[diesel(table_name = create::schema::transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Transactions {
    transaction_value: f64, //float 64 bits
    product_description: String,
    payment_method: String,
    card_number: u64,
    name_in_card: String,
    card_expiration_date: String,
    cvv: u8
}