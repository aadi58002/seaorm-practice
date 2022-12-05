use migration::DbErr;
use sea_orm::{Set, ActiveModelTrait};
use sea_orm_practice::establish_connection;
use entity::table;

#[tokio::main]
async fn main() -> Result<(), DbErr>{
    let db = establish_connection().await?;

    let user = table::ActiveModel {
        email: Set(String::from("aadi58002@gmail.com")),
        password: Set(String::from("Like i will tell you")),
        ..Default::default()
    };

    let user_result: table::Model = user.insert(&db).await?;

    println!("User created with ID: {}, TITLE: {}", user_result.email, user_result.password);

    Ok(())
}
