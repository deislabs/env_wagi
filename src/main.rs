fn main() {
    println!("Status: 200\nContent-Type: text/plain\n");

    println!("### Arguments ###");
    std::env::args().for_each(|a| println!("arg: {}", a));

    println!("### Env Vars ###");
    std::env::vars().for_each(|v| {
        println!("Env Var: {:?}", v);
    });
}
