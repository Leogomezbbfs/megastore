use conectastore::models::{Catalog, Product, Customer};
use conectastore::graph::StoreGraph;
use conectastore::recommendation::Recommender;

#[test]
fn test_full_recommendation_flow() {
    let mut catalog = Catalog::new();
    let mut graph = StoreGraph::new();

    // 1. Cadastrar produtos
    let p1 = Product { id: 101, name: "Teclado Mecânico".into(), category: "Periféricos".into() };
    let p2 = Product { id: 102, name: "Rato Gamer".into(), category: "Periféricos".into() };
    let p3 = Product { id: 103, name: "Tapete XL".into(), category: "Acessórios".into() };
    
    catalog.add_product(p1.clone());
    catalog.add_product(p2.clone());
    catalog.add_product(p3.clone());

    // 2. Cadastrar clientes e histórico de compras
    let c1 = Customer { id: 1, name: "Ana".into() };
    let c2 = Customer { id: 2, name: "Bruno".into() };
    catalog.add_customer(c1);
    catalog.add_customer(c2);

    // Ana comprou Teclado e Rato
    graph.record_purchase(1, 101);
    graph.record_purchase(1, 102);

    // Bruno comprou Rato e Tapete
    graph.record_purchase(2, 102);
    graph.record_purchase(2, 103);

    // 3. Gerar recomendações para quem comprou apenas o Teclado (101)
    let recommender = Recommender::new(&catalog, &graph);
    let recs = recommender.recommend_from_product(101, 2);

    assert!(!recs.is_empty());
    assert_eq!(recs[0].id, 102); // O vizinho direto mais próximo é o Rato Gamer
}