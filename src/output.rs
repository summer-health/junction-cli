use colored::Colorize;
use serde_json::Value;

pub fn print_json(value: &Value) {
    match serde_json::to_string_pretty(value) {
        Ok(s) => println!("{s}"),
        Err(e) => eprintln!("{}: {e}", "error formatting output".red()),
    }
}

pub fn print_success(msg: &str) {
    println!("{} {msg}", "✓".green());
}

pub fn print_error(msg: &str) {
    eprintln!("{} {msg}", "✗".red());
}
