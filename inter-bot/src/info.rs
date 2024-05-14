pub async fn rust_info() -> String {

    let response = match reqwest::get("https://www.rust-lang.org").await{
        Ok(req_part1) => {
            let text = req_part1.text().await;
            match text {
                Ok(content) => content,
                Err(_) => return String::from("Could not read the text in the website!"),
            }
        },
        Err(_) => return String::from("Could not access the website"),
    };
    String::from("test")
    // response
}