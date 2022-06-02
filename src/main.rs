use std::iter::FromIterator;

fn main() {
    let contents = Vec::new();
    convert_to_tree_impl_legacy(&contents)
}

pub fn convert_to_tree_impl_legacy(contents: &Vec<FortuneContent>){
    let mut root_element:Vec<_> = contents.iter()
        .filter(|content|content.parent_id==0)
        .collect();

}

pub struct FortuneContent {
    pub id: i32,
    pub parent_id: i32,
    pub created_time: i64,
    pub updated_time: i64,
    pub name: String,
    pub contents_type: i32,
    pub deleted: i32,
    pub hidden: i32,
    pub sort: i32,
}


