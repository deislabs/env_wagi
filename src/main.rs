fn main() {
    println!("Status: 200\nContent-Type: text/plain\nX-Foo-Header: Bar\n");

    println!("### Arguments ###");
    std::env::args().for_each(|a| println!("arg: {}", a));

    println!("\n### Env Vars ###");
    std::env::vars().for_each(|v| {
        println!("{} = {}", v.0, v.1);
    });

    println!("\n### STDIN ###");
    std::io::copy(&mut std::io::stdin(), &mut std::io::stdout()).unwrap();

    println!("\n### Files ###");
    std::fs::read_dir("/").unwrap().for_each(|d| {
        let o = d.unwrap();
        println!("{}", o.file_name().into_string().unwrap())
    });
    println!(
        "\n### Read LICENSE.txt ###\n {}",
        std::fs::read_to_string("/LICENSE.txt").unwrap()
    );
}
