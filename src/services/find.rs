use crate::models::identifiable::Identifiable;

pub fn find_by_id<T: Identifiable>(items: &[T], id: u32) -> Option<&T> {
    for item in items {
        if item.id() == id {
            return Some(item);
        }
    }

    None
}

pub fn find_by_id_mut<T: Identifiable>(items: &mut [T], id: u32) -> Option<&mut T> {
    for item in items {
        if item.id() == id {
            return Some(item);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::invoice::Invoice;
    use crate::models::payment::Payment;

    fn invoice(id: u32) -> Invoice {
        Invoice {
            id,
            customer: format!("Customer {id}"),
            amount: 50.0,
            paid: 0.0,
            payments: Vec::new(),
        }
    }

    fn payment(id: u32) -> Payment {
        Payment {
            id,
            amount: 10.0,
            invoice_id: 1001,
        }
    }

    #[test]
    fn finds_an_invoice_by_id() {
        let invoices = vec![invoice(1001), invoice(1002)];

        let found = find_by_id(&invoices, 1002).unwrap();

        assert_eq!(found.customer, "Customer 1002");
    }

    #[test]
    fn returns_none_when_the_id_is_missing() {
        let invoices = vec![invoice(1001)];

        assert!(find_by_id(&invoices, 9999).is_none());
    }

    #[test]
    fn finds_a_payment_with_the_same_function() {
        let payments = vec![payment(1), payment(2)];

        let found = find_by_id(&payments, 2).unwrap();

        assert_eq!(found.amount, 10.0);
        assert_eq!(found.invoice_id, 1001);
    }

    #[test]
    fn find_mut_updates_the_matching_item() {
        let mut invoices = vec![invoice(1001), invoice(1002)];

        find_by_id_mut(&mut invoices, 1001).unwrap().customer = "Updated".to_string();

        assert_eq!(invoices[0].customer, "Updated");
        assert_eq!(invoices[1].customer, "Customer 1002");
    }
}
