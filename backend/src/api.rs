use{ 
    rocket::form::{FromForm, Form},
    rocket::{get, post},
    rocket::serde::json::Json,
    serde::*,
};

#[derive(FromForm, Serialize, Deserialize, Debug ,Clone)]
struct Player{
    name : String,
    score : String,
}
#[derive(FromForm ,Serialize, Deserialize, Debug, Clone)]
struct ScoreTable{
    key : String,
    players : Vec<Player>,
}

#[post("/createTable", data = "<table_data>")]
async fn create_new_table(table_data : Form<ScoreTable>) -> Option<String>{
    // return the key of the table created in the database checks iff table the
    // table is in the database
    None
}

#[get("/getTable/<key>")]
async fn get_table(key : &str) -> Option<Json<ScoreTable>>{
    None
}


pub fn get_api_routes() -> Vec<rocket::Route>{
    routes![create_new_table, get_table]
}
