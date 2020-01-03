use reqwest;

pub fn get_lists() -> Result<String, reqwest::Error> {
    let mut res = reqwest::get("http://localhost:4000/lists")?;
    Ok(res.text()?)
}
