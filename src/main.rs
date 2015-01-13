#![feature(plugin)]

extern crate hyper;

#[plugin]
extern crate from_json_macros;

extern crate from_json;
extern crate serialize;


use std::io::stdout;
use std::io::util::copy;

use hyper::Client;

use from_json::FromJson;

#[from_json_struct]
#[deriving(Show)]
struct Recipes {
    id : i32,
    #[from_json_name = "type"]
    recipe_type : String,
    output_item_id : i32,
    output_item_count : i32,
    time_to_craft_ms : i32,
    disciplines : Vec<String>,
    min_rating : i32,
    flags : Vec<String>,
    ingredients : Vec<Ingredients>
}

#[from_json_struct]
#[deriving(Show)]
struct Ingredients {
    item_id : i32,
    count : i32
}

fn main () {
    // create a client
    let mut client = Client::new();

    // Create an outgoing request.
    let mut res = match client.get("https://api.guildwars2.com/v2/recipes/7319")
                              .send() {
        Ok(res)  => res,
        Err(err) => panic!("Failed to connect: {:?}", err)
    };

    // read the response

    println!("Response: {}", res.status);
    let body = res.read_to_string().unwrap();
    println!("Body: {}", body);

    let body_json = serialize::json::from_str(body.as_slice()).unwrap();
    let content: Recipes = FromJson::from_json(&body_json).unwrap();
    assert_eq!(content.id, 7319);
}

#[test]
fn test1() {
    let test_json = serialize::json::from_str(r#"
{
  "type": "RefinementEctoplasm",
  "output_item_id": 46742,
  "output_item_count": 1,
  "min_rating": 450,
  "time_to_craft_ms": 5000,
  "disciplines": [ "Armorsmith", "Artificer", "Huntsman", "Weaponsmith" ],
  "flags": [ "AutoLearned" ],
  "ingredients": [
    { "item_id": 19684, "count": 50 },
    { "item_id": 19721, "count": 1 },
    { "item_id": 46747, "count": 10 }
  ],
  "id": 7319
}"#).unwrap();

    let content: Recipes = FromJson::from_json(&test_json).unwrap();

    assert_eq!(content.id, 7319);
    assert_eq!(content.output_item_id, 46742);
    assert_eq!(content.output_item_count, 1);
    assert_eq!(content.min_rating, 450);
    assert_eq!(content.disciplines.len(), 4);
    assert_eq!(content.ingredients.len(), 3);
    assert_eq!(content.flags[0], "AutoLearned");
}
