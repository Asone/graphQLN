pub use crate::db::schema::payment;
use crate::lnd::invoice::LndInvoice;
use chrono::NaiveDateTime;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use uuid::Uuid;

#[derive(Queryable, PartialEq, Associations)]
#[table_name = "payment"]
#[belongs_to(parent = Post, foreign_key = "post_uuid")]
pub struct Payment {
    pub uuid: Uuid,
    pub request: String,
    pub state: Option<String>,
    pub hash: String,
    pub post_uuid: Uuid,
    pub expires_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "payment"]
pub struct NewPayment {
    uuid: Uuid,
    hash: String,
    request: String,
    post_uuid: Uuid,
    expires_at: NaiveDateTime,
}

impl From<(LndInvoice, uuid::Uuid)> for NewPayment {
    fn from(data: (LndInvoice, uuid::Uuid)) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            hash: data.0.r_hash,
            request: data.0.payment_request,
            post_uuid: data.1,
            expires_at: data.0.expires_at,
        }
    }
}

impl Payment {
    pub fn find_one_by_request(
        payment_request: String,
        connection: &PgConnection,
    ) -> Option<Payment> {
        use crate::db::schema::payment::dsl::*;
        payment
            .filter(request.eq(payment_request))
            .first::<Payment>(connection)
            .optional()
            .unwrap()
    }

    pub fn create(new_payment: NewPayment, connection: &PgConnection) -> QueryResult<Payment> {
        use crate::db::schema::payment::dsl::*;

        diesel::insert_into::<payment>(payment)
            .values(&new_payment)
            .get_result(connection)
    }
}
