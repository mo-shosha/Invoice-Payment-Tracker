use std::io::{self, Write};

struct Invoice {
    id: u32,
    customer: String,
    amount: f64,
    paid: f64,
}

enum InvoiceStatus {
    Unpaid,
    PartiallyPaid,
    Paid,
}

impl Invoice {
    fn remaining(&self) -> f64 {
        self.amount - self.paid
    }

    fn status(&self) -> InvoiceStatus {
        if self.paid == 0.0 {
            InvoiceStatus::Unpaid
        } 
        else if self.paid < self.amount {
            InvoiceStatus::PartiallyPaid
        } 
        else {
            InvoiceStatus::Paid
        }
    }

    fn display(&self) {
        println!();
        println!("Invoice ID : {}", self.id);
        println!("Customer   : {}", self.customer);
        println!("Amount     : {:.2}", self.amount);
        println!("Paid       : {:.2}", self.paid);
        println!("Remaining  : {:.2}", self.remaining());

        match self.status() {
            InvoiceStatus::Unpaid => println!("Status     : Unpaid"),
            InvoiceStatus::PartiallyPaid => println!("Status     : Partially Paid"),
            InvoiceStatus::Paid => println!("Status     : Paid"),
        }
    }
}

fn read_input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

fn read_u32(message: &str) -> u32 {
    loop {
        let input = read_input(message);

        match input.parse::<u32>() {
            Ok(value) => return value,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

fn read_f64(message: &str) -> f64 {
    loop {
        let input = read_input(message);

        match input.parse::<f64>() {
            Ok(value) if value >= 0.0 => return value,
            _ => println!("Please enter a valid positive number."),
        }
    }
}

fn add_invoice(invoices: &mut Vec<Invoice>, next_id: &mut u32) {
    println!();
    println!("=== Add Invoice ===");

    let customer = read_input("Customer name: ");
    let amount = read_f64("Invoice amount: ");

    let invoice = Invoice {
        id: *next_id,
        customer,
        amount,
        paid: 0.0,
    };

    invoices.push(invoice);

    println!();
    println!("Invoice created successfully.");
    println!("Invoice ID: {}", *next_id);

    *next_id += 1;
}

fn list_invoices(invoices: &[Invoice]) {
    println!();
    println!("=== All Invoices ===");

    if invoices.is_empty() {
        println!("No invoices found.");
        return;
    }

    for invoice in invoices {
        println!(
            "#{} | {} | Total: {:.2} | Paid: {:.2} | Remaining: {:.2}",
            invoice.id,
            invoice.customer,
            invoice.amount,
            invoice.paid,
            invoice.remaining()
        );

        match invoice.status() {
            InvoiceStatus::Unpaid => println!("    Status: Unpaid"),
            InvoiceStatus::PartiallyPaid => println!("    Status: Partially Paid"),
            InvoiceStatus::Paid => println!("    Status: Paid"),
        }
    }
}

fn find_invoice(invoices: &[Invoice], id: u32) -> Option<&Invoice> {
    for invoice in invoices {
        if invoice.id == id {
            return Some(invoice);
        }
    }

    None
}

fn find_invoice_mut(invoices: &mut [Invoice],id: u32,) -> Option<&mut Invoice> {
    for invoice in invoices {
        if invoice.id == id {
            return Some(invoice);
        }
    }

    None
}

fn show_invoice(invoices: &[Invoice]) {
    println!();
    println!("=== Show Invoice ===");

    let id = read_u32("Invoice ID: ");

    match find_invoice(invoices, id) {
        Some(invoice) => invoice.display(),
        None => println!("Invoice not found."),
    }
}

fn record_payment(invoices: &mut [Invoice]) {
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

    invoice.paid += payment;

    println!();
    println!("Payment recorded successfully.");

    invoice.display();
}

fn show_menu() {
    println!();
    println!("=================================");
    println!("     Invoice & Payment Tracker");
    println!("=================================");
    println!("1. Add Invoice");
    println!("2. List Invoices");
    println!("3. Record Payment");
    println!("4. Show Invoice");
    println!("5. Exit");
    println!("=================================");
}

fn main() {
    let mut invoices: Vec<Invoice> = Vec::new();
    let mut next_id: u32 = 1001;

    loop {
        show_menu();

        let choice = read_input("Choose an option: ");

        match choice.as_str() {
            "1" => {
                add_invoice(&mut invoices, &mut next_id);
            }

            "2" => {
                list_invoices(&invoices);
            }

            "3" => {
                record_payment(&mut invoices);
            }

            "4" => {
                show_invoice(&invoices);
            }

            "5" => {
                println!("باى من غير سلام...........");
                break;
            }

            _ => {
                println!("Invalid option. Please choose from 1 to 5.");
            }
        }
    }
}