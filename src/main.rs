fn main() {
    // The first section of WAGI's output must be the CGI headers. At minimum,
    // you must specify either 'content-type' or 'location'. Then the header
    // must be followed by a blank line.
    println!("Content-Type: text/plain");

    // You can override the HTTP status by setting it here. The default
    // is 200 OK. If you set a 'location' header, this is automatically
    // changed to 302 Found.
    println!("Status: 200");
    // You can send customer headers with hte 'X-' prefix.
    println!("X-Foo-Header: Bar");
    println!(); // empty line

    // Loop through all of the arguments.
    //
    // Argument 0 is the path from the URL
    // The query string name/value pairs are each given as an argument.
    // So a=b&p=q will become two args: "a=b" and "p=q"
    println!("### Arguments ###");
    std::env::args().for_each(|a| println!("arg: {}", a));

    // The environment variables contain a mix of WAGI/CGI variables
    // and HTTP headers (all prefixed with HTTP_).
    println!("\n### Env Vars ###");
    std::env::vars().for_each(|v| {
        println!("{} = {}", v.0, v.1);
    });

    // If a body is sent by the client, it will be available on STDIN.
    // The CONTENT_LENGTH environment variable will tell the length of
    // this body. Note that CGI does not support streaming, though WAGI
    // may diverge from this and support streaming in the future.
    //
    // If no body is set, this will be empty.
    println!("\n### STDIN ###");
    std::io::copy(&mut std::io::stdin(), &mut std::io::stdout()).unwrap();

    // WAGI supports mounting directories to a WAGI module. This will
    // attempt to list all of the files under '/'. This can fail if
    // no 'volumes' are set in the 'modules.toml'.
    println!("\n### Files ###");
    std::fs::read_dir("/").unwrap().for_each(|d| {
        let o = d.unwrap();
        println!("{}", o.file_name().into_string().unwrap())
    });

    // This shows how to read the contents of a file from the 'volumes'.
    println!(
        "\n### Read LICENSE.txt ###\n {}",
        std::fs::read_to_string("/LICENSE.txt").unwrap()
    );
}
