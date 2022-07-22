#![allow(dead_code)]

use std::time::SystemTime;

#[derive(FromSqlRow, AsExpression)]
#[diesel(foreign_derive)]
#[sql_type = "::sql_types::Timestamp"]
struct SystemTimeProxy(SystemTime);

#[cfg(feature = "chrono")]
mod chrono {
    extern crate chrono;
    use self::chrono::*;
    use sql_types::{Date, Time, Timestamp};

    #[derive(FromSqlRow, AsExpression)]
    #[diesel(foreign_derive)]
    #[sql_type = "Date"]
    struct NaiveDateProxy(NaiveDate);

    #[derive(FromSqlRow, AsExpression)]
    #[diesel(foreign_derive)]
    #[sql_type = "Time"]
    struct NaiveTimeProxy(NaiveTime);

    #[derive(FromSqlRow, AsExpression)]
    #[diesel(foreign_derive)]
    #[sql_type = "Timestamp"]
    #[cfg_attr(feature = "postgres", sql_type = "::sql_types::Timestamptz")]
    #[cfg_attr(feature = "mysql", sql_type = "::sql_types::Datetime")]
    struct NaiveDateTimeProxy(NaiveDateTime);

    #[derive(FromSqlRow, AsExpression)]
    #[diesel(foreign_derive)]
    #[cfg_attr(feature = "postgres", sql_type = "::sql_types::Timestamptz")]
    struct DateTimeProxy<Tz: TimeZone>(DateTime<Tz>);
}

#[cfg(feature = "time")]
mod time {
    use time::{Date as NaiveDate, OffsetDateTime, PrimitiveDateTime, Time as NaiveTime};

    use crate::deserialize::FromSqlRow;
    use crate::expression::AsExpression;
    use crate::sql_types::{Date, Time, Timestamp};

    #[derive(AsExpression, FromSqlRow)]
    #[diesel(foreign_derive)]
    #[diesel(sql_type = Date)]
    struct NaiveDateProxy(NaiveDate);

    #[derive(AsExpression, FromSqlRow)]
    #[diesel(foreign_derive)]
    #[diesel(sql_type = Time)]
    struct NaiveTimeProxy(NaiveTime);

    #[derive(AsExpression, FromSqlRow)]
    #[diesel(foreign_derive)]
    #[diesel(sql_type = Timestamp)]
    #[cfg_attr(
        feature = "postgres_backend",
        diesel(sql_type = crate::sql_types::Timestamptz)
    )]
    // #[cfg_attr(feature = "mysql_backend", diesel(sql_type = crate::sql_types::Datetime))]
    struct NaiveDateTimeProxy(PrimitiveDateTime);

    #[derive(FromSqlRow)]
    #[diesel(foreign_derive)]
    #[cfg_attr(
        any(feature = "postgres_backend", feature = "sqlite"),
        derive(AsExpression)
    )]
    #[cfg_attr(
        feature = "postgres_backend",
        diesel(sql_type = crate::sql_types::Timestamptz)
    )]
    // #[cfg_attr(feature = "sqlite", diesel(sql_type = crate::sql_types::TimestamptzSqlite))]
    struct DateTimeProxy(OffsetDateTime);
}
