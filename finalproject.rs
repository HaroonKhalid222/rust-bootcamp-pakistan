use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: u32,
}

struct Inventory {
    products: HashMap<u32, Product>,
    next_id: u32,
}

impl Inventory {
    fn new() -> Inventory {
        Inventory {
            products: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_product(&mut self, product: Product) {
        self.products.insert(self.next_id, product);
        self.next_id += 1;
    }

    fn edit_product(&mut self, id: u32, updated_product: Product) -> Result<(), &'static str> {
        if let Some(product) = self.products.get_mut(&id) {
            *product = updated_product;
            Ok(())
        } else {
            Err("Product not found.")
        }
    }

    fn delete_product(&mut self, id: u32) -> Result<(), &'static str> {
        if self.products.remove(&id).is_some() {
            Ok(())
        } else {
            Err("Product not found.")
        }
    }

    fn generate_report(&self) {
        println!("Inventory Report:");
        println!("{:<5} {:<15} {:<20} {:<10} {:<10}", "ID", "Name", "Description", "Price", "Quantity");
        for (id, product) in &self.products {
            println!(
                "{:<5} {:<15} {:<20} {:<10} {:<10}",
                id, product.name, product.description, product.price, product.quantity
            );
        }
    }
}

fn main() {
    let mut inventory = Inventory::new();

    loop {
        println!("Options:");
        println!("1. Add Product");
        println!("2. Edit Product");
        println!("3. Delete Product");
        println!("4. Generate Report");
        println!("5. Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let choice: u32 = input.trim().parse().expect("Invalid input");

        match choice {
            1 => {
                let product = create_product_from_input();
                inventory.add_product(product);
                println!("Product added successfully!");
            }
            2 => {
                let id = get_product_id_from_input();
                let updated_product = create_product_from_input();
                match inventory.edit_product(id, updated_product) {
                    Ok(_) => println!("Product edited successfully!"),
                    Err(err) => println!("{}", err),
                }
            }
            3 => {
                let id = get_product_id_from_input();
                match inventory.delete_product(id) {
                    Ok(_) => println!("Product deleted successfully!"),
                    Err(err) => println!("{}", err),
                }
            }
            4 => {
                inventory.generate_report();
            }
            5 => {
                println!("Exiting Rusty Store Inventory Management System. Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice. Please choose a number from 1 to 5.");
            }
        }
    }
}

fn create_product_from_input() -> Product {
    println!("Enter product name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    println!("Enter product description:");
    let mut description = String::new();
    io::stdin().read_line(&mut description).expect("Failed to read input");

    println!("Enter product price:");
    let mut price_str = String::new();
    io::stdin().read_line(&mut price_str).expect("Failed to read input");
    let price: f64 = price_str.trim().parse().expect("Invalid input");

    println!("Enter product quantity:");
    let mut quantity_str = String::new();
    io::stdin().read_line(&mut quantity_str).expect("Failed to read input");
    let quantity: u32 = quantity_str.trim().parse().expect("Invalid input");

    Product {
        name: name.trim().to_string(),
        description: description.trim().to_string(),
        price,
        quantity,
    }
}

fn get_product_id_from_input() -> u32 {
    println!("Enter product ID:");
    let mut id_str = String::new();
    io::stdin().read_line(&mut id_str).expect("Failed to read input");
    id_str.trim().parse().expect("Invalid input")
}
