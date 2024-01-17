#[cfg(not(target_arch = "wasm32"))]
#[macro_export]
macro_rules! flog {
    ($path:expr, $($arg:tt)*) => {{
        use chrono::Local;
        use std::fs::OpenOptions;
        use std::io::Write;

        let log_message = format!($($arg)*);
        let now = Local::now();
        let formatted = format!("{}", now.format("%H:%M:%S%.3f")) ;
        let log_line = format!("[{}] {}", formatted, log_message);

        if let Ok(mut file) = OpenOptions::new()
            .append(true)
            .create(true)
            .open($path)
        {
            if let Err(e) = writeln!(file, "{}", log_line) {
                eprintln!("Error writing to log file: {}", e);
            }
        } else {
            eprintln!("Error opening log file!");
        }
    }};
}
