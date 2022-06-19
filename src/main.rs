#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derives;

mod model;

use diesel::{ ExpressionMethods, QueryDsl};

fn main() {
    get_bill_book_account_sum();
}


pub fn get_bill_book_account_sum(){
    use crate::diesel::GroupByDsl;
    use diesel::dsl::max;
    use crate::model::diesel::dict::dict_schema::test as bill_record_table;
    let source_query = bill_record_table::table
        .group_by(bill_record_table::id)
        .select((max(bill_record_table::tags),bill_record_table::id))
        .filter(bill_record_table::dsl::tags.eq(9));
}

