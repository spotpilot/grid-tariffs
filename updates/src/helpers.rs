use scraper::{ElementRef, Html, Selector};

pub(crate) fn find_text_elements<'a, 'b>(
    document: &'a Html,
    target_text: &'b str,
) -> Vec<ElementRef<'a>> {
    let all_selector = Selector::parse("*").unwrap();
    document
        .select(&all_selector)
        .filter(|el| el.text().collect::<String>().starts_with(target_text))
        .collect()
}

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
