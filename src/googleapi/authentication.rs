use reqwest;
use webbrowser;

fn get_auth_url() -> Result<String, reqwest::Error> {
    let mut res = reqwest::get("http://localhost:4000/")?;
    Ok(res.text()?)
}

pub fn authenticate() -> bool {
    let mut authenticated = false;
    let mut url = String::new();

    match get_auth_url() {
        Ok(v) => {
            println!("{:?}", v);
            url = v;
        }
        Err(_) => println!("user not authenticated"),
    };

    if url != "" {
        if webbrowser::open(&url).is_ok() {
            println!("opening browser");
            authenticated = true;
        };
    }

    println!("authenticated: {}", authenticated);

    authenticated
}
