use krtek::Krtecek;

fn main() {
    let link = "https://html5boilerplate.com/";

    let krtecek = Krtecek::new(link).find_tag("h1").find_class("btn-download");

    match krtecek.run() {
        Ok(result) => {
            for (key, value) in result {
                println!("{}: {:?}", key, value);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
