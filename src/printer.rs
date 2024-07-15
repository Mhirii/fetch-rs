use crate::fetcher::{self, Fetched};

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
