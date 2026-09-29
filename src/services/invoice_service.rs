use crate::models::invoice::Invoice;
use crate::models::payment::Payment;
use crate::ui::input::{
    read_f64,
    read_u32
};
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

pub fn find_invoice(invoices: &[Invoice], id: u32) -> Option<&Invoice> {
    for invoice in invoices {
        if invoice.id == id {
            return Some(invoice);
        }
    }

    None
}

pub fn find_invoice_mut(invoices: &mut [Invoice],id: u32,) -> Option<&mut Invoice> {
    for invoice in invoices {
        if invoice.id == id {
            return Some(invoice);
        }
    }

    None
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

    match find_invoice(invoices, id) {
        Some(invoice) => invoice.display(),
        None => println!("Invoice not found."),
    }
}

pub fn record_payment(invoices: &mut [Invoice]) {
    println!();
    println!("=== Record Payment ===");

    let id = read_u32("Invoice ID: ");

    let invoice = match find_invoice_mut(invoices, id) {
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

    let remaining = invoice.remaining();

    println!("Invoice amount : {:.2}", invoice.amount);
    println!("Already paid   : {:.2}", invoice.paid);
    println!("Remaining      : {:.2}", remaining);

    let payment = read_f64("Payment amount: ");

    if payment == 0.0 {
        println!("Payment must be greater than zero.");
        return;
    }

    if payment > remaining {
        println!(
            "Payment cannot be greater than the remaining amount ({:.2}).",
            remaining
        );
        return;
    }

    let payment_id = invoice.payment_count() as u32 + 1;

    invoice.payments.push(Payment {
        id: payment_id,
        amount: payment,
        invoice_id: invoice.id,
    });
    invoice.paid += payment;

    println!();
    println!("Payment recorded successfully.");

    invoice.display();
}


pub fn invoice_payments(invoices: &[Invoice]) {
    println!();
    println!("=== Invoice Payments ===");

    let id = read_u32("Invoice ID: ");

    let invoice = match find_invoice(invoices, id) {
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