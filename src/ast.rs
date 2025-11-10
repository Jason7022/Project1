#[derive(Debug, Clone)]
pub enum Node {
    Html(Vec<Node>),

    Comment(String),

    Head(Vec<Node>),
    Title(String),

    Body(Vec<Node>),

    Paragraph(Vec<Node>),

    Bold(String),
    Italics(String),

    List(Vec<Node>),
    ListItem(Vec<Node>),

    Newline,
    Audio(String),
    Video(String),

    Text(String),

    VarDef { name: String, value: String },
    VarUse { name: String },
}
