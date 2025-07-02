use std::{
    thread,
    time::{Duration, Instant},
};
use rand::Rng;

// variáveis globais mutáveis sem nenhuma proteção de sincronização
static mut FILA_DE_PEDIDOS: Vec<u32> = Vec::new();
static mut PEDIDOS_PRONTOS: Vec<u32> = Vec::new();

fn main() {
    let num_pedidos = 100;
    let num_cozinheiros = 5;

    unsafe {
        // variável estática e mutável necessita do bloco unsafe
        FILA_DE_PEDIDOS = (0..num_pedidos).collect();
    }

    let mut cozinheiros = vec![];

    let inicio = Instant::now();

    // cria as threads que simulam os cozinheiros
    for i in 0..num_cozinheiros {

        let thread = thread::spawn(move || {
            let mut rng = rand::thread_rng();

            loop {
                // tentativa de pegar um pedido da fila
                let pedido_opt = unsafe {
                    // condição de corrida: threads podem entrar nesse bloco ao mesmo tempo (corrupção da fila)
                    if FILA_DE_PEDIDOS.is_empty() {
                        None
                    } else {
                        // remove o primeiro item da fila (várias threads podem tentar remover ao mesmo tempo)
                        Some(FILA_DE_PEDIDOS.remove(0))
                    }
                };

                // se não havia mais pedidos, a thread termina
                let pedido = match pedido_opt {
                    Some(p) => p,
                    None => break,
                };

                println!("Cozinheiro {} preparando pedido={}", i, pedido);

                // tempo aleatório de preparo entre 1 e 5 segundos
                let tempo_ms = rng.gen_range(1000..=5000);
                thread::sleep(Duration::from_millis(tempo_ms));

                println!(
                    "Cozinheiro {} terminou pedido={} (tempo de preparo: {}ms)",
                    i, pedido, tempo_ms
                );

                unsafe {
                    // adiciona o pedido pronto na lista global sem proteção
                    // isso é outra condição de corrida: múltiplas threads podem escrever ao mesmo tempo
                    PEDIDOS_PRONTOS.push(pedido);
                }
            }
        });

        // armazena a thread para poder dar join depois
        cozinheiros.push(thread);
    }

    // espera todas as threads terminarem
    for c in cozinheiros {
        c.join().unwrap();
    }

    let fim = Instant::now();
    let tempo_total = fim.duration_since(inicio);

    println!("Tempo total: {:.2?} segundos", tempo_total);

    unsafe {
        // verifica quantos pedidos foram preparados (pode haver pedidos perdidos ou duplicados)
        let qtdd_prontos = PEDIDOS_PRONTOS.len();
        if qtdd_prontos == num_pedidos {
            println!("Todos os {} pedidos foram preparados!", qtdd_prontos);
        } else {
            println!("Existem pedidos que não foram preparados!!!");
        }
    }
}
