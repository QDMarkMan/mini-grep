/** ---------------------------------------------------------------------------------------------
 *  @Author [Tongfu.E].
 *  @Email [etongfu@outlook.com].
 *  @Date [2024-11-25 14:22:06].
 *-------------------------------------------------------------------------------------------- */
use std::env;
use std::process;
use mini_grep::Config;

fn main() {
    println!("Hello, Welcome mini grep!");
    // dbg!(args);
    let config = Config::generate(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let filename = &config.filename;
    println!("The query string is {}, filename is {}", config.query, filename);
    
    if let Err(e) = mini_grep::run(config)  {
        println!("Application error: {e}");
        process::exit(1);
    }
}
