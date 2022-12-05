use migration::DbErr;
use sea_orm::EntityTrait;
use sea_orm_practice::establish_connection;
use entity::table;

#[tokio::main]
async fn main() -> Result<(), DbErr>{
    let db = establish_connection().await?;

    let users: Vec<table::Model> = table::Entity::find().all(&db).await?;

    println!("All the users in db:");
    for user in users {
        println!("Email: {}, Password: {}", user.email, user.password);
    }

    Ok(())
}
