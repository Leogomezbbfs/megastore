use std::time::Instant;
use conectastore::models::{Catalog, Product};
use conectastore::graph::StoreGraph;
use conectastore::recommendation::Recommender;

fn benchmark_recommendations(num_products: u32, purchases_per_customer: u32) {
    let mut catalog = Catalog::new();
    let mut graph = StoreGraph::new();

    println!("--- Benchmark: {} Produtos ---", num_products);

    // 1. Povoamento de dados
    let start_setup = Instant::now();
    for id in 1..=num_products {
        catalog.add_product(Product {
            id,
            name: format!("Produto {}", id),
            category: format!("Categoria {}", id % 10),
        });
    }

    // Simula clientes comprando blocos de produtos para formar arestas
    let num_customers = num_products / 5;
    for c in 1..=num_customers {
        for offset in 0..purchases_per_customer {
            let prod_id = ((c + offset) % num_products) + 1;
            graph.record_purchase(c, prod_id);
        }
    }
    let duration_setup = start_setup.elapsed();
    println!("Tempo de montagem do grafo: {:?}", duration_setup);

    // 2. Execução da travessia BFS
    let recommender = Recommender::new(&catalog, &graph);
    let start_rec = Instant::now();
    let results = recommender.recommend_from_product(1, 10);
    let duration_rec = start_rec.elapsed();

    println!("Tempo para calcular recomendação BFS: {:?}", duration_rec);
    println!("Itens recomendados: {}\n", results.len());
}

fn main() {
    println!("=== ConectaStore: Sistema de Recomendação ===\n");

    // Executa medição com volume pequeno
    benchmark_recommendations(100, 3);

    // Executa medição com volume médio
    benchmark_recommendations(1_000, 5);

    // Executa medição com volume maior
    benchmark_recommendations(10_000, 8);
}