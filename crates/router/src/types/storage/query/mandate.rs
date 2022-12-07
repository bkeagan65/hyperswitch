use diesel::{associations::HasTable, BoolExpressionMethods, ExpressionMethods};
use error_stack::report;
use router_env::tracing::{self, instrument};

use super::generics::{self, ExecuteQuery};
use crate::{
    connection::PgPooledConn,
    core::errors::{self, CustomResult},
    schema::mandate::dsl,
    types::storage::{self, mandate::*},
};

impl MandateNew {
    #[instrument(skip(conn))]
    pub async fn insert(
        self,
        conn: &PgPooledConn,
    ) -> CustomResult<storage::Mandate, errors::StorageError> {
        generics::generic_insert::<_, _, storage::Mandate, _>(conn, self, ExecuteQuery::new()).await
    }
}

impl Mandate {
    pub async fn find_by_merchant_id_mandate_id(
        conn: &PgPooledConn,
        merchant_id: &str,
        mandate_id: &str,
    ) -> CustomResult<Self, errors::StorageError> {
        generics::generic_find_one::<<Self as HasTable>::Table, _, _>(
            conn,
            dsl::merchant_id
                .eq(merchant_id.to_owned())
                .and(dsl::mandate_id.eq(mandate_id.to_owned())),
        )
        .await
    }

    pub async fn find_by_merchant_id_customer_id(
        conn: &PgPooledConn,
        merchant_id: &str,
        customer_id: &str,
    ) -> CustomResult<Vec<Self>, errors::StorageError> {
        generics::generic_filter::<<Self as HasTable>::Table, _, _>(
            conn,
            dsl::merchant_id
                .eq(merchant_id.to_owned())
                .and(dsl::customer_id.eq(customer_id.to_owned())),
            None,
        )
        .await
    }

    pub async fn update_by_merchant_id_mandate_id(
        conn: &PgPooledConn,
        merchant_id: &str,
        mandate_id: &str,
        mandate: MandateUpdate,
    ) -> CustomResult<Self, errors::StorageError> {
        generics::generic_update_with_results::<<Self as HasTable>::Table, _, _, Self, _>(
            conn,
            dsl::merchant_id
                .eq(merchant_id.to_owned())
                .and(dsl::mandate_id.eq(mandate_id.to_owned())),
            MandateUpdateInternal::from(mandate),
            ExecuteQuery::new(),
        )
        .await?
        .first()
        .cloned()
        .ok_or_else(|| {
            report!(errors::StorageError::DatabaseError(
                errors::DatabaseError::NotFound
            ))
            .attach_printable("Error while updating mandate")
        })
    }
}