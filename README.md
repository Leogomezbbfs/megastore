# ConectaStore - Sistema de Recomendação em Rust

Sistema de recomendação e processamento de catálogo em alta performance desenvolvido em Rust, cobrindo estruturas de dados avançadas, grafos, algoritmos clássicos de ordenação/busca, paralelismo de dados e gerenciamento seguro de memória via RAII.

---

## 1. Objetivo e Funcionamento do Sistema
O objetivo do **ConectaStore** é fornecer sugestões colaborativas de produtos em tempo hábil para e-commerce. A solução constrói um grafo de co-ocorrência a partir do histórico de compras dos clientes e emprega travessia em largura (BFS) para sugerir conexões de produtos evitando repetições ou auto-recomendações.

O sistema também inclui:
- Gerenciamento de catálogo indexado via tabelas hash.
- Fila de prioridades (*Max-Heap*) para destaque das melhores ofertas.
- Pipeline de algoritmos clássicos de ordenação e busca para relatórios internos.
- Processamento concorrente com divisão paralela de tarefas.

---

## 2. Tecnologias e Estruturas Utilizadas
- **Linguagem:** Rust (Edição 2024).
- **Grafos & Indexação:** `HashMap` e `HashSet` (Listas de Adjacência em $O(1)$).
- **Travessia & Filas:** `VecDeque` (Fila FIFO para execução do algoritmo BFS).
- **Fila de Prioridade:** `BinaryHeap` (Max-Heap para priorização de descontos por `Ord`/`PartialOrd`).
- **Algoritmos Clássicos:** Merge Sort recursivo estável ($O(n \log n)$), Quick Sort particionado in-place com profundidade de pilha limitada a $O(\log n)$, Busca Binária ($O(\log n)$) e Busca Interpolada ($O(\log \log n)$ em média).
- **Paralelismo:** Crate `rayon` para iteração paralela (`.par_iter()`) e ordenação concorrente.
- **Gerenciamento de Memória & RAII:** Trait `Drop` em `UserSession` para liberação determinística de recursos de escopo sem Garbage Collector.

---

## 3. Arquitetura da Solução

text
megastore/
├── src/
│   ├── lib.rs              # Exportação dos módulos da biblioteca
│   ├── main.rs             # CLI, demonstração de RAII/Heap e benchmarks
│   ├── models.rs           # Entidades (Product, Customer, Offer, UserSession, Catalog)
│   ├── graph.rs            # Grafo de adjacência e conexões de co-ocorrência
│   ├── recommendation.rs   # Motor de recomendação BFS com deduplicação
│   └── algorithms.rs       # Merge Sort, Quick Sort, Buscas e Rayon
├── tests/
│   └── integration_test.rs # Teste de integração de fluxo ponta a ponta
├── Cargo.toml              # Metadados e dependências (Rayon)
└── README.md               # Documentação técnica e resultados

---

## 4. Instruções para Compilação e Execução

### Pré-requisitos
- Rust (Cargo) instalado via `rustup`.

### Compilar e Executar a Demonstração / Benchmarks
bash
cargo run

---

## 5. Instruções para Execução dos Testes

### Executar Testes Unitários e de Integração
bash
cargo test

Para inspecionar saídas de console detalhadas durante os testes:
bash
cargo test -- --nocapture

---

## 6. Exemplos de Uso
rust
use conectastore::models::{Catalog, Product};
use conectastore::graph::StoreGraph;
use conectastore::recommendation::Recommender;

// 1. Instanciação
let mut catalog = Catalog::new();
let mut graph = StoreGraph::new();

// 2. Cadastro de Produtos
catalog.add_product(Product {
id: 1,
name: "Teclado Mecânico".into(),
category: "Periféricos".into(),
price: 25000,
});

// 3. Registro de Compras (Arestas no Grafo)
graph.record_purchase(100, 1);
graph.record_purchase(100, 2);

// 4. Recomendação com BFS
let recommender = Recommender::new(&catalog, &graph);
let sugestoes = recommender.recommend_from_product(1, 5);

---

## 7. Resultados dos Testes de Desempenho

Métricas coletadas na execução de `cargo run` com volumes crescentes de dados:

| Operação / Algoritmo | 1.000 Elementos | 10.000 Elementos | 50.000 Elementos |
| :--- | :--- | :--- | :--- |
| **Recomendação BFS** | ~7.0 µs | ~15.8 µs | ~21.0 µs |
| **Merge Sort** | 1.55 ms | 11.95 ms | 66.67 ms |
| **Quick Sort (Otimizado)** | 1.20 ms | 22.23 ms | 75.43 ms |
| **Busca Binária** | 2.4 µs | 3.0 µs | 4.4 µs |
| **Busca Interpolada** | 1.9 µs | 6.2 µs | 2.5 µs |
| **Soma Paralela (Rayon)** | 2.03 ms | 4.01 ms | 995.5 µs |

---

## 8. Link do Vídeo Pitch
- **Vídeo de Apresentação:** `[Inserir link público do vídeo aqui]`