// @generated automatically by Diesel CLI.

diesel::table! {
    transactions (id) {
        id -> Uuid,
        transaction_value -> Nullable<Float8>,
        product_description -> Nullable<Text>,
        card_number -> Nullable<Int8>,
        #[max_length = 50]
        name_in_card -> Nullable<Varchar>,
        card_expiration_date -> Nullable<Date>,
        cvv -> Nullable<Int4>,
    }
}
