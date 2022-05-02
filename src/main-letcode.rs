#[macro_use]
extern crate diesel;

fn main() {

    let num = [0,1];
    for n in num {
        let mut len = 1;
        let element = num[num.len-len];
        if element == 0 {
            for next_elements in 0..len {
                let next_element = num[num.len() - next_elements];
                if next_element != 0 {
                    let tmp_element = element;
                    num[num.len-len] = num[num - next_elements];
                    num[num - next_elements] = tmp_element;
                }
            }
        }
        len = len + 1;
    }
    println!("{:?}", num);
}