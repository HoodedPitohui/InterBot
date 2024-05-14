//test function for APIs
pub async fn rust_info() -> String {

    //get the text from the response
    //you can't use and_then and map_err because those rely on the functions not being async
    let response = match reqwest::get("https://www.rust-lang.org").await{

        //gets past the first part of fetching the API
        Ok(req_part1) => {
            let text = req_part1.text().await;

            //see if you get the text out
            match text {
                Ok(content) => content,
                Err(_) => return String::from("Could not read the text in the website!"),
            }
        },
        Err(_) => return String::from("Could not access the website"),
    };

    //throttle the amount of lines that can be shown otherwise discord won't like it
    let num_lines: u32 = 10;
    let mut reply: String = format!("First {} lines are: ", num_lines);
    let mut lines = response.lines();


    //iterate through the number of lines specified and add them
    for n in 0..num_lines {
        reply.push_str(&lines.next().unwrap());
        let suffix = String::from("\n");
        reply.push_str(&suffix);
    }
    reply

}