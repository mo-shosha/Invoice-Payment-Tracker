mod models;
mod services;
mod ui;

use models::invoice::Invoice;
use services::invoice_service;
use ui::input::read_input;
use ui::menu::show_menu;
use ui::input::read_f64;


fn main() {
    let mut invoices: Vec<Invoice> = Vec::new();
    let mut next_id: u32 = 1001;

    loop {
        show_menu();

        let choice = read_input("Choose an option: ");

        match choice.as_str() {
            "1" => {
                let customer = read_input("Customer name: ");
                let amount = read_f64("Invoice amount: ");

                invoice_service::add_invoice(
                    &mut invoices,
                    &mut next_id,
                    customer,
                    amount,
                );

                println!("Invoice created successfully.");
            }

            "2" => {
                invoice_service::list_invoices(&invoices);
            }

            "3" => {
                invoice_service::record_payment(&mut invoices);
            }

            "4" => {
                invoice_service::show_invoice(&invoices);
            }

            "5" => {
                invoice_service::invoice_payments(&invoices);
            }

            "6" => {
                println!("باى من غير سلام...........");
                break;
            }

            _ => {
                println!("Invalid option. Please choose from 1 to 6.");
            }
        }
    }
}