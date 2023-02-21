use std::collections::HashMap;
/// Bring Book struct into scope
use crate::cars::Cars;
/// Use once_cell for creating a global variable e.g. our DATA data.
use once_cell::sync::Lazy;

/// Use Mutex for thread-safe access to a variable e.g. our DATA data.
use std::sync::Mutex;

/// Create a data store as a global variable with `Lazy` and `Mutex`.
/// This demo implementation uses a `HashMap` for ease and speed.
/// The map key is a primary key for lookup; the map value is a Book.
pub static DATA: Lazy<Mutex<HashMap<u32, Cars>>> = Lazy::new(|| Mutex::new(
    HashMap::from([
        (1, Cars { 
            id: 1, 
            name: "City".into(), 
            brand: "Honda".into()
        }),
        (2, Cars { 
            id: 2, 
            name: "Civic".into(), 
            brand: "Honda".into()
        }),
        (3, Cars { 
            id: 3, 
            name: "Jazz".into(), 
            brand: "Honda".into()
        }),
    ])
));