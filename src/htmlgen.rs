// htmlgen.rs
// This module takes the AST and turns it into HTML text.
// We walk through the AST nodes and emit HTML tags based on node type.

use crate::ast::Node;

pub struct HtmlGen;

impl HtmlGen {
    // Create a new HTML generator (no state needed).
    pub fn new() -> Self { Self }

    // Entry function: takes the AST and returns a full HTML string.
    pub fn generate(&mut self, ast: &Vec<Node>) -> String {
        let mut out = String::from("<html>\n");
        self.emit_nodes(ast, &mut out, 1);
        out.push_str("</html>\n");
        out
    }

    // Small helper for indentation in formatted output.
    fn indent(n: usize) -> String { "    ".repeat(n) }

    // Emit a list of nodes, respecting indentation for block-level HTML.
    fn emit_nodes(&self, nodes: &[Node], out: &mut String, level: usize) {
        for node in nodes {
            match node {
                // If there's a root Html(...) node, just emit its children.
                Node::Html(kids) => self.emit_nodes(kids, out, level),

                // HTML comment
                Node::Comment(t) => {
                    out.push_str(&format!("{}<!-- {} -->\n", Self::indent(level), t.trim()));
                }

                // <head>...</head>
                Node::Head(kids) => {
                    out.push_str(&format!("{}<head>\n", Self::indent(level)));
                    self.emit_nodes(kids, out, level + 1);
                    out.push_str(&format!("{}{}</head>\n", Self::indent(level), ""));
                }

                // <title>text</title>
                Node::Title(t) => {
                    out.push_str(&format!("{}<title> {} </title>\n", Self::indent(level), t.trim()));
                }

                // <p> ... </p>
                Node::Paragraph(kids) => {
                    out.push_str(&format!("{}<p> ", Self::indent(level)));
                    self.emit_nodes_inline(kids, out);
                    out.push_str("</p>\n");
                }

                // <b>text</b>
                Node::Bold(t) => {
                    out.push_str(&format!("<b> {} </b>", t.trim()));
                }

                // <i>text</i>
                Node::Italics(t) => {
                    out.push_str(&format!("<i> {} </i>", t.trim()));
                }

                // <br>
                Node::Newline => {
                    out.push_str("<br>\n");
                }

                // <ul> ... </ul>
                Node::List(items) => {
                    out.push_str(&format!("{}<ul>\n", Self::indent(level)));
                    self.emit_nodes(items, out, level + 1);
                    out.push_str(&format!("{}{}</ul>\n", Self::indent(level), ""));
                }

                // <li> ... </li>
                Node::ListItem(kids) => {
                    out.push_str(&format!("{}<li> ", Self::indent(level)));
                    self.emit_nodes_inline(kids, out);
                    out.push_str("</li>\n");
                }

                // Audio element
                Node::Audio(url) => {
                    out.push_str(&format!(
                        "{}<audio controls>\n{}<source src=\"{}\">\n{}</audio>\n",
                        Self::indent(level),
                        Self::indent(level + 1),
                        url.trim(),
                        Self::indent(level)
                    ));
                }

                // Video (YouTube iframe)
                Node::Video(url) => {
                    out.push_str(&format!(
                        "{}<iframe src=\"{}\"/>\n",
                        Self::indent(level),
                        url.trim()
                    ));
                }

                // Regular text inside blocks
                Node::Text(t) => {
                    out.push_str(t);
                }

                // These nodes are handled earlier in semantic stage, so we skip here.
                Node::VarDef { .. } | Node::VarUse { .. } | Node::Body(_) => { }
            }
        }
    }

    // Inline writer: used inside <p> and <li> so we don't insert new lines unnecessarily.
    fn emit_nodes_inline(&self, nodes: &[Node], out: &mut String) {
        for node in nodes {
            match node {
                Node::Bold(t)    => out.push_str(&format!("<b> {} </b>", t.trim())),
                Node::Italics(t) => out.push_str(&format!("<i> {} </i>", t.trim())),
                Node::Newline    => out.push_str("<br>\n"),
                Node::Text(t)    => out.push_str(t),

                Node::Audio(u)   => out.push_str(&format!(
                    "<audio controls><source src=\"{}\"></audio>", u.trim()
                )),

                Node::Video(u)   => out.push_str(&format!(
                    "<iframe src=\"{}\"/>", u.trim()
                )),

                // If nested blocks somehow end up inline, flatten them.
                Node::ListItem(k) | Node::Paragraph(k) | Node::Html(k) |
                Node::List(k) | Node::Head(k) | Node::Body(k) => {
                    self.emit_nodes_inline(k, out);
                }

                // Ignore nodes that don't belong inline.
                Node::Title(_) | Node::Comment(_) |
                Node::VarDef { .. } | Node::VarUse { .. } => { }
            }
        }
    }
}
