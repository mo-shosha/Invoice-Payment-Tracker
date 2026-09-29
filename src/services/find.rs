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
