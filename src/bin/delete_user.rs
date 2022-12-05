use entity::table;
use migration::DbErr;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, ModelTrait};
use sea_orm_practice::establish_connection;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db = establish_connection().await?;
    let matching_users = table::Entity::find()
        .filter(table::Column::Email.eq("aadi58002@gmail.com"))
        .all(&db)
        .await;

    for users in matching_users {
        for ele in users {
            println!("{:?}", ele.delete(&db).await?);
        }
    }
    Ok(())
}
