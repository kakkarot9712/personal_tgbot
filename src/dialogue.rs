pub mod add_person_diag;
pub mod add_transaction_diag;
pub mod mode;
pub mod settle_due;
pub mod state;

pub fn insert_cancel_helper_text(data: String) -> String {
    format!(
        "{}\n\n{}",
        data, "Send /cancel to terminate current operation."
    )
}
