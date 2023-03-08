#![recursion_limit = "1024"]
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    let mut connection = establish_connection();

    {
        use crate::schema::table_0::dsl::table_0;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_0.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_1::dsl::table_1;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_1.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_2::dsl::table_2;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_2.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_3::dsl::table_3;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_3.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_4::dsl::table_4;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_4.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_5::dsl::table_5;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_5.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_6::dsl::table_6;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_6.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_7::dsl::table_7;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_7.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_8::dsl::table_8;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_8.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_9::dsl::table_9;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_9.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_10::dsl::table_10;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_10.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_11::dsl::table_11;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_11.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_12::dsl::table_12;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_12.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_13::dsl::table_13;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_13.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_14::dsl::table_14;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_14.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_15::dsl::table_15;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_15.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_16::dsl::table_16;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_16.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_17::dsl::table_17;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_17.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_18::dsl::table_18;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_18.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_19::dsl::table_19;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_19.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_20::dsl::table_20;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_20.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_21::dsl::table_21;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_21.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_22::dsl::table_22;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_22.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_23::dsl::table_23;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_23.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_24::dsl::table_24;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_24.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_25::dsl::table_25;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_25.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_26::dsl::table_26;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_26.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_27::dsl::table_27;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_27.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_28::dsl::table_28;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_28.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_29::dsl::table_29;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_29.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_30::dsl::table_30;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_30.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_31::dsl::table_31;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_31.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_32::dsl::table_32;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_32.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_33::dsl::table_33;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_33.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_34::dsl::table_34;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_34.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_35::dsl::table_35;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_35.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_36::dsl::table_36;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_36.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_37::dsl::table_37;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_37.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_38::dsl::table_38;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_38.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_39::dsl::table_39;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_39.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_40::dsl::table_40;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_40.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_41::dsl::table_41;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_41.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_42::dsl::table_42;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_42.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_43::dsl::table_43;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_43.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_44::dsl::table_44;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_44.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_45::dsl::table_45;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_45.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_46::dsl::table_46;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_46.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_47::dsl::table_47;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_47.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_48::dsl::table_48;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_48.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_49::dsl::table_49;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_49.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_50::dsl::table_50;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_50.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_51::dsl::table_51;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_51.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_52::dsl::table_52;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_52.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_53::dsl::table_53;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_53.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_54::dsl::table_54;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_54.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_55::dsl::table_55;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_55.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_56::dsl::table_56;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_56.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_57::dsl::table_57;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_57.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_58::dsl::table_58;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_58.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_59::dsl::table_59;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_59.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_60::dsl::table_60;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_60.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_61::dsl::table_61;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_61.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_62::dsl::table_62;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_62.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_63::dsl::table_63;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_63.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_64::dsl::table_64;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_64.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_65::dsl::table_65;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_65.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_66::dsl::table_66;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_66.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_67::dsl::table_67;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_67.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_68::dsl::table_68;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_68.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_69::dsl::table_69;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_69.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_70::dsl::table_70;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_70.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_71::dsl::table_71;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_71.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_72::dsl::table_72;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_72.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_73::dsl::table_73;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_73.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_74::dsl::table_74;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_74.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_75::dsl::table_75;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_75.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_76::dsl::table_76;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_76.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_77::dsl::table_77;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_77.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_78::dsl::table_78;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_78.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_79::dsl::table_79;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_79.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_80::dsl::table_80;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_80.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_81::dsl::table_81;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_81.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_82::dsl::table_82;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_82.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_83::dsl::table_83;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_83.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_84::dsl::table_84;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_84.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_85::dsl::table_85;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_85.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_86::dsl::table_86;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_86.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_87::dsl::table_87;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_87.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_88::dsl::table_88;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_88.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_89::dsl::table_89;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_89.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_90::dsl::table_90;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_90.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_91::dsl::table_91;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_91.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_92::dsl::table_92;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_92.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_93::dsl::table_93;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_93.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_94::dsl::table_94;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_94.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_95::dsl::table_95;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_95.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_96::dsl::table_96;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_96.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_97::dsl::table_97;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_97.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_98::dsl::table_98;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_98.load::<Table>(&mut connection).unwrap();
    }

    {
        use crate::schema::table_99::dsl::table_99;

        #[derive(Debug, Queryable)]
        struct Table {
            id: i32,
            number: i32,
            line_1: String,
            line_2: String,
            optional_line_3: Option<String>,
            line_4: String,
            line_5: String,
            line_6: String,
            line_7: String,
        }
        let _result = table_99.load::<Table>(&mut connection).unwrap();
    }
}

// too much
