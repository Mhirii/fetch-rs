pub fn seconds_to_string(seconds: usize) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;

    let mut result = String::with_capacity(10);

    if days > 0 {
        result.push_str(&format!("{}d", days));
    }
    if hours > 0 {
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(&format!("{}h", hours));
    }
    if minutes > 0 || result.is_empty() {
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(&format!("{}m", minutes));
    }

    result
}

pub fn memory_to_gb(memory: u64) -> String {
    let mem = memory as f64 / 1024.0 / 1024.0;
    format!("{:.2}", mem)
}

pub fn print_err(e: libmacchina::traits::ReadoutError) {
    eprintln!("Error: {}", e);
}

pub fn ok_readout<T: Default>(result: Result<T, libmacchina::traits::ReadoutError>) -> T {
    match result {
        Ok(r) => r,
        Err(e) => {
            print_err(e);
            T::default()
        }
    }
}
