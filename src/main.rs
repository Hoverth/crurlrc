use crurlrc::check_url;
use std::{env, io::stdin};

const HELP_TEXT: &str = r#"crurlrc [options] [URLs]
clean and reliable URL redirect checker

If a redirect to a URL is found, that URL will be printed to stdout, otherwise it'll just be a blank line. All other output is printed to stderr.

Options
-h --help => show this help
-i        => read URLs from stdin (useful for scripts)
-c        => disable colour output to stderr"#;
pub const GREY: &str = "\x1b[90;3m";
pub const RESET: &str = "\x1b[0m";

fn process_str_for_url(url: &str) {
    if url.contains("http://") || url.contains("https://") {
        if let Some(res) = check_url(url) {
            println!("{}", res.redirect_to);
        } else {
            eprintln!("Error fetching redirect!");
            println!();
        }
    } else {
        println!();
    }
}

fn main() {
    let mut do_stdin_mode = false;
    let mut no_colour = false;
    let args = env::args();
    for arg in args {
        if arg == "-h" || arg == "--help" {
            println!("{}", HELP_TEXT);
            return;
        } else if arg == "-i" {
            do_stdin_mode = true;
        } else if arg == "-c" {
            no_colour = true;
        } else if arg.contains("://") {
            process_str_for_url(&arg);
        }
    }

    if do_stdin_mode {
        let mut buf = String::new();
        while let Ok(_) = stdin().read_line(&mut buf) {
            buf = String::from(buf.trim());
            if buf.is_empty() {
                return;
            }
            if no_colour {
                eprintln!("{buf:?}");
            } else {
                eprintln!("{GREY}{buf:?}{RESET}");
            }
            process_str_for_url(&buf);
            buf = String::new();
        }
    }
}
