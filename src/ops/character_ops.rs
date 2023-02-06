use crate::models::{NewCharacter, Character};
use crate::db::establish_connection;
use diesel::prelude::*;

pub fn check_for_character(u_id: i32) -> bool {
    use crate::schema::characters::dsl::*;
    let mut conn = establish_connection();
    let matching_chars = characters.filter(user_id.eq(u_id)).load::<Character>(&mut conn).
    expect("falla al buscar el personaje en bd");
    
    if matching_chars.len() > 0 {
        return true;
    } else {
        return false;
    };
}

pub fn get_character(u_id: i32) -> Character{
    use crate::schema::characters::dsl::*;
    let mut conn = establish_connection();
    let matching_chars = characters.filter(user_id.eq(u_id)).load::<Character>(&mut conn).
    expect("falla al buscar el personaje en bd");

    return matching_chars[0].clone();
}


pub fn create_character(u_id: i32, life_point: bool, damage_point: bool, mana_point:bool){
    use crate::schema::characters::dsl::*;
    let mut conn = establish_connection();

    let new_char = NewCharacter{
         vida: if life_point {120} else {100},
         mana:  if mana_point {120} else {100},
         danio: if damage_point{20} else {10},
         oro: 0,
         user_id: &u_id,

    };

    diesel::insert_into(characters)
        .values(&new_char)
        .execute(&mut conn)
        .expect("Error creating new user");

}