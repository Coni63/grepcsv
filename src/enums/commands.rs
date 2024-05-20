#[derive(Debug, PartialEq, Eq)]
pub enum CommandType {
    Head,
    Tail,
    ColumnName,
    ColumnIndex,
    Help,
}
