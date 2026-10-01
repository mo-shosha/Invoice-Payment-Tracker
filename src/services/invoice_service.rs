use crate::models::invoice::Invoice;
use crate::models::payment::Payment;
use crate::services::find::{find_by_id, find_by_id_mut};
use crate::ui::input::{read_f64, read_u32};

pub fn add_invoice(invoices: &mut Vec<Invoice>,next_id: &mut u32,customer: String,amount: f64,) {
    let invoice = Invoice {
        id: *next_id,
        customer,
        amount,
        paid: 0.0,
        payments: Vec::new(),
    };

    invoices.push(invoice);

    *next_id += 1;
}

pub fn list_invoices(invoices: &[Invoice]) {
    println!();
    println!("=== All Invoices ===");

    if invoices.is_empty() {
        println!("No invoices found.");
        return;
    }

    for invoice in invoices {
        println!(
            "#{} | {} | Total: {:.2} | Paid: {:.2} | Remaining: {:.2} | Payments: {}",
            invoice.id,
            invoice.customer,
            invoice.amount,
            invoice.paid,
            invoice.remaining(),
            invoice.payment_count()
        );
    }
}


pub fn show_invoice(invoices: &[Invoice]) {
    println!();
    println!("=== Show Invoice ===");

    let id = read_u32("Invoice ID: ");

    match find_by_id(invoices, id) {
        Some(invoice) => invoice.display(),
        None => println!("Invoice not found."),
    }
}

#[derive(Debug, PartialEq)]
enum PaymentError {
    NotFound,
    AlreadyPaid,
    ZeroAmount,
    ExceedsRemaining { remaining: f64 },
}

fn apply_payment(invoices: &mut [Invoice], id: u32, amount: f64) -> Result<(), PaymentError> {
    let invoice = find_by_id_mut(invoices, id).ok_or(PaymentError::NotFound)?;

    if invoice.paid >= invoice.amount {
        return Err(PaymentError::AlreadyPaid);
    }

    let remaining = invoice.remaining();

    if amount == 0.0 {
        return Err(PaymentError::ZeroAmount);
    }

    if amount > remaining {
        return Err(PaymentError::ExceedsRemaining { remaining });
    }

    let payment_id = invoice.payment_count() as u32 + 1;

    invoice.payments.push(Payment {
        id: payment_id,
        amount,
        invoice_id: invoice.id,
    });
    invoice.paid += amount;

    Ok(())
}

pub fn record_payment(invoices: &mut [Invoice]) {
    println!();
    println!("=== Record Payment ===");

    let id = read_u32("Invoice ID: ");

    let (amount, paid, remaining) = {
        let invoice = match find_by_id(invoices, id) {
            Some(invoice) => invoice,
            None => {
                println!("Invoice not found.");
                return;
            }
        };

        if invoice.paid >= invoice.amount {
            println!("This invoice is already fully paid.");
            return;
        }

        (invoice.amount, invoice.paid, invoice.remaining())
    };

    println!("Invoice amount : {:.2}", amount);
    println!("Already paid   : {:.2}", paid);
    println!("Remaining      : {:.2}", remaining);

    let payment = read_f64("Payment amount: ");

    match apply_payment(invoices, id, payment) {
        Ok(()) => {
            println!();
            println!("Payment recorded successfully.");

            if let Some(invoice) = find_by_id(invoices, id) {
                invoice.display();
            }
        }
        Err(PaymentError::ZeroAmount) => {
            println!("Payment must be greater than zero.");
        }
        Err(PaymentError::ExceedsRemaining { remaining }) => {
            println!(
                "Payment cannot be greater than the remaining amount ({:.2}).",
                remaining
            );
        }
        Err(PaymentError::NotFound) => println!("Invoice not found."),
        Err(PaymentError::AlreadyPaid) => {
            println!("This invoice is already fully paid.");
        }
    }
}


pub fn invoice_payments(invoices: &[Invoice]) {
    println!();
    println!("=== Invoice Payments ===");

    let id = read_u32("Invoice ID: ");

    let invoice = match find_by_id(invoices, id) {
        Some(invoice) => invoice,
        None => {
            println!("Invoice not found.");
            return;
        }
    };

    println!();
    println!("Invoice #{} | {}", invoice.id, invoice.customer);
    println!("Payments: {}", invoice.payment_count());

    if invoice.payments.is_empty() {
        println!("No payments recorded.");
        return;
    }

    for payment in &invoice.payments {
        println!(
            "Payment #{} | Amount: {:.2}",
            payment.id,
            payment.amount
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn add_sample(invoices: &mut Vec<Invoice>, next_id: &mut u32) {
        add_invoice(invoices, next_id, "Ada".to_string(), 100.0);
    }

    #[test]
    fn add_invoice_stores_an_unpaid_invoice_and_advances_the_id() {
        let mut invoices = Vec::new();
        let mut next_id = 1001;

        add_sample(&mut invoices, &mut next_id);
        add_invoice(&mut invoices, &mut next_id, "Grace".to_string(), 80.0);

        assert_eq!(invoices.len(), 2);
        assert_eq!(invoices[0].id, 1001);
        assert_eq!(invoices[0].customer, "Ada");
        assert_eq!(invoices[0].amount, 100.0);
        assert_eq!(invoices[0].paid, 0.0);
        assert!(invoices[0].payments.is_empty());
        assert_eq!(invoices[1].id, 1002);
        assert_eq!(next_id, 1003);
    }

    #[test]
    fn apply_payment_records_each_payment_and_updates_the_balance() {
        let mut invoices = Vec::new();
        let mut next_id = 1001;
        add_sample(&mut invoices, &mut next_id);

        apply_payment(&mut invoices, 1001, 40.0).unwrap();
        apply_payment(&mut invoices, 1001, 60.0).unwrap();

        let invoice = &invoices[0];
        assert_eq!(invoice.paid, 100.0);
        assert_eq!(invoice.remaining(), 0.0);
        assert_eq!(invoice.payment_count(), 2);
        assert_eq!(invoice.payments[0].id, 1);
        assert_eq!(invoice.payments[0].amount, 40.0);
        assert_eq!(invoice.payments[0].invoice_id, 1001);
        assert_eq!(invoice.payments[1].id, 2);
        assert_eq!(invoice.payments[1].amount, 60.0);
        assert_eq!(invoice.status(), crate::models::invoice::InvoiceStatus::Paid);
    }

    #[test]
    fn apply_payment_rejects_invalid_amounts_without_changing_the_invoice() {
        let mut invoices = Vec::new();
        let mut next_id = 1001;
        add_sample(&mut invoices, &mut next_id);
        apply_payment(&mut invoices, 1001, 40.0).unwrap();

        assert_eq!(
            apply_payment(&mut invoices, 1001, 0.0),
            Err(PaymentError::ZeroAmount)
        );
        assert_eq!(
            apply_payment(&mut invoices, 1001, 70.0),
            Err(PaymentError::ExceedsRemaining { remaining: 60.0 })
        );
        assert_eq!(
            apply_payment(&mut invoices, 9999, 10.0),
            Err(PaymentError::NotFound)
        );

        apply_payment(&mut invoices, 1001, 60.0).unwrap();

        assert_eq!(
            apply_payment(&mut invoices, 1001, 10.0),
            Err(PaymentError::AlreadyPaid)
        );
        assert_eq!(invoices[0].paid, 100.0);
        assert_eq!(invoices[0].payment_count(), 2);
    }
}