use std::time::Instant;
use conectastore::models::{Catalog, Product, Offer, UserSession};
use conectastore::graph::StoreGraph;
use conectastore::recommendation::Recommender;
use conectastore::algorithms::{merge_sort, quick_sort, binary_search, interpolation_search, sum_prices_parallel};

fn main() {
    println!("============================================================");
    println!("       CONECTASTORE: SISTEMA DE RECOMENDAÇÃO EM RUST        ");
    println!("============================================================\n");

    // 1. Demonstração de RAII (Unidade 4)
    {
        let _user_session = UserSession::start("sess_main_01", 42);
        println!("Processando requisições com sessão ativa...");
    } // Aqui o drop() é executado automaticamente

    println!("\n--- DEMONSTRAÇÃO DE FILA DE PRIORIDADE (BinaryHeap) ---");
    let mut catalog = Catalog::new();
    catalog.add_offer(Offer { product_id: 101, discount_percent: 10 });
    catalog.add_offer(Offer { product_id: 102, discount_percent: 70 });
    catalog.add_offer(Offer { product_id: 103, discount_percent: 30 });

    if let Some(best) = catalog.pop_best_offer() {
        println!("Melhor Oferta do Dia (Max-Heap): Produto #{} com {}% OFF!", best.product_id, best.discount_percent);
    }

    println!("\n--- BENCHMARK DE ALGORITMOS E ESCALABILIDADE (Unidades 1, 2 e 3) ---");
    let sizes = [1_000, 10_000, 50_000];

    for &size in &sizes {
        println!("\n>>> Cenário com {} Elementos:", size);

        // Povoamento do catálogo e montagem do grafo
        let mut test_catalog = Catalog::new();
        let mut graph = StoreGraph::new();
        let mut ids: Vec<u32> = (1..=size).rev().collect();

        for id in 1..=size {
            test_catalog.add_product(Product {
                id,
                name: format!("Item {}", id),
                category: format!("Cat {}", id % 5),
                price: (id as u64) * 100,
            });
        }

        for i in 1..=(size / 10) {
            graph.record_purchase(i, (i % size) + 1);
            graph.record_purchase(i, ((i + 1) % size) + 1);
        }

        // Benchmark Recomendação BFS (Unidades 1 e 2)
        let recommender = Recommender::new(&test_catalog, &graph);
        let t_bfs = Instant::now();
        let recs = recommender.recommend_from_product(1, 5);
        println!("   [Recomendação BFS]     Tempo: {:?} ({} itens)", t_bfs.elapsed(), recs.len());

        // Benchmark Merge Sort
        let t0 = Instant::now();
        let sorted_merge = merge_sort(&ids);
        println!("   [Merge Sort]           Tempo: {:?}", t0.elapsed());

        // Benchmark Quick Sort Otimizado
        let t1 = Instant::now();
        quick_sort(&mut ids);
        println!("   [Quick Sort]           Tempo: {:?}", t1.elapsed());

        // Benchmark Buscas
        let target = size / 2;
        let t2 = Instant::now();
        let _ = binary_search(&sorted_merge, target);
        println!("   [Busca Binária]        Tempo: {:?}", t2.elapsed());

        let t3 = Instant::now();
        let _ = interpolation_search(&sorted_merge, target);
        println!("   [Busca Interpolada]    Tempo: {:?}", t3.elapsed());

        // Benchmark Rayon (Paralelo)
        let prices: Vec<u64> = vec![990; size as usize];
        let t4 = Instant::now();
        let _ = sum_prices_parallel(&prices);
        println!("   [Soma Paralela Rayon]  Tempo: {:?}", t4.elapsed());
    }

    println!("\nExecução concluída com sucesso.");
}