use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
pub struct StoreGraph {
    // Mapeia o ID de um cliente para os IDs dos produtos comprados
    customer_purchases: HashMap<u32, HashSet<u32>>,
    // Mapeia o ID de um produto para outros produtos comprados em conjunto (co-ocorrência)
    product_connections: HashMap<u32, HashSet<u32>>,
}

impl StoreGraph {
    pub fn new() -> Self {
        Self {
            customer_purchases: HashMap::new(),
            product_connections: HashMap::new(),
        }
    }

    /// Registra a compra de um produto por um cliente e cria as conexões com produtos anteriores
    pub fn record_purchase(&mut self, customer_id: u32, product_id: u32) {
        let purchases = self.customer_purchases.entry(customer_id).or_default();

        // Conecta o novo produto bidirecionalmente a todos os produtos já comprados por este cliente
        for &past_product_id in purchases.iter() {
            if past_product_id != product_id {
                self.product_connections
                    .entry(past_product_id)
                    .or_default()
                    .insert(product_id);
                self.product_connections
                    .entry(product_id)
                    .or_default()
                    .insert(past_product_id);
            }
        }

        purchases.insert(product_id);
    }

    /// Retorna os produtos comprados por um cliente específico
    pub fn get_customer_purchases(&self, customer_id: u32) -> Option<&HashSet<u32>> {
        self.customer_purchases.get(&customer_id)
    }

    /// Retorna os vizinhos diretos conectados a um produto
    pub fn get_connected_products(&self, product_id: u32) -> Option<&HashSet<u32>> {
        self.product_connections.get(&product_id)
    }

    /// Retorna todos os nós de produtos presentes no grafo
    pub fn all_product_nodes(&self) -> Vec<u32> {
        self.product_connections.keys().copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_connections() {
        let mut graph = StoreGraph::new();

        // Cliente 1 compra produtos 10, 20 e 30
        graph.record_purchase(1, 10);
        graph.record_purchase(1, 20);
        graph.record_purchase(1, 30);

        let connections_10 = graph.get_connected_products(10).unwrap();
        assert!(connections_10.contains(&20));
        assert!(connections_10.contains(&30));

        let connections_20 = graph.get_connected_products(20).unwrap();
        assert!(connections_20.contains(&10));
        assert!(connections_20.contains(&30));

        let customer_purchases = graph.get_customer_purchases(1).unwrap();
        assert_eq!(customer_purchases.len(), 3);
    }
}