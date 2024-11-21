pub mod initialization;
pub mod settings;


pub fn init_db() -> rusqlite::Result<()> {
    initialization::initialize_database()
}