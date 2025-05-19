pub mod business;
pub mod presentation;
pub mod database;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        presentation::view::render();
        business::user::create();
        database::user_dao::create();
    }
}
