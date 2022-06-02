
fn main() {
    let contents = Vec::new();
    convert_to_tree_impl_legacy(&contents)
}

pub fn convert_to_tree_impl_legacy(contents: &Vec<FortuneContent>){
    let mut root_element =  contents.iter().filter(|content|content.parent_id==0)
        .collect::<FortuneContent>();
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

impl FromIterator<&FortuneContent> for FortuneContent {
    fn from_iter< I: IntoIterator<Item=&_FortuneContent>>(iter: I) -> Self {
        Self {
            id: 0,
            parent_id: 0,
            created_time: 0,
            updated_time: 0,
            name: "".to_string(),
            contents_type: 0,
            deleted: 0,
            hidden: 0,
            sort: 0
        }
    }
}
