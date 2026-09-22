use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub category: String,
    pub price: u64, // Em centavos para evitar ponto flutuante
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Customer {
    pub id: u32,
    pub name: String,
}

// Unidade 4: Estrutura para Fila de Prioridade (BinaryHeap - Max Heap)
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Offer {
    pub product_id: u32,
    pub discount_percent: u8,
}

impl Ord for Offer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.discount_percent.cmp(&other.discount_percent)
    }
}

impl PartialOrd for Offer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Unidade 4: RAII (Resource Acquisition Is Initialization) com trait Drop
#[derive(Debug)]
pub struct UserSession {
    pub session_id: String,
    pub customer_id: u32,
}

impl UserSession {
    pub fn start(session_id: &str, customer_id: u32) -> Self {
        println!("[RAII] Sessão '{}' iniciada para o cliente {}.", session_id, customer_id);
        Self {
            session_id: session_id.to_string(),
            customer_id,
        }
    }
}

impl Drop for UserSession {
    fn drop(&mut self) {
        println!("[RAII] Sessão '{}' finalizada. Liberando recursos da memória.", self.session_id);
    }
}

#[derive(Debug, Default)]
pub struct Catalog {
    products: HashMap<u32, Product>,
    customers: HashMap<u32, Customer>,
    offers_heap: BinaryHeap<Offer>,
}

impl Catalog {
    pub fn new() -> Self {
        Self {
            products: HashMap::new(),
            customers: HashMap::new(),
            offers_heap: BinaryHeap::new(),
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

    pub fn add_offer(&mut self, offer: Offer) {
        self.offers_heap.push(offer);
    }

    pub fn pop_best_offer(&mut self) -> Option<Offer> {
        self.offers_heap.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog_and_raii_session() {
        let mut catalog = Catalog::new();
        catalog.add_product(Product {
            id: 1,
            name: "Mouse Gamer".to_string(),
            category: "Periféricos".to_string(),
            price: 15000,
        });

        assert!(catalog.get_product(1).is_some());

        // Testa o ciclo de vida RAII
        {
            let _session = UserSession::start("sess-01", 1);
        } // Ao sair deste bloco, o drop(&mut self) é invocado automaticamente
    }

    #[test]
    fn test_binary_heap_offers() {
        let mut catalog = Catalog::new();
        catalog.add_offer(Offer { product_id: 1, discount_percent: 15 });
        catalog.add_offer(Offer { product_id: 2, discount_percent: 50 });
        catalog.add_offer(Offer { product_id: 3, discount_percent: 30 });

        // O Max-Heap deve desempilhar a maior porcentagem de desconto primeiro
        let best = catalog.pop_best_offer().unwrap();
        assert_eq!(best.discount_percent, 50);
        assert_eq!(best.product_id, 2);
    }
}