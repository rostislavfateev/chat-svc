
pub mod memory;
pub mod postgres;

pub trait ChatStore {
    fn save_message();
    fn recent_history();
    fn register_user();
    // @todo extend with more
}
