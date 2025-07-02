# ⚠️ Threads em Rust SEM Mutex (com Condição de Corrida)

Este projeto demonstra o uso de múltiplas threads em Rust para processar pedidos simultaneamente, **porém sem utilizar mecanismos de sincronização (`Mutex`)**, o que provoca condições de corrida (race conditions) e resultados imprevisíveis.

## 📋 Descrição

- `n` cozinheiros são criados como threads.
- Todos compartilham uma fila de pedidos global e uma lista de pedidos prontos, mas **sem proteção sincronizada**.
- Cada cozinheiro tenta remover um pedido da fila e, após preparar, adiciona à lista de pedidos prontos.
- Como não há controle de acesso, as threads podem acessar e modificar os dados compartilhados simultaneamente, causando erros ou dados corrompidos.

## 🔧 Tecnologias

- Linguagem: [Rust](https://www.rust-lang.org/)
- Concorrência: `thread`
- Uso de variáveis globais `static mut` sem sincronização

## 🚀 Execução

### 1. Clone o repositório

```bash
git clone git@github.com:andref03/threads-em-rust.git
cd threads-em-rust
```

### 2. Compile e execute

```bash
cargo run --bin sem_mutex
```

### 3. Como usar o código

- Altere os valores de num_pedidos e num_cozinheiros para testar diferentes cenários.
- O programa cria várias threads que simulam cozinheiros acessando uma fila global sem proteção.
- Cada thread tenta pegar pedidos e processá-los, mas o acesso simultâneo pode causar:
- Pedidos "pulados" ou repetidos
- Panics ou travamentos devido a corrupção de dados
- Ao final, o programa exibe o total de pedidos preparados, que geralmente não será igual ao número esperado.

### 4. Possível saída (não é determinística)

```text
Cozinheiro 0 preparando pedido=0
Cozinheiro 1 preparando pedido=0
Cozinheiro 2 preparando pedido=1
...
Existem pedidos que não foram preparados!!!
Tempo total: 10.23s segundos
```

## 👥 Equipe

- André Felipe
- Bernardo Ruas
- Heuller Ramos
- Gustavo dos Santos
