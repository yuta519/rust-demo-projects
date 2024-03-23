pub fn color_name_to_jp(color: &str) -> Option<&str> {
    match color {
        "red" => Some("赤"),
        "green" => Some("緑"),
        "blue" => Some("青"),
        _ => Some("その他"),
    }
}
