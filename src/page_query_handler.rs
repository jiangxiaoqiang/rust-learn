use diesel::pg::Pg;
use diesel::query_builder::AstPass;
use diesel::sql_types::BigInt;

pub fn handle_table_query(&self, mut out: AstPass<Pg>){
    // https://stackoverflow.com/questions/6218902/the-sql-over-clause-when-and-why-is-it-useful
    out.push_sql("SELECT *, COUNT(*) OVER () FROM ");
    if self.is_sub_query {
        out.push_sql("(");
    }
    self.query.walk_ast(out.reborrow())?;
    if self.is_sub_query {
        out.push_sql(")");
    }
    out.push_sql(" t LIMIT ");
    out.push_bind_param::<BigInt, _>(&self.per_page)?;
    out.push_sql(" OFFSET ");
    let offset = (self.page - 1) * self.per_page;
    out.push_bind_param::<BigInt, _>(&offset)?;
}

fn handle_big_table_query(&self, mut out: AstPass<Pg>){
    out.push_sql("SELECT *, COUNT(*) OVER () FROM ");
    if self.is_sub_query {
        out.push_sql("(");
    }
    self.query.walk_ast(out.reborrow())?;
    if self.is_sub_query {
        out.push_sql(")");
    }
    out.push_sql(" t LIMIT ");
    out.push_bind_param::<BigInt, _>(&self.per_page)?;
    out.push_sql(" OFFSET ");
    let offset = (self.page - 1) * self.per_page;
    out.push_bind_param::<BigInt, _>(&offset)?;
}

