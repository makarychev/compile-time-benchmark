use sea_orm::*;

mod models;
// mod schema;

use models::*;

async fn init_connection() -> Result<DatabaseConnection, DbErr> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(database_url).await?;

    Ok(db)
}

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();

    let db = init_connection().await.unwrap();
    // schema::setup_schema(&db).await?;


    let result = table_0::Entity::find().all(&db).await.unwrap();

    let result = table_1::Entity::find().all(&db).await.unwrap();

    let result = table_2::Entity::find().all(&db).await.unwrap();

    let result = table_3::Entity::find().all(&db).await.unwrap();

    let result = table_4::Entity::find().all(&db).await.unwrap();

    let result = table_5::Entity::find().all(&db).await.unwrap();

    let result = table_6::Entity::find().all(&db).await.unwrap();

    let result = table_7::Entity::find().all(&db).await.unwrap();

    let result = table_8::Entity::find().all(&db).await.unwrap();

    let result = table_9::Entity::find().all(&db).await.unwrap();

    let result = table_10::Entity::find().all(&db).await.unwrap();

    let result = table_11::Entity::find().all(&db).await.unwrap();

    let result = table_12::Entity::find().all(&db).await.unwrap();

    let result = table_13::Entity::find().all(&db).await.unwrap();

    let result = table_14::Entity::find().all(&db).await.unwrap();

    let result = table_15::Entity::find().all(&db).await.unwrap();

    let result = table_16::Entity::find().all(&db).await.unwrap();

    let result = table_17::Entity::find().all(&db).await.unwrap();

    let result = table_18::Entity::find().all(&db).await.unwrap();

    let result = table_19::Entity::find().all(&db).await.unwrap();

    let result = table_20::Entity::find().all(&db).await.unwrap();

    let result = table_21::Entity::find().all(&db).await.unwrap();

    let result = table_22::Entity::find().all(&db).await.unwrap();

    let result = table_23::Entity::find().all(&db).await.unwrap();

    let result = table_24::Entity::find().all(&db).await.unwrap();

    let result = table_25::Entity::find().all(&db).await.unwrap();

    let result = table_26::Entity::find().all(&db).await.unwrap();

    let result = table_27::Entity::find().all(&db).await.unwrap();

    let result = table_28::Entity::find().all(&db).await.unwrap();

    let result = table_29::Entity::find().all(&db).await.unwrap();

    let result = table_30::Entity::find().all(&db).await.unwrap();

    let result = table_31::Entity::find().all(&db).await.unwrap();

    let result = table_32::Entity::find().all(&db).await.unwrap();

    let result = table_33::Entity::find().all(&db).await.unwrap();

    let result = table_34::Entity::find().all(&db).await.unwrap();

    let result = table_35::Entity::find().all(&db).await.unwrap();

    let result = table_36::Entity::find().all(&db).await.unwrap();

    let result = table_37::Entity::find().all(&db).await.unwrap();

    let result = table_38::Entity::find().all(&db).await.unwrap();

    let result = table_39::Entity::find().all(&db).await.unwrap();

    let result = table_40::Entity::find().all(&db).await.unwrap();

    let result = table_41::Entity::find().all(&db).await.unwrap();

    let result = table_42::Entity::find().all(&db).await.unwrap();

    let result = table_43::Entity::find().all(&db).await.unwrap();

    let result = table_44::Entity::find().all(&db).await.unwrap();

    let result = table_45::Entity::find().all(&db).await.unwrap();

    let result = table_46::Entity::find().all(&db).await.unwrap();

    let result = table_47::Entity::find().all(&db).await.unwrap();

    let result = table_48::Entity::find().all(&db).await.unwrap();

    let result = table_49::Entity::find().all(&db).await.unwrap();

    let result = table_50::Entity::find().all(&db).await.unwrap();

    let result = table_51::Entity::find().all(&db).await.unwrap();

    let result = table_52::Entity::find().all(&db).await.unwrap();

    let result = table_53::Entity::find().all(&db).await.unwrap();

    let result = table_54::Entity::find().all(&db).await.unwrap();

    let result = table_55::Entity::find().all(&db).await.unwrap();

    let result = table_56::Entity::find().all(&db).await.unwrap();

    let result = table_57::Entity::find().all(&db).await.unwrap();

    let result = table_58::Entity::find().all(&db).await.unwrap();

    let result = table_59::Entity::find().all(&db).await.unwrap();

    let result = table_60::Entity::find().all(&db).await.unwrap();

    let result = table_61::Entity::find().all(&db).await.unwrap();

    let result = table_62::Entity::find().all(&db).await.unwrap();

    let result = table_63::Entity::find().all(&db).await.unwrap();

    let result = table_64::Entity::find().all(&db).await.unwrap();

    let result = table_65::Entity::find().all(&db).await.unwrap();

    let result = table_66::Entity::find().all(&db).await.unwrap();

    let result = table_67::Entity::find().all(&db).await.unwrap();

    let result = table_68::Entity::find().all(&db).await.unwrap();

    let result = table_69::Entity::find().all(&db).await.unwrap();

    let result = table_70::Entity::find().all(&db).await.unwrap();

    let result = table_71::Entity::find().all(&db).await.unwrap();

    let result = table_72::Entity::find().all(&db).await.unwrap();

    let result = table_73::Entity::find().all(&db).await.unwrap();

    let result = table_74::Entity::find().all(&db).await.unwrap();

    let result = table_75::Entity::find().all(&db).await.unwrap();

    let result = table_76::Entity::find().all(&db).await.unwrap();

    let result = table_77::Entity::find().all(&db).await.unwrap();

    let result = table_78::Entity::find().all(&db).await.unwrap();

    let result = table_79::Entity::find().all(&db).await.unwrap();

    let result = table_80::Entity::find().all(&db).await.unwrap();

    let result = table_81::Entity::find().all(&db).await.unwrap();

    let result = table_82::Entity::find().all(&db).await.unwrap();

    let result = table_83::Entity::find().all(&db).await.unwrap();

    let result = table_84::Entity::find().all(&db).await.unwrap();

    let result = table_85::Entity::find().all(&db).await.unwrap();

    let result = table_86::Entity::find().all(&db).await.unwrap();

    let result = table_87::Entity::find().all(&db).await.unwrap();

    let result = table_88::Entity::find().all(&db).await.unwrap();

    let result = table_89::Entity::find().all(&db).await.unwrap();

    let result = table_90::Entity::find().all(&db).await.unwrap();

    let result = table_91::Entity::find().all(&db).await.unwrap();

    let result = table_92::Entity::find().all(&db).await.unwrap();

    let result = table_93::Entity::find().all(&db).await.unwrap();

    let result = table_94::Entity::find().all(&db).await.unwrap();

    let result = table_95::Entity::find().all(&db).await.unwrap();

    let result = table_96::Entity::find().all(&db).await.unwrap();

    let result = table_97::Entity::find().all(&db).await.unwrap();

    let result = table_98::Entity::find().all(&db).await.unwrap();

    let result = table_99::Entity::find().all(&db).await.unwrap();


}
