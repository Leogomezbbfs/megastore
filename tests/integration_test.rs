use conectastore::models::{Catalog, Product, Customer, Offer, UserSession};
use conectastore::graph::StoreGraph;
use conectastore::recommendation::Recommender;
use conectastore::algorithms::{merge_sort, binary_search, sum_prices_parallel};

#[test]
fn test_complete_store_flow() {
    // --- Unidade 4: RAII & Sessão de Usuário ---
    let session = UserSession::start("sess-integ-001", 100);
    assert_eq!(session.customer_id, 100);

    // --- Unidades 1 e 2: Catálogo e Grafo ---
    let mut catalog = Catalog::new();
    let mut graph = StoreGraph::new();

    let p1 = Product { id: 1, name: "Monitor 144Hz".into(), category: "Displays".into(), price: 120000 };
    let p2 = Product { id: 2, name: "Cabo DisplayPort".into(), category: "Cabos".into(), price: 5000 };
    let p3 = Product { id: 3, name: "Braço Articulado".into(), category: "Suportes".into(), price: 25000 };

    catalog.add_product(p1.clone());
    catalog.add_product(p2.clone());
    catalog.add_product(p3.clone());

    let customer = Customer { id: 100, name: "Carlos".into() };
    catalog.add_customer(customer);

    // Histórico de compras simulando co-ocorrência
    graph.record_purchase(100, 1);
    graph.record_purchase(100, 2);

    // Outro cliente compra Cabo e Braço
    graph.record_purchase(200, 2);
    graph.record_purchase(200, 3);

    // --- Recomendação BFS com Deduplicação ---
    let recommender = Recommender::new(&catalog, &graph);
    let recs_for_p1 = recommender.recommend_from_product(1, 3);
    
    // Deve sugerir o Cabo (conexão direta) e não a si mesmo
    assert!(!recs_for_p1.is_empty());
    assert_eq!(recs_for_p1[0].id, 2);

    // --- Unidade 3: Algoritmos e Pipelines ---
    let prices: Vec<u64> = catalog.all_products().iter().map(|p| p.price).collect();
    let total_parallel = sum_prices_parallel(&prices);
    assert_eq!(total_parallel, 150000);

    let ids: Vec<u32> = vec![3, 1, 2];
    let sorted_ids = merge_sort(&ids);
    assert_eq!(sorted_ids, vec![1, 2, 3]);

    assert_eq!(binary_search(&sorted_ids, 2), Some(1));

    // --- Unidade 4: BinaryHeap (Ofertas) ---
    catalog.add_offer(Offer { product_id: 1, discount_percent: 10 });
    catalog.add_offer(Offer { product_id: 2, discount_percent: 40 });
    
    let best_offer = catalog.pop_best_offer().unwrap();
    assert_eq!(best_offer.discount_percent, 40);
    assert_eq!(best_offer.product_id, 2);
}