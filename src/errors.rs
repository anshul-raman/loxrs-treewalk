pub fn error(line: i32, message: &str) {
    report(line, "", message);
}

fn report(line: i32, extra: &str, message: &str) {
    eprintln!("[Line: {line}] Error: {extra}: {message}")
}
