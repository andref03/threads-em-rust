# 🧵 Threads em Rust com Sincronização

Este projeto é uma simulação simples de múltiplos **cozinheiros (threads)** preparando **pedidos** de forma concorrente em Rust, utilizando os mecanismos de **sincronização `Arc` e `Mutex`** para garantir acesso seguro aos dados compartilhados.

## 📋 Descrição

- `n` cozinheiros são criados como threads.
- Todos compartilham uma fila de pedidos a serem preparados.
- Cada cozinheiro retira um pedido da fila, simula o tempo de preparo e o adiciona à lista de pedidos prontos.
- A sincronização entre as threads é feita com `Arc<Mutex<T>>`, garantindo que **apenas uma thread por vez** possa acessar/modificar os vetores compartilhados.

## 🔧 Tecnologias

- Linguagem: [Rust](https://www.rust-lang.org/)
- Concorrência: `thread`, `Arc`, `Mutex`
- Geração aleatória: [`rand`](https://docs.rs/rand/latest/rand/)

## 🚀 Execução

### 1. Clone o repositório

```bash
git clone git@github.com:andref03/threads-em-rust.git
cd threads-em-rust
```

### 2. Instale a dependência

Garanta que esteja adicionado ao `Cargo.toml`:

```toml
[dependencies]
rand = "0.8"
```

### 3. Como usar o código

Este projeto foi feito para demonstrar concorrência segura em Rust. Ao executá-lo:

- Altere os valores atribuídos às variáveis `num_pedidos` e `num_cozinheiros` se preferir.
- O programa cria várias threads que simulam cozinheiros.
- Os cozinheiros compartilham uma fila de pedidos protegida por `Mutex`.
- Cada thread remove um pedido da fila, espera de 1 a 5 segundos, e registra o pedido como pronto.
- Ao final, o programa mostra o tempo total de execução e valida se todos os pedidos foram concluídos.

### 4. Saída esperada

```text
Cozinheiro 0 pronto para receber pedidos!
Cozinheiro 1 pronto para receber pedidos!
Cozinheiro 2 pronto para receber pedidos!
Cozinheiro 3 pronto para receber pedidos!
Cozinheiro 4 pronto para receber pedidos!
Cozinheiro 1 preparando pedido=0
Cozinheiro 0 preparando pedido=1
Cozinheiro 2 preparando pedido=2
...
Cozinheiro 3 terminou pedido=97 (tempo de preparo: 1290ms)
Cozinheiro 1 terminou pedido=98 (tempo de preparo: 1770ms)
Cozinheiro 4 terminou pedido=99 (tempo de preparo: 2267ms)
Tempo total: 20.17s segundos
Todos os 100 pedidos foram preparados!
```

## 👥 Equipe

- André Felipe
- Bernardo Ruas
- Heuller Ramos
- Gustavo dos Santos
