use std::{
    sync::{Arc, Mutex},          // Arc = permite múltiplas threads; Mutex = exclusão mútua
    thread,
    time::{Duration, Instant},
};

use rand::Rng;

fn main() {
    let num_pedidos = 100;
    let num_cozinheiros = 5;

    // vetores protegidos por Mutex e compartilhados por Arc
    let fila_de_pedidos = Arc::new(Mutex::new((0..num_pedidos).collect::<Vec<_>>()));
    let pedidos_prontos = Arc::new(Mutex::new(Vec::new()));

    let mut cozinheiros = vec![]; // lista de threads (cozinheiros)

    let inicio = Instant::now();

    // cria threads (cozinheiros)
    for i in 0..num_cozinheiros {

        // Clona referência segura da fila e dos pedidos prontos, para cada thread
        let fila = Arc::clone(&fila_de_pedidos);
        let prontos = Arc::clone(&pedidos_prontos);

        let thread = thread::spawn(move || {
            println!("Cozinheiro {} pronto para receber pedidos!", i);

            let mut rng = rand::thread_rng();

            loop {
                // tenta obter um pedido da fila
                let pedido = {
                    let mut fila = fila.lock().unwrap(); // bloqueia o mutex para acesso exclusivo da thread

                    if fila.is_empty() {
                        break; // fila vazia: cozinheiro encerra
                    }

                    fila.remove(0) // retira o primeiro pedido da fila e o armazena na variável "pedido"
                }; 
                
                // Mutex liberado aqui (ao sair do bloco)

                println!("Cozinheiro {} preparando pedido={}", i, pedido);

                let tempo_ms = rng.gen_range(1000..=5000); // n° aleatório de 1 a 5 segundos
                thread::sleep(Duration::from_millis(tempo_ms)); // simula preparo

                println!(
                    "Cozinheiro {} terminou pedido={} (tempo de preparo: {}ms)",
                    i, pedido, tempo_ms
                );

                // adiciona o pedido finalizado na lista de prontos
                let mut prontos = prontos.lock().unwrap();
                prontos.push(pedido);
            }
            // Mutex liberado automaticamente
        });

        cozinheiros.push(thread);
    }

    // aguarda todas as threads terminarem
    for c in cozinheiros {
        c.join().unwrap();
    }

    let fim = Instant::now();
    let tempo_total = fim.duration_since(inicio);

    println!("Tempo total: {:.2?} segundos", tempo_total);

    // confere se todos os pedidos foram preparados
    let qtdd_prontos = pedidos_prontos.lock().unwrap().len();
    if num_pedidos == qtdd_prontos {
        println!("Todos os {} pedidos foram preparados!", qtdd_prontos);
    } else {
        println!("Existem pedidos que não foram preparados!!!");
    }

}
