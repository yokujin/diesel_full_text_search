#[macro_use] extern crate diesel;

mod types {
    #[allow(deprecated)]
    use diesel::SqlType;

    #[derive(Clone, Copy, SqlType)]
    #[postgres(oid = "3615", array_oid = "3645")]
    pub struct Tsquery;

    #[derive(Clone, Copy, SqlType)]
    #[postgres(oid = "3614", array_oid = "3643")]
    pub struct Tsvector;

    #[derive(SqlType)]
    #[postgres(type_name = "regconfig")]
    pub struct Regconfig;
}

#[allow(deprecated)]
mod functions {
    use types::*;
    use diesel::sql_types::*;

    sql_function!(fn length(x: Tsvector) -> Integer);
    sql_function!(fn numnode(x: Tsquery) -> Integer);
    sql_function!(fn plainto_tsquery(x: Text) -> Tsquery);
    sql_function!(fn querytree(x: Tsquery) -> Text);
    sql_function!(fn strip(x: Tsvector) -> Tsvector);
    sql_function!(fn to_tsquery(x: Text) -> Tsquery);
    sql_function! {
        #[sql_name = "to_tsquery"]
        fn to_tsquery_with_search_config(config: Regconfig, querytext: Text) -> Tsquery;
    }
    sql_function!(fn to_tsvector(x: Text) -> Tsvector);
    sql_function! {
        #[sql_name = "to_tsvector"]
        fn to_tsvector_with_search_config(config: Regconfig, document_content: Text) -> Tsvector;
    }
    sql_function!(fn ts_headline(x: Text, y: Tsquery) -> Text);
    sql_function!(fn ts_rank(x: Tsvector, y: Tsquery) -> Float);
    sql_function!(fn ts_rank_cd(x: Tsvector, y: Tsquery) -> Float);
    sql_function!(fn phraseto_tsquery(x: Text) -> Tsquery);
    sql_function!(fn websearch_to_tsquery(x: Text) -> Tsquery);
}

mod dsl {
    use types::*;
    use diesel::expression::{Expression, AsExpression};
    use diesel::expression::grouped::Grouped;

    mod predicates {
        use types::*;
        use diesel::pg::Pg;

        diesel_infix_operator!(Matches, " @@ ", backend: Pg);
        diesel_infix_operator!(Concat, " || ", Tsvector, backend: Pg);
        diesel_infix_operator!(And, " && ", Tsquery, backend: Pg);
        diesel_infix_operator!(Or, " || ", Tsquery, backend: Pg);
        diesel_infix_operator!(Contains, " @> ", backend: Pg);
        diesel_infix_operator!(ContainedBy, " <@ ", backend: Pg);
    }

    use self::predicates::*;

    pub type Concat<T, U> = Grouped<predicates::Concat<T, U>>;

    pub trait TsvectorExtensions: Expression<SqlType=Tsvector> + Sized {
        fn matches<T: AsExpression<Tsquery>>(self, other: T) -> Matches<Self, T::Expression> {
            Matches::new(self, other.as_expression())
        }

        fn concat<T: AsExpression<Tsvector>>(self, other: T) -> Concat<Self, T::Expression> {
            Grouped(predicates::Concat::new(self, other.as_expression()))
        }
    }

    pub trait TsqueryExtensions: Expression<SqlType=Tsquery> + Sized {
        fn matches<T: AsExpression<Tsvector>>(self, other: T) -> Matches<Self, T::Expression> {
            Matches::new(self, other.as_expression())
        }

        fn and<T: AsExpression<Tsquery>>(self, other: T) -> And<Self, T::Expression> {
            And::new(self, other.as_expression())
        }

        fn or<T: AsExpression<Tsquery>>(self, other: T) -> Or<Self, T::Expression> {
            Or::new(self, other.as_expression())
        }

        fn contains<T: AsExpression<Tsquery>>(self, other: T) -> Contains<Self, T::Expression> {
            Contains::new(self, other.as_expression())
        }

        fn contained_by<T: AsExpression<Tsquery>>(self, other: T) -> ContainedBy<Self, T::Expression> {
            ContainedBy::new(self, other.as_expression())
        }
    }

    impl<T: Expression<SqlType=Tsvector>> TsvectorExtensions for T {
    }

    impl<T: Expression<SqlType=Tsquery>> TsqueryExtensions for T {
    }
}

pub use self::types::*;
pub use self::functions::*;
pub use self::dsl::*;
