use super::identifiable::Identifiable;

#[derive(Debug)]
pub struct Payment {
    pub id: u32,
    pub amount: f64,
    pub invoice_id: u32,
}

impl Identifiable for Payment {
    fn id(&self) -> u32 {
        self.id
    }
}