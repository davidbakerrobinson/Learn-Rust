fn main() {
    let a = [2;1000];

    let output = if a.len() >= 100 {
        "Wow, that's a big array!"
    } else {
        "Meh, I eat arrays like that for breakfast."
    };

    println!("{}", output)
}
