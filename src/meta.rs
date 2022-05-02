use diesel::types::Jsonb;

#[derive(FromSqlRow, AsExpression, serde::Serialize, serde::Deserialize, Debug, Default)]
#[sql_type = "Jsonb"]
pub struct Meta {
    pub error_info: Option<String>,
    pub suggested_id_tag: Option<String>,
}