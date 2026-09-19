use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub category: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Customer {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Default)]
pub struct Catalog {
    products: HashMap<u32, Product>,
    customers: HashMap<u32, Customer>,
}

impl Catalog {
    pub fn new() -> Self {
        Self {
            products: HashMap::new(),
            customers: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        self.products.insert(product.id, product);
    }

    pub fn get_product(&self, id: u32) -> Option<&Product> {
        self.products.get(&id)
    }

    pub fn add_customer(&mut self, customer: Customer) {
        self.customers.insert(customer.id, customer);
    }

    pub fn get_customer(&self, id: u32) -> Option<&Customer> {
        self.customers.get(&id)
    }

    pub fn all_products(&self) -> Vec<&Product> {
        self.products.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_and_customer_registration() {
        let mut catalog = Catalog::new();

        let prod = Product {
            id: 1,
            name: "Teclado Mecânico".to_string(),
            category: "Periféricos".to_string(),
        };
        catalog.add_product(prod.clone());

        let cust = Customer {
            id: 101,
            name: "Leonardo".to_string(),
        };
        catalog.add_customer(cust.clone());

        assert_eq!(catalog.get_product(1), Some(&prod));
        assert_eq!(catalog.get_customer(101), Some(&cust));
        assert!(catalog.get_product(999).is_none());
    }
}