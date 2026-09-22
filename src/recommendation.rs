use std::collections::{HashSet, VecDeque};
use crate::graph::StoreGraph;
use crate::models::{Catalog, Product};

pub struct Recommender<'a> {
    catalog: &'a Catalog,
    graph: &'a StoreGraph,
}

impl<'a> Recommender<'a> {
    pub fn new(catalog: &'a Catalog, graph: &'a StoreGraph) -> Self {
        Self { catalog, graph }
    }

    /// Recomenda produtos a partir de um produto inicial usando BFS
    pub fn recommend_from_product(&self, start_product_id: u32, max_recommendations: usize) -> Vec<Product> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut recommendations = Vec::new();

        visited.insert(start_product_id);
        queue.push_back(start_product_id);

        while let Some(current_id) = queue.pop_front() {
            if let Some(neighbors) = self.graph.get_connected_products(current_id) {
                for &neighbor_id in neighbors {
                    if visited.insert(neighbor_id) {
                        if let Some(product) = self.catalog.get_product(neighbor_id) {
                            recommendations.push(product.clone());
                            if recommendations.len() >= max_recommendations {
                                return recommendations;
                            }
                        }
                        queue.push_back(neighbor_id);
                    }
                }
            }
        }

        recommendations
    }

    /// Recomenda produtos para um cliente, excluindo os já comprados
    pub fn recommend_for_customer(&self, customer_id: u32, max_recommendations: usize) -> Vec<Product> {
        let purchases = match self.graph.get_customer_purchases(customer_id) {
            Some(p) if !p.is_empty() => p,
            _ => return Vec::new(),
        };

        let mut visited: HashSet<u32> = purchases.iter().copied().collect();
        let mut queue = VecDeque::new();
        let mut recommendations = Vec::new();

        for &bought_id in purchases {
            queue.push_back(bought_id);
        }

        while let Some(current_id) = queue.pop_front() {
            if let Some(neighbors) = self.graph.get_connected_products(current_id) {
                for &neighbor_id in neighbors {
                    if visited.insert(neighbor_id) {
                        if let Some(product) = self.catalog.get_product(neighbor_id) {
                            recommendations.push(product.clone());
                            if recommendations.len() >= max_recommendations {
                                return recommendations;
                            }
                        }
                        queue.push_back(neighbor_id);
                    }
                }
            }
        }

        recommendations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recommendations_and_deduplication() {
        let mut catalog = Catalog::new();
        let mut graph = StoreGraph::new();

        // Cadastra 4 produtos com o campo price adicionado
        for id in 1..=4 {
            catalog.add_product(Product {
                id,
                name: format!("Item {}", id),
                category: "Tech".to_string(),
                price: id as u64 * 1000,
            });
        }

        // Cliente 1 comprou 1, 2 e 3
        graph.record_purchase(1, 1);
        graph.record_purchase(1, 2);
        graph.record_purchase(1, 3);

        // Cliente 2 comprou 3 e 4
        graph.record_purchase(2, 3);
        graph.record_purchase(2, 4);

        let recommender = Recommender::new(&catalog, &graph);

        // Recomendações para quem consulta o Produto 1
        let recs_prod = recommender.recommend_from_product(1, 5);
        let rec_ids: Vec<u32> = recs_prod.iter().map(|p| p.id).collect();

        assert!(!rec_ids.contains(&1));
        assert!(rec_ids.contains(&2));
        assert!(rec_ids.contains(&3));

        // Recomendações para o Cliente 2
        let recs_cust = recommender.recommend_for_customer(2, 5);
        let cust_rec_ids: Vec<u32> = recs_cust.iter().map(|p| p.id).collect();

        assert!(!cust_rec_ids.contains(&3));
        assert!(!cust_rec_ids.contains(&4));
        assert!(cust_rec_ids.contains(&1) || cust_rec_ids.contains(&2));
    }
}