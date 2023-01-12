use crate::models::{User, NewUser};
use crate::db::establish_connection;
use diesel::prelude::*;

pub fn create_user(username: String, password: String) {
    use crate::schema::users::dsl::*;

    let mut conn = establish_connection();
    let new_user = NewUser{
        nombre : &username,
        pass : &password,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut conn)
        .expect("Error creating new user");

    println!("LOG bd_ops; created user {}", username)
}


pub fn get_user(username: String) -> User {
    use crate::schema::users::dsl::*;
    let mut connection = establish_connection();

    let users_matcheando = users.filter(nombre.eq(username)).load::<User>(&mut connection)
    .expect("falla el mostrar");
    
    
    let result = users_matcheando[0].clone();
    return result;
    
}

pub fn check_user(username: String) -> bool {
    use crate::schema::users::dsl::*;
    let mut connection = establish_connection();

    let users_matcheando = users.filter(nombre.eq(username)).load::<User>(&mut connection)
    .expect("falla el mostrar");
    
    
    if users_matcheando.len() > 0{
        return true;
    } else {
        return false;
    }    
}