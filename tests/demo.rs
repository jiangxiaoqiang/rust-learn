use actix_web::{test, web, App};

#[cfg(test)]
mod tests {

    #[actix_rt::test]
    async fn test_compile_see() {
        add_demo();
    }
}
