use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
extern crate derive_more;
extern crate serde;
extern crate serde_json;

mod schema;
mod db;
mod models;
mod ops;
mod structures;


#[get("/user/is_registered/{username}")]
async fn is_user_registered(path: web::Path<String>) -> impl Responder {
    let is_registered = ops::user_ops::check_user(path.into_inner());

    println!("LOG actix; created user");
    if is_registered == true {
        return HttpResponse::Ok().body("User_is_registred");
    } else {
        return HttpResponse::Ok().body("User_is_not_registred");
    }

}


#[get("/user/login/{username}/{password}")]
async fn login_user(path: web::Path<(String, String)>) -> impl Responder {
    let is_registered = ops::user_ops::check_user(path.0.clone());
    
    if is_registered {
        let user_bd = ops::user_ops::get_user(path.0.clone());
        if user_bd.pass == path.1 {
            println!("LOG actix; succesfull_login");
            return HttpResponse::Ok().json(user_bd);
        } else {
            println!("LOG actix; bad_credentials");
            return HttpResponse::Ok().body("Bad_credentials");
        }
    } else {
    	println!("LOG actix; user_not_registered");
        return HttpResponse::Ok().body("User_is_not_registred");
    }
}


#[get("/user/register/{username}/{password}")]
async fn register_user(path: web::Path<(String, String)>) -> impl Responder {
    ops::user_ops::create_user(path.0.clone(), path.1.clone());
    HttpResponse::Ok().body("Registrado")
}


#[get("/user/{user_id}/haves_character")]
async fn user_haves_character(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner().parse::<i32>().unwrap();
    let haves_char = ops::character_ops::check_for_character(id);


    println!("LOG actix; Character pinged");
    if haves_char == true{
        return HttpResponse::Ok().body("User_haves_char");
    } else {
        return HttpResponse::Ok().body("User_does_not_have_char");
    }
}


#[get("/user/{user_id}/create_character/{life}/{damage}/{mana}")]
async fn create_character(path: web::Path<(String, String, String, bool)>) -> impl Responder {
    let id = path.0.clone().parse::<i32>().unwrap();
    let life_point = path.1.clone().parse::<bool>().unwrap();
    let damage_point = path.2.clone().parse::<bool>().unwrap();
    let mana_point = path.3.clone();

    ops::character_ops::create_character(id, life_point, damage_point, mana_point);

    println!("LOG actix; Character created succesfully");
    return HttpResponse::Ok().body("Created_user_char");

}

#[get("/user/{user_id}/get_character")]
async fn get_character(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner().parse::<i32>().unwrap();
    let haves_char = ops::character_ops::get_character(id);


    println!("LOG actix; Character requested");
    return HttpResponse::Ok().json(haves_char);
   
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(register_user)
            .service(is_user_registered)
            .service(login_user)
            .service(user_haves_character)
            .service(create_character)
            .service(get_character)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
 
