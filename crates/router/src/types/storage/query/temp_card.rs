use diesel::{associations::HasTable, ExpressionMethods};
use router_env::tracing::{self, instrument};

use super::generics::{self, ExecuteQuery};
use crate::{
    connection::PgPooledConn,
    core::errors::{self, CustomResult},
    schema::temp_card::dsl,
    types::storage::{TempCard, TempCardNew},
};

impl TempCardNew {
    #[instrument(skip(conn))]
    pub async fn insert_diesel(
        self,
        conn: &PgPooledConn,
    ) -> CustomResult<TempCard, errors::StorageError> {
        generics::generic_insert::<_, _, TempCard, _>(conn, self, ExecuteQuery::new()).await
    }
}

impl TempCard {
    #[instrument(skip(conn))]
    pub async fn insert_with_token(
        self,
        conn: &PgPooledConn,
    ) -> CustomResult<Self, errors::StorageError> {
        generics::generic_insert::<_, _, TempCard, _>(conn, self, ExecuteQuery::new()).await
    }

    #[instrument(skip(conn))]
    pub async fn find_by_transaction_id(
        conn: &PgPooledConn,
        transaction_id: &str,
    ) -> CustomResult<Option<TempCard>, errors::StorageError> {
        generics::generic_find_one_optional::<<Self as HasTable>::Table, _, _>(
            conn,
            dsl::txn_id.eq(transaction_id.to_owned()),
        )
        .await
    }

    #[instrument(skip(conn))]
    pub async fn find_by_token(
        conn: &PgPooledConn,
        token: &i32,
    ) -> CustomResult<Self, errors::StorageError> {
        generics::generic_find_one::<<Self as HasTable>::Table, _, _>(
            conn,
            dsl::id.eq(token.to_owned()),
        )
        .await
    }
}