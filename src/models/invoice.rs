use super::identifiable::Identifiable;
use super::payment::Payment;

#[derive(Debug)]
pub struct Invoice {
    pub id: u32,
    pub customer: String,
    pub amount: f64,
    pub paid: f64,
    pub payments: Vec<Payment>
}

impl Identifiable for Invoice {
    fn id(&self) -> u32 {
        self.id
    }
}

#[derive(Debug, PartialEq)]
pub enum InvoiceStatus {
    Unpaid,
    PartiallyPaid,
    Paid,
}


impl Invoice {
    pub fn remaining(&self) -> f64 {
        self.amount - self.paid
    }

    pub fn payment_count(&self) -> usize {
        self.payments.len()
    }

    pub fn status(&self) -> InvoiceStatus {
        if self.paid == 0.0 {
            InvoiceStatus::Unpaid
        } else if self.paid < self.amount {
            InvoiceStatus::PartiallyPaid
        } else {
            InvoiceStatus::Paid
        }
    }

    pub fn display(&self) {
        println!();
        println!("Invoice ID : {}", self.id);
        println!("Customer   : {}", self.customer);
        println!("Amount     : {:.2}", self.amount);
        println!("Paid       : {:.2}", self.paid);
        println!("Remaining  : {:.2}", self.remaining());
        println!("Payments   : {}", self.payment_count());

        for payment in &self.payments {
            println!("  - Payment #{}: {:.2}", payment.id, payment.amount);
        }

        match self.status() {
            InvoiceStatus::Unpaid => println!("Status     : Unpaid"),
            InvoiceStatus::PartiallyPaid => println!("Status     : Partially Paid"),
            InvoiceStatus::Paid => println!("Status     : Paid"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn invoice(paid: f64, payments: Vec<Payment>) -> Invoice {
        Invoice {
            id: 1001,
            customer: "Ada".to_string(),
            amount: 100.0,
            paid,
            payments,
        }
    }

    #[test]
    fn remaining_is_amount_minus_paid() {
        let invoice = invoice(40.0, Vec::new());

        assert_eq!(invoice.remaining(), 60.0);
    }

    #[test]
    fn payment_count_matches_stored_payments() {
        let invoice = invoice(
            25.0,
            vec![Payment {
                id: 1,
                amount: 25.0,
                invoice_id: 1001,
            }],
        );

        assert_eq!(invoice.payment_count(), 1);
    }

    #[test]
    fn status_follows_how_much_is_paid() {
        assert_eq!(invoice(0.0, Vec::new()).status(), InvoiceStatus::Unpaid);
        assert_eq!(invoice(40.0, Vec::new()).status(), InvoiceStatus::PartiallyPaid);
        assert_eq!(invoice(100.0, Vec::new()).status(), InvoiceStatus::Paid);
    }
}