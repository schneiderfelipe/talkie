use talkie::Input;

fn main() {
    let input: String = Input::new().unwrap().interact_text().unwrap();
    println!("{input}");
}
