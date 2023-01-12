use core::fmt;

use::diesel::prelude::*;
use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;
use crate::schema::users;
use crate::schema::characters;


//modelos de USER
#[derive(Queryable, Clone)]
pub struct User {
    pub id: i32,
    pub nombre: String,
    pub pass: String,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.nombre, self.pass)
    }
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("User", 2)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("nombre", &self.nombre)?;
        state.end()
    }
}

#[derive(Insertable)]
#[diesel(table_name=users)]
pub struct NewUser <'a>{
    pub nombre: &'a String,
    pub pass: &'a String,
}



//modelos de CHARACTER
#[derive(Queryable, Clone)]
pub struct Character {
    pub id: i32,
    pub vida: i32,
    pub mana: i32,
    pub danio: i32,
    pub oro: i32,
    pub user_id: i32,
}

impl Serialize for Character {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Character", 6)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("vida", &self.vida)?;
        state.serialize_field("mana", &self.mana)?;
        state.serialize_field("danio", &self.danio)?;
        state.serialize_field("oro", &self.oro)?;
        state.serialize_field("user_id", &self.user_id)?;
        state.end()
    }
}


#[derive(Insertable)]
#[diesel(table_name=characters)]
pub struct NewCharacter <'a>{
    pub vida: i32,
    pub mana:  i32,
    pub danio: i32,
    pub oro: i32,
    pub user_id: &'a i32,
}