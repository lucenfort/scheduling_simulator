// src/main.rs

use colored::*; // para saída colorida no terminal
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

/// Estrutura que representa um processo.
#[derive(Clone)]
struct Process {
    pid: usize,          // Identificador do processo
    arrival_time: u32,   // Tempo de chegada
    burst_time: u32,     // Tempo de execução total (CPU burst)
    remaining_time: u32, // Tempo restante para a execução (usado em simulações preemptivas)
    priority: u32,       // Prioridade (quanto menor, mais alta a prioridade)
}

impl Process {
    /// Cria um novo processo com os valores informados.
    fn new(pid: usize, arrival: u32, burst: u32, priority: u32) -> Self {
        Process {
            pid,
            arrival_time: arrival,
            burst_time: burst,
            remaining_time: burst,
            priority,
        }
    }
}

/// Função auxiliar que aguarda o usuário pressionar Enter.
fn wait_for_enter() {
    let mut input = String::new();
    println!("{}", "Pressione Enter para continuar...".blue());
    io::stdin().read_line(&mut input).unwrap();
}

/// Função que retorna um vetor de processos de exemplo.
fn get_sample_processes() -> Vec<Process> {
    vec![
        Process::new(1, 0, 5, 2),
        Process::new(2, 1, 3, 1),
        Process::new(3, 2, 8, 3),
        Process::new(4, 3, 6, 2),
    ]
}

/// Simulação do algoritmo FCFS (First Come, First Served).
fn simulate_fcfs(mut processes: Vec<Process>) {
    println!("{}", "Simulação: FCFS".green().bold());
    // Ordena os processos pelo tempo de chegada.
    processes.sort_by_key(|p| p.arrival_time);
    let mut current_time = 0;
    for process in processes {
        if current_time < process.arrival_time {
            println!(
                "{}: Sistema ocioso por {} unidade(s).",
                format!("Tempo {}", current_time).yellow(),
                process.arrival_time - current_time
            );
            current_time = process.arrival_time;
        }
        println!(
            "{}: Processo {} inicia execução (Burst: {}).",
            format!("Tempo {}", current_time).yellow(),
            process.pid.to_string().cyan(),
            process.burst_time
        );
        // Simula a execução com uma pausa (500 ms por unidade de burst)
        thread::sleep(Duration::from_millis(process.burst_time as u64 * 500));
        current_time += process.burst_time;
        println!(
            "{}: Processo {} finalizado.",
            format!("Tempo {}", current_time).yellow(),
            process.pid.to_string().cyan()
        );
        wait_for_enter();
    }
    println!("{}", "Simulação FCFS concluída.".green().bold());
}

/// Simulação do algoritmo SJF (Shortest Job First) não-preemptivo.
fn simulate_sjf(processes: Vec<Process>) {
    // Reatribui o vetor para uma variável mutável local.
    let mut processes = processes;
    println!("{}", "Simulação: SJF".green().bold());
    let mut current_time = 0;
    let mut completed = vec![];
    let n = processes.len();
    while completed.len() < n {
        // Seleciona os processos que já chegaram e ainda não foram completados.
        let mut available: Vec<Process> = processes
            .iter()
            .filter(|p| p.arrival_time <= current_time && !completed.contains(&p.pid))
            .cloned()
            .collect();
        if available.is_empty() {
            println!(
                "{}: Sistema ocioso.",
                format!("Tempo {}", current_time).yellow()
            );
            current_time += 1;
            thread::sleep(Duration::from_millis(500));
            continue;
        }
        // Seleciona o processo com o menor tempo de burst.
        available.sort_by_key(|p| p.burst_time);
        let proc_to_run = available[0].clone();
        println!(
            "{}: Processo {} inicia execução (Burst: {}).",
            format!("Tempo {}", current_time).yellow(),
            proc_to_run.pid.to_string().cyan(),
            proc_to_run.burst_time
        );
        thread::sleep(Duration::from_millis(proc_to_run.burst_time as u64 * 500));
        current_time += proc_to_run.burst_time;
        println!(
            "{}: Processo {} finalizado.",
            format!("Tempo {}", current_time).yellow(),
            proc_to_run.pid.to_string().cyan()
        );
        completed.push(proc_to_run.pid);
        wait_for_enter();
    }
    println!("{}", "Simulação SJF concluída.".green().bold());
}

/// Simulação do algoritmo Round Robin.
fn simulate_round_robin(mut processes: Vec<Process>, quantum: u32) {
    println!("{}", "Simulação: Round Robin".green().bold());
    let mut current_time = 0;
    let mut queue: Vec<Process> = vec![];
    let n = processes.len();
    let mut completed: Vec<usize> = vec![];
    // Ordena os processos pelo tempo de chegada.
    processes.sort_by_key(|p| p.arrival_time);
    while completed.len() < n {
        // Adiciona à fila os processos que já chegaram e não estão na fila nem foram completados.
        for process in processes.iter_mut() {
            if process.arrival_time <= current_time
                && !queue.iter().any(|p| p.pid == process.pid)
                && !completed.contains(&process.pid)
            {
                queue.push(process.clone());
            }
        }
        if queue.is_empty() {
            println!(
                "{}: Sistema ocioso.",
                format!("Tempo {}", current_time).yellow()
            );
            current_time += 1;
            thread::sleep(Duration::from_millis(500));
            continue;
        }
        let mut process = queue.remove(0);
        let exec_time = if process.remaining_time > quantum {
            quantum
        } else {
            process.remaining_time
        };
        println!(
            "{}: Processo {} executa por {} unidade(s) (Restante: {}).",
            format!("Tempo {}", current_time).yellow(),
            process.pid.to_string().cyan(),
            exec_time,
            process.remaining_time
        );
        thread::sleep(Duration::from_millis(exec_time as u64 * 500));
        current_time += exec_time;
        process.remaining_time -= exec_time;
        if process.remaining_time == 0 {
            println!(
                "{}: Processo {} finalizado.",
                format!("Tempo {}", current_time).yellow(),
                process.pid.to_string().cyan()
            );
            completed.push(process.pid);
        } else {
            println!(
                "{}: Processo {} interrompido, restante: {}.",
                format!("Tempo {}", current_time).yellow(),
                process.pid.to_string().cyan(),
                process.remaining_time
            );
            // Reinsere o processo no final da fila.
            queue.push(process);
        }
        wait_for_enter();
    }
    println!("{}", "Simulação Round Robin concluída.".green().bold());
}

/// Simulação do algoritmo de Prioridade (não-preemptivo).
fn simulate_priority(processes: Vec<Process>) {
    println!("{}", "Simulação: Prioridade".green().bold());
    let mut current_time = 0;
    let mut completed: Vec<usize> = vec![];
    let n = processes.len();
    while completed.len() < n {
        // Seleciona os processos disponíveis que já chegaram e não foram completados.
        let mut available: Vec<Process> = processes
            .iter()
            .filter(|p| p.arrival_time <= current_time && !completed.contains(&p.pid))
            .cloned()
            .collect();
        if available.is_empty() {
            println!(
                "{}: Sistema ocioso.",
                format!("Tempo {}", current_time).yellow()
            );
            current_time += 1;
            thread::sleep(Duration::from_millis(500));
            continue;
        }
        // Seleciona o processo com maior prioridade (menor valor numérico).
        available.sort_by_key(|p| p.priority);
        let proc_to_run = available[0].clone();
        println!(
            "{}: Processo {} (Prioridade: {}) inicia execução (Burst: {}).",
            format!("Tempo {}", current_time).yellow(),
            proc_to_run.pid.to_string().cyan(),
            proc_to_run.priority.to_string().magenta(),
            proc_to_run.burst_time
        );
        thread::sleep(Duration::from_millis(proc_to_run.burst_time as u64 * 500));
        current_time += proc_to_run.burst_time;
        println!(
            "{}: Processo {} finalizado.",
            format!("Tempo {}", current_time).yellow(),
            proc_to_run.pid.to_string().cyan()
        );
        completed.push(proc_to_run.pid);
        wait_for_enter();
    }
    println!("{}", "Simulação Prioridade concluída.".green().bold());
}

/// Simulação do algoritmo Multinível com Feedback (MLFQ).
/// Nesta simulação, utilizamos duas filas:
/// - Fila de alta prioridade com quantum quantum_high.
/// - Fila de baixa prioridade com quantum quantum_low.
fn simulate_mlfq(mut processes: Vec<Process>, quantum_high: u32, quantum_low: u32) {
    println!(
        "{}",
        "Simulação: Escalonamento Multinível com Feedback"
            .green()
            .bold()
    );
    let mut current_time = 0;
    let mut high_queue: Vec<Process> = vec![];
    let mut low_queue: Vec<Process> = vec![];
    let n = processes.len();
    let mut completed: Vec<usize> = vec![];
    // Ordena os processos pelo tempo de chegada.
    processes.sort_by_key(|p| p.arrival_time);
    while completed.len() < n {
        // Adiciona processos que chegaram à fila de alta prioridade.
        for process in processes.iter_mut() {
            if process.arrival_time <= current_time
                && !high_queue.iter().any(|p| p.pid == process.pid)
                && !low_queue.iter().any(|p| p.pid == process.pid)
                && !completed.contains(&process.pid)
            {
                high_queue.push(process.clone());
            }
        }
        if high_queue.is_empty() && low_queue.is_empty() {
            println!(
                "{}: Sistema ocioso.",
                format!("Tempo {}", current_time).yellow()
            );
            current_time += 1;
            thread::sleep(Duration::from_millis(500));
            continue;
        }
        // Se houver processos na fila de alta prioridade, eles são executados com quantum_high.
        if !high_queue.is_empty() {
            let mut process = high_queue.remove(0);
            let exec_time = if process.remaining_time > quantum_high {
                quantum_high
            } else {
                process.remaining_time
            };
            println!(
                "{}: [Alta Prioridade] Processo {} executa por {} unidade(s) (Restante: {}).",
                format!("Tempo {}", current_time).yellow(),
                process.pid.to_string().cyan(),
                exec_time,
                process.remaining_time
            );
            thread::sleep(Duration::from_millis(exec_time as u64 * 500));
            current_time += exec_time;
            process.remaining_time -= exec_time;
            if process.remaining_time == 0 {
                println!(
                    "{}: Processo {} finalizado.",
                    format!("Tempo {}", current_time).yellow(),
                    process.pid.to_string().cyan()
                );
                completed.push(process.pid);
            } else {
                println!(
                    "{}: Processo {} interrompido, movendo para fila de baixa prioridade (Restante: {}).",
                    format!("Tempo {}", current_time).yellow(),
                    process.pid.to_string().cyan(),
                    process.remaining_time
                );
                low_queue.push(process);
            }
        }
        // Caso a fila de alta esteja vazia, executa a fila de baixa prioridade com quantum_low.
        else if !low_queue.is_empty() {
            let mut process = low_queue.remove(0);
            let exec_time = if process.remaining_time > quantum_low {
                quantum_low
            } else {
                process.remaining_time
            };
            println!(
                "{}: [Baixa Prioridade] Processo {} executa por {} unidade(s) (Restante: {}).",
                format!("Tempo {}", current_time).yellow(),
                process.pid.to_string().cyan(),
                exec_time,
                process.remaining_time
            );
            thread::sleep(Duration::from_millis(exec_time as u64 * 500));
            current_time += exec_time;
            process.remaining_time -= exec_time;
            if process.remaining_time == 0 {
                println!(
                    "{}: Processo {} finalizado.",
                    format!("Tempo {}", current_time).yellow(),
                    process.pid.to_string().cyan()
                );
                completed.push(process.pid);
            } else {
                println!(
                    "{}: Processo {} interrompido, reinserido na fila de baixa prioridade (Restante: {}).",
                    format!("Tempo {}", current_time).yellow(),
                    process.pid.to_string().cyan(),
                    process.remaining_time
                );
                low_queue.push(process);
            }
        }
        wait_for_enter();
    }
    println!("{}", "Simulação MLFQ concluída.".green().bold());
}

/// Função principal: exibe um menu interativo e permite que o usuário escolha o método de escalonamento para simulação.
fn main() {
    loop {
        println!(
            "{}",
            "==========================================".blue().bold()
        );
        println!(
            "{}",
            "Simulador de Escalonamento de Processos".blue().bold()
        );
        println!("{}", "Escolha o método de escalonamento:".blue());
        println!("1. FCFS");
        println!("2. SJF");
        println!("3. Round Robin");
        println!("4. Prioridade");
        println!("5. Multinível com Feedback");
        println!("6. Sair");
        print!("{}", "Digite sua escolha: ".blue());
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Falha ao ler entrada.");
        let choice = choice.trim();
        match choice {
            "1" => {
                let processes = get_sample_processes();
                simulate_fcfs(processes);
            }
            "2" => {
                let processes = get_sample_processes();
                simulate_sjf(processes);
            }
            "3" => {
                let processes = get_sample_processes();
                // Para Round Robin, definimos quantum = 2
                simulate_round_robin(processes, 2);
            }
            "4" => {
                let processes = get_sample_processes();
                simulate_priority(processes);
            }
            "5" => {
                let processes = get_sample_processes();
                // Para MLFQ, definimos quantum de alta prioridade = 4 e de baixa = 2
                simulate_mlfq(processes, 4, 2);
            }
            "6" => {
                println!("{}", "Saindo do simulador. Até logo!".green());
                break;
            }
            _ => {
                println!("{}", "Opção inválida, tente novamente.".red());
            }
        }
    }
}
