use convert_case::{Case, Casing};
use ego_tree::NodeRef;
use grid_tariffs::{Country, GridOperator};
use scraper::{ElementRef, Node};

pub(crate) fn snakeify(operator_name: &str) -> String {
    slug::slugify(operator_name).to_case(Case::Snake)
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

pub(crate) fn get_text_with_links_excluding_scripts(el: ElementRef) -> Vec<String> {
    fn dfs(node: NodeRef<Node>, out: &mut Vec<String>) {
        match node.value() {
            Node::Element(el) => {
                // Don't descend into <script> elements
                if ["script", "svg", "style"].contains(&el.name.local.as_ref()) {
                    return;
                }
                if let Some(link) = el.attr("href") {
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

pub(crate) fn where_operator_name_starts_with(
    text: &str,
    country: Option<Country>,
) -> Vec<&'static GridOperator> {
    GridOperator::all()
        .into_iter()
        .filter(|op| {
            let needle = text.to_ascii_lowercase();
            let op_name = op.name().to_ascii_lowercase();
            let matches_country = country.is_none() || country == Some(op.country());
            matches_country && op_name.to_ascii_lowercase().starts_with(&needle)
        })
        .collect()
}

pub(crate) fn where_operator_vat_number_is(vat_number: &str) -> Vec<&'static GridOperator> {
    GridOperator::all()
        .into_iter()
        .filter(|op| op.vat_number().to_ascii_lowercase() == vat_number.to_ascii_lowercase())
        .collect()
}
