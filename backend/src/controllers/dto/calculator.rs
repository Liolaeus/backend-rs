use serde::Deserialize;

#[derive(Deserialize)]
pub struct CalcQuery {
    // no option: missing param -> 400
    pub expr: String,
}
