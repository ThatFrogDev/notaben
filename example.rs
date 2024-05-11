// Define a struct to represent a Customer
#[derive(Debug)]
struct Customer {
    id: u32,
    name: String,
    address: String,
    email: String,
    phone_number: String,
}

// Define a struct to represent the CustomerDatabase
struct CustomerDatabase {
    customers: Vec<Customer>,
}

impl CustomerDatabase {
    // Function to search for a customer by name
    fn find_customer_by_name(&self, name: &str) -> Option<&Customer> {
        self.customers.iter().find(|customer| customer.name == name)
    }
}

fn main() {
    // Create a sample customer database
    let customer1 = Customer {
        id: 1,
        name: String::from("John Doe"),
        address: String::from("123 Main St"),
        email: String::from("john.doe@example.com"),
        phone_number: String::from("555-1234"),
    };

    let customer2 = Customer {
        id: 2,
        name: String::from("Jane Smith"),
        address: String::from("456 Elm St"),
        email: String::from("jane.smith@example.com"),
        phone_number: String::from("555-5678"),
    };

    let database = CustomerDatabase {
        customers: vec![customer1, customer2],
    };

    // Search for a customer by name
    let search_name = String::from("John Doe");
    if let Some(customer) = database.find_customer_by_name(&search_name) {
        println!("Customer found: {:?}", customer);
    } else {
        println!("Customer not found.");
    }
}
