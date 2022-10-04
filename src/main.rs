use reqwest;

#[derive(Debug)]
struct AOC {
    token: String,
    base_url: String,
    year: u32,
    day: u32,
}

impl AOC {
    fn new(token: String, year: u32, day: u32) -> Self {
        AOC {
            token,
            base_url: "https://adventofcode.com".to_string(),
            year,
            day,
        }
    }
    fn url(&self) -> String {
        format!("{}/{}/day/{}", self.base_url, self.year, self.day)
    }
    fn input_url(&self) -> String {
        format!("{}/{}", self.url(), "input")
    }
    #[allow(dead_code)]
    pub fn download_input(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok("".to_string())
    }
    fn get_input_content(&self) -> Result<String, reqwest::Error> {
        let url = self.input_url();
        let session_token_value = format!("session={}", self.token);
        let client = reqwest::blocking::Client::new();
        let res = client.get(url)
            .header("Cookie", session_token_value)
            .send()?
            .text()?;
        Ok(res)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session_token = "53616c7465645f5fac47f59ea6fa035c7b91823b0f162891399e6dd6299341715e449503ce1fc018e4fa725b71041a81774a0d2481d04526ae45165caaa8e597".to_string();
    let aocurl = AOC::new(session_token, 2020, 1);
    let input_data = aocurl.get_input_content()?;
    println!("{}", input_data);
    Ok(())
}
