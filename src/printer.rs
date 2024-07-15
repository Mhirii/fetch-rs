use colored::Colorize;
use pfetch_logo_parser::{Logo, LogoPart};

use crate::fetcher::Fetched;

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

fn add_line(lines: &mut Vec<(String, String)>, key: &str, value: Option<String>) {
    if let Some(value) = value {
        lines.push((key.to_string(), value));
    }
}

pub fn space_lines(data: Vec<(String, String)>) -> Vec<(String, String)> {
    let mut spaced: Vec<(String, String)> = vec![];
    let max_len = get_max_len(&data);
    for (key, value) in data.iter() {
        spaced.push((
            add_spaces(String::from(key), max_len).0,
            String::from(value),
        ));
    }
    spaced
}

fn get_max_len(v: &[(String, String)]) -> usize {
    let mut max_len: usize = 0;
    for (key, _) in v.iter() {
        let len = key.len();
        if len > max_len {
            max_len = len;
        }
    }
    max_len
}

fn add_spaces(string: String, len: usize) -> (String, usize) {
    if string.len() == len {
        return (string, len);
    }
    add_spaces(string + " ", len)
}

pub fn render(info_lines: Vec<(String, String)>, logo: Logo, logo_enabled: bool) {
    let raw_logo = if logo_enabled {
        logo.logo_parts
            .iter()
            .map(|LogoPart { content, .. }| content.as_ref())
            .collect::<String>()
    } else {
        "".to_string()
    };

    let logo = logo.to_string();
    let mut logo_lines = logo.lines();
    let raw_logo_lines: Vec<_> = raw_logo.lines().collect();

    let line_amount = usize::max(raw_logo_lines.len(), info_lines.len());

    let logo_width = raw_logo_lines
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);

    let padding1 = 2;
    let padding2 = 2;
    let padding3 = 2;

    let mut result = String::new();
    for l in 0..line_amount {
        result += &format!(
            "{padding1}\x1b[1m{logo}{padding2}{key}\x1b[0m{separator}{padding3}{value}\n",
            padding1 = " ".repeat(padding1),
            logo = if logo_enabled {
                logo_lines.next().unwrap_or("")
            } else {
                ""
            },
            padding2 = " ".repeat(
                logo_width - raw_logo_lines.get(l).map_or(0, |line| line.chars().count())
                    + if logo_enabled { padding2 } else { 0 }
            ),
            key = info_lines.get(l).map_or("", |line| &line.0).blue().bold(),
            separator = " ",
            padding3 = " ".repeat(padding3),
            value = info_lines.get(l).map_or("", |line| &line.1).white(),
        )
    }

    // disable line wrap so that the logo dont get messed up on small terminals
    crossterm::execute!(std::io::stdout(), crossterm::terminal::DisableLineWrap)
        .unwrap_or_default();

    println!("{result}");

    // re enable line wrap
    crossterm::execute!(std::io::stdout(), crossterm::terminal::EnableLineWrap).unwrap_or_default();
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_get_max_len() {
        let empty = String::from("");
        let mut vec: Vec<(String, String)> = vec![(String::from("hello"), empty)];
        vec.push((String::from("hello world!"), String::from("")));
        assert!(get_max_len(&vec) == 12);
        vec.push((String::from("hello     world!"), String::from("")));
        assert!(get_max_len(&vec) == 16);
    }

    #[test]
    fn test_add_spaces() {
        let len: usize = 10;
        let spaced = add_spaces(String::from("hi"), len).0.len();

        // should be the same length
        assert_eq!(spaced, len)
    }

    #[test]
    fn test_space_lines() {
        let empty = String::from("");
        let mut data = vec![(String::from("hello"), empty.clone())];
        data.push(("OS".to_owned(), "Arch Linux".to_owned()));
        let vec = space_lines(data.clone());
        let first = &vec[0].0;
        let second = &vec[1].0;

        // should be the same length
        assert_eq!(first.len(), second.len());
        // should preserve order
        assert_eq!(first.trim(), data[0].0.trim());
        assert_eq!(second.trim(), data[1].0.trim());
    }
}
