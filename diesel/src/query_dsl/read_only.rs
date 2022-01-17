use crate::{
    backend::Backend,
    connection::read_only::ReadOnly,
    dsl::{Limit, LoadIter},
    expression::{SqlLiteral, UncheckedBind},
    query_builder::{BoxedSqlQuery, SelectStatement, SqlQuery},
    query_dsl::methods::{ExecuteDsl, LimitDsl},
    query_dsl::LoadQuery,
    Connection, QueryResult, RunQueryDsl, Table,
};

/// Marker trait implemented only for SQL statements that are read-only
pub trait ReadOnlyStatement {}

/// A read-only analog to RunQueryDsl
pub trait RunReadOnlyQueryDsl<Conn>: RunQueryDsl<Conn> + ReadOnlyStatement {
    /// Delegates to RunQueryDsl::execute
    fn execute_ro(self, conn: &mut ReadOnly<Conn>) -> QueryResult<usize>
    where
        Conn: Connection,
        Self: ExecuteDsl<Conn>,
    {
        RunQueryDsl::execute(self, &mut conn.0)
    }

    /// Delegates to RunQueryDsl::load
    fn load_ro<'query, U>(self, conn: &mut ReadOnly<Conn>) -> QueryResult<Vec<U>>
    where
        Self: LoadQuery<'query, Conn, U>,
    {
        RunQueryDsl::load(self, &mut conn.0)
    }

    /// Delegates to RunQueryDsl::load_iter
    fn load_iter_ro<'conn, 'query: 'conn, U>(
        self,
        conn: &'conn mut ReadOnly<Conn>,
    ) -> QueryResult<LoadIter<'conn, 'query, Self, Conn, U>>
    where
        U: 'conn,
        Self: LoadQuery<'query, Conn, U> + 'conn,
    {
        RunQueryDsl::load_iter(self, &mut conn.0)
    }

    /// Delegates to RunQueryDsl::get_result
    fn get_result_ro<'query, U>(self, conn: &mut ReadOnly<Conn>) -> QueryResult<U>
    where
        Self: LoadQuery<'query, Conn, U>,
    {
        RunQueryDsl::get_result(self, &mut conn.0)
    }

    /// Delegates to RunQueryDsl::get_results
    fn get_results_ro<'query, U>(self, conn: &mut ReadOnly<Conn>) -> QueryResult<Vec<U>>
    where
        Self: LoadQuery<'query, Conn, U>,
    {
        RunQueryDsl::get_results(self, &mut conn.0)
    }

    /// Delegates to RunQueryDsl::first
    fn first_ro<'query, U>(self, conn: &mut ReadOnly<Conn>) -> QueryResult<U>
    where
        Self: LimitDsl,
        Limit<Self>: LoadQuery<'query, Conn, U>,
    {
        RunQueryDsl::first(self, &mut conn.0)
    }
}

impl<'a, DB: Backend, Query> ReadOnlyStatement for BoxedSqlQuery<'a, DB, Query> {}
impl<From, Select, Distinct, Where, Order, LimitOffset, GroupBy, Having, Locking> ReadOnlyStatement
    for SelectStatement<From, Select, Distinct, Where, Order, LimitOffset, GroupBy, Having, Locking>
{
}
impl<Inner> ReadOnlyStatement for SqlQuery<Inner> {}
impl<Q, V> ReadOnlyStatement for UncheckedBind<Q, V> {}
impl<ST, T> ReadOnlyStatement for SqlLiteral<ST, T> {}
impl<T> ReadOnlyStatement for T where T: Table {}

impl<T: RunQueryDsl<Conn> + ReadOnlyStatement, Conn: Connection> RunReadOnlyQueryDsl<Conn> for T {}
