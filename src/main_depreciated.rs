extern crate http;
extern crate serialize;

use serialize::json;
use serialize::{Decoder, Decodable};
use http::client::RequestWriter;
use http::method::Get;

#[deriving(Decodable)]
pub struct ApiBuild{
    build_id: int,
}
#[deriving(Decodable)]
pub struct ApiItems{
    items: Vec<int>,
}
#[deriving(Decodable)]
pub struct ApiRecipeDetails{
    recipe_id: int,
    type_: String,
    output_item_id: String,
    output_item_count: String,
    min_rating: String,
    time_to_craft_ms: String,
    vendor_value: String,
    disciplines: Vec<String>,
    flags: Vec<String>,
    ingredients: Vec<ApiIngredient>,
}
#[deriving(Decodable)]
pub struct ApiIngredient{
    item_id: String,
    count: String,
}

fn main() {
    // Build ID
    let api_build_url = "https://api.guildwars2.com/v1/build.json";
    let api_build_data = make_api_request(api_build_url.as_slice());
    let GW2apiBuild: ApiBuild = decode_api_data(api_build_data);
    println!("Build ID: {}",GW2apiBuild.build_id);
    /*
    // Items
    let api_items_url = "https://api.guildwars2.com/v1/items.json";
    let api_items_data = make_api_request(api_items_url.as_slice());
    let GW2apiItems: ApiItems = decode_api_data(api_items_data);
    println!("Build ID: {}",GW2apiItems.items);
    */
    let api_RecipeDetails_url = "https://api.guildwars2.com/v1/recipe_details.json?recipe_id=1275";
    let api_RecipeDetails_data = make_api_request(api_RecipeDetails_url.as_slice());
    let GW2apiRecipeDetails: ApiRecipeDetails = decode_api_data(api_RecipeDetails_data);
    println!("{}",GW2apiRecipeDetails.type_);
}

fn decode_api_data<T: Decodable<json::Decoder,json::DecoderError>>(strData: Box<String>) -> T{
    // Process the JSON request
    let json_object = json::from_str(strData.as_slice());
    let mut decoder = json::Decoder::new(json_object.unwrap());
    let decoded: T = match Decodable::decode(&mut decoder) {
        Ok(v)  => v,
        Err(e) => fail!("Decoding error: {}",e),
    };
    return decoded;
}

fn make_api_request(url: &str) -> Box<String>{
    // Make the request
    let request: RequestWriter = RequestWriter::new(Get, from_str(url)
                                                         .expect("Invalid URL"))
                                                         .unwrap();
    println!("Requested URL: {}", request.url.to_str());
    let mut response = match request.read_response() {
        Ok(response)  => response,
        Err(_request) => fail!("No response"),
    };
    println!("Status: {}",response.status);
    let body = match response.read_to_end() {
        Ok(body)  => body,
        Err(err)  => fail!("Failed to read response: {}",err),
    };
    let data = box String::from_utf8(body).unwrap();
    return data;
}
