use sqlx::*;

async fn run() -> Result<()> {
    let database_url = "postgres://postgres:whatever@localhost";

    let pool: PgPool = Pool::connect(database_url).await?;

    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_0" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_1" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_2" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_3" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_4" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_5" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_6" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_7" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_8" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_9" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_10" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_11" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_12" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_13" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_14" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_15" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_16" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_17" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_18" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_19" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_20" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_21" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_22" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_23" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_24" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_25" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_26" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_27" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_28" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_29" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_30" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_31" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_32" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_33" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_34" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_35" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_36" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_37" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_38" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_39" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_40" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_41" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_42" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_43" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_44" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_45" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_46" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_47" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_48" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_49" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_50" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_51" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_52" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_53" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_54" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_55" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_56" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_57" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_58" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_59" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_60" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_61" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_62" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_63" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_64" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_65" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_66" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_67" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_68" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_69" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_70" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_71" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_72" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_73" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_74" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_75" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_76" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_77" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_78" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_79" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_80" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_81" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_82" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_83" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_84" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_85" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_86" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_87" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_88" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_89" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_90" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_91" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_92" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_93" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_94" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_95" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_96" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_97" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_98" )
    .fetch_all( &pool )
    .await?;
    let result = sqlx::query!( "SELECT id, number, line_1, line_2, optional_line_3, line_4, line_5, line_6, line_7 FROM table_99" )
    .fetch_all( &pool )
    .await?;
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    run().await
}

// 11m 21s | 8m 47s | 8m 19s | 14m 4s | 9m 6s | avg: 10m 19s - build
