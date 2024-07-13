use crate::fetcher::{self, Fetched};

fn add_line(lines: &mut Vec<(String, String)>, key: &str, value: Option<String>) {
    if let Some(value) = value {
        lines.push((key.to_string(), value));
    }
}

pub fn data_lines(data: Fetched) -> Vec<(String, String)> {
    let mut lines = vec![];
    add_line(&mut lines, "OS", data.os);
    add_line(&mut lines, "Kernel", data.kernel);
    add_line(&mut lines, "Host", data.host);
    add_line(&mut lines, "User", data.user);
    add_line(&mut lines, "Window Manager", data.window_manager);
    add_line(&mut lines, "Session", data.session);
    add_line(&mut lines, "Terminal", data.terminal);
    add_line(&mut lines, "Uptime", data.uptime);
    add_line(&mut lines, "CPU", data.cpu);
    add_line(&mut lines, "Memory", data.memory);
    add_line(&mut lines, "Packages", data.packages);
    add_line(&mut lines, "GPU1", data.gpu1);
    add_line(&mut lines, "GPU2", data.gpu2);

    lines
}
