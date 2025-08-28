pub(crate) fn remove_unneeded_newlines(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let mut result = Vec::with_capacity(lines.len());
    let mut num_empty = 0;
    for line in lines {
        if line.is_empty() {
            num_empty += 1;
        } else {
            if num_empty > 1 {
                result.push("");
            }
            result.push(line);
            num_empty = 0;
        }
    }
    result.join("\n")
}
