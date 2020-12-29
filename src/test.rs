extern crate reqwest;
use crate::gd;
extern crate dash_rs;
extern crate serde_json;


use serde_json::
{
    Map, Value,
};

pub fn test()
{
    let page:u32 = 1;
    let test = gd::prepare_search_request("Based", page);

    match test
    {
        Ok(data) =>
            {
                let level_list = gd::process_levels21_response(&data);

                //println!("{:?}", &level_list);

                match level_list
                {
                    Ok(list) =>
                        {
                            let j = serde_json::to_string(&list).unwrap();

                            println!("{}", j);

                            // let level = list.get(0).unwrap();
                            // let creator_name = gd::get_creator_name(&level.creator.as_ref().unwrap());
                            // println!("{}, {}", level.creator.as_ref().unwrap().name, level.name);
                            // let based_desc = gd::get_level_description(&level.description);
                            // match based_desc
                            // {
                            //     Some(desc) =>
                            //         {
                            //         println!("{}", desc);
                            //     },
                            //     None => println!("No Description provided"),
                            // }
                        },
                    Err(err) => println!("Couldn't parse GD stream, {}", err)
                };
            },
        Err(err) => println!("Something went wrong, {}", err)
    }

    // let Based = dash_rs::request::level::LevelRequest::new(63454942);
    //
    // let url = Based.to_url();
    // println!("{}", url);
    //
    // let demons = dash_rs::request::level::LevelsRequest::default()
    //     .request_type(dash_rs::request::level::LevelRequestType::Awarded)
    //     .page(1);
    //
    // let parameters = [("gameVersion", "21"), ("binaryVersion", "33"), ("secret", "Wmfd2893gb7"), ("type", "11"), ("str", ""), ("len", "-"),("diff", "-"), ("page", "0"),
    //     ("total", "0"), ("featured", "0"), ("original", "0"), ("twoPlayer", "0"), ("coins", "0"), ("epic", "0"), ("star", "0"), ("onlyCompleted", "0"), ("coins", "0"),
    //     ("uncompleted", "0")];
    //
    // let client = reqwest::blocking::Client::new();
    // let response = client.post("http://www.boomlings.com/database/getGJLevels21.php")
    //     .form(&parameters)
    //     .send()
    //     .unwrap()
    //     .text()
    //     .unwrap();
    //
    // let levelstream = dash_rs::response::parse_get_gj_levels_response(&response).unwrap();
    // println!("{:?}", levelstream)



}