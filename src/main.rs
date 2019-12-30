mod ui;
mod googleapi;

// fn get_tasks() ->  Result<(), Box<dyn std::error::Error>> {
//     let mut res = reqwest::get("http://localhost:4000/tasks")?;
//     let mut body = String::new();
//     res.read_to_string(&mut body)?;
//     println!("{}", body);
//     Ok(())
// }

fn main() {
    ui::application::load_application();
    // let authenticated = googleapi::authentication::authenticate();
    // println!("body = {:?}", auth_url.status());
    // get_tasks();
}
