use ego_tree::NodeRef;
use scraper::{ElementRef, Node};

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

pub(super) fn get_text_with_links_excluding_scripts(el: ElementRef) -> Vec<String> {
    fn dfs(node: NodeRef<Node>, out: &mut Vec<String>) {
        match node.value() {
            Node::Element(e) => {
                // Don't descend into <script> elements
                if e.name.local.as_ref() == "script" {
                    return;
                }
                if let Some(link) = e.attr("href") {
                    out.push(link.to_string());
                }
                for child in node.children() {
                    dfs(child, out);
                }
            }
            Node::Text(t) => {
                out.push(t.to_string());
            }
            _ => {
                for child in node.children() {
                    dfs(child, out);
                }
            }
        }
    }

    let mut buf = Vec::new();
    for child in el.children() {
        dfs(child, &mut buf);
    }
    buf
}
