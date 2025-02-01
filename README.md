# Fundamentos Teóricos e Algoritmos de Gerenciamento de Processos 🚀

Este projeto é um simulador interativo escrito em Rust que demonstra, de forma educativa e prática, os principais algoritmos de escalonamento de processos. O simulador apresenta métodos tradicionais e modernos, permitindo acompanhar passo a passo a execução de cada algoritmo com saída colorida e interativa no terminal.

## Índice
1. [Fundamentos Teóricos](#1-fundamentos-teoricos)
    - [Processo 🖥️](#11-processo-️)
    - [Threads 🔀](#12-threads-)
2. [Algoritmos de Gerenciamento de Processos](#2-algoritmos-de-gerenciamento-de-processos)
    - [Sistemas Operacionais Tradicionais](#21-sistemas-operacionais-tradicionais)
        - [FCFS (First Come, First Served) ⏱️](#211-fcfs-first-come-first-served-️)
        - [SJF (Shortest Job First) ⏳](#212-sjf-shortest-job-first-)
        - [Round Robin (RR) 🔄](#213-round-robin-rr-)
        - [Algoritmo de Prioridade ⭐](#214-algoritmo-de-prioridade-)
        - [Escalonamento Multinível com Feedback 🔀💡](#215-escalonamento-multinivel-com-feedback-)
3. [Como Executar o Projeto](#3-como-executar-o-projeto)
4. [Conclusão](#4-conclusao)

---

## 1. Fundamentos Teóricos <a name="1-fundamentos-teoricos"></a>

### 1.1. Processo 🖥️ <a name="11-processo-️"></a>
Um **processo** é uma instância de um programa em execução, composto por:
- **Código executável**
- **Dados e variáveis**
- **Contexto de execução** (registradores, pilha, espaço de memória)

Cada processo possui seu próprio espaço de endereço, garantindo isolamento e segurança, o que é fundamental para a execução simultânea de múltiplas tarefas.

### 1.2. Threads 🔀 <a name="12-threads-"></a>
Uma **thread** é a menor unidade de execução dentro de um processo. Diferentemente dos processos, as threads compartilham:
- **Espaço de memória**
- **Recursos do processo**

Isso permite a execução concorrente, especialmente em sistemas com múltiplos núcleos, mas requer mecanismos de sincronização para evitar condições de corrida.

---

## 2. Algoritmos de Gerenciamento de Processos <a name="2-algoritmos-de-gerenciamento-de-processos"></a>

### 2.1. Sistemas Operacionais Tradicionais <a name="21-sistemas-operacionais-tradicionais"></a>

#### 2.1.1. FCFS (First Come, First Served) ⏱️ <a name="211-fcfs-first-come-first-served-️"></a>
- **Descrição:**  
  Atende os processos na ordem de chegada, sem preempção.
- **Complexidade Algorítmica:**  
  - Inserção e remoção em fila: **O(n)**
- **Casos de Uso:**  
  - Sistemas batch e cenários onde a ordem de chegada é determinante.
- **Observações:**  
  - Método simples, mas pode causar alta latência para processos que chegam posteriormente.

**Fluxograma FCFS:**
```mermaid
flowchart TD
    A[Início]
    B[Receber Processo]
    C[Inserir no Final da Fila]
    D[Processar Primeiro da Fila]
    E[Remover Processo]
    F[Fila Vazia?]
    G[Fim]

    A --> B
    B --> C
    C --> D
    D --> E
    E --> F
    F -- Não --> D
    F -- Sim --> G
```

---

#### 2.1.2. SJF (Shortest Job First) ⏳ <a name="212-sjf-shortest-job-first-"></a>
- **Descrição:**  
  Prioriza a execução dos processos com o menor tempo de CPU estimado, reduzindo o tempo médio de espera.
- **Complexidade Algorítmica:**  
  - Utilização de estruturas ordenadas: **O(n log n)**
- **Casos de Uso:**  
  - Sistemas com tarefas de duração previsível.
- **Observações:**  
  - Pode levar à inanição de processos com tempos maiores e depende da precisão da estimativa.

**Fluxograma SJF:**
```mermaid
flowchart TD
    A[Início]
    B[Receber Processo com Tempo Estimado]
    C[Inserir na Fila Ordenada]
    D[Selecionar Processo com Menor Tempo]
    E[Processar Processo]
    F[Remover Processo da Fila]
    G[Fila Vazia?]
    H[Fim]

    A --> B
    B --> C
    C --> D
    D --> E
    E --> F
    F --> G
    G -- Não --> D
    G -- Sim --> H
```

---

#### 2.1.3. Round Robin (RR) 🔄 <a name="213-round-robin-rr-"></a>
- **Descrição:**  
  Cada processo recebe uma fatia de tempo (time slice) de forma cíclica; após a expiração, o processo é interrompido e reinserido na fila.
- **Complexidade Algorítmica:**  
  - Preempção: **O(1)** por ciclo; ciclo completo: **O(n)**
- **Casos de Uso:**  
  - Sistemas interativos onde a responsividade é essencial.
- **Observações:**  
  - A escolha do time slice é crucial para equilibrar a eficiência e a resposta do sistema.

**Fluxograma Round Robin:**
```mermaid
flowchart TD
    A[Início]
    B[Receber Processo]
    C[Alocar Time Slice]
    D[Executar Processo por Time Slice]
    E[Time Slice Expirado?]
    F[Processo Concluído?]
    G[Remover Processo]
    H[Reinserir na Fila]
    I[Fila Vazia?]
    J[Fim]

    A --> B
    B --> C
    C --> D
    D --> E
    E -- Sim --> F
    F -- Sim --> G
    F -- Não --> H
    H --> I
    I -- Não --> B
    I -- Sim --> J
```
---

#### 2.1.4. Algoritmo de Prioridade ⭐ <a name="214-algoritmo-de-prioridade-"></a>
- **Descrição:**  
  Processos são escalonados com base em prioridades atribuídas. Pode ser preemptivo ou não-preemptivo, utilizando técnicas de envelhecimento (aging) para evitar inanição.
- **Complexidade Algorítmica:**  
  - Inserção em heap: **O(log n)**
- **Casos de Uso:**  
  - Ambientes onde certos processos são críticos (ex.: aplicações financeiras, controle industrial).
- **Observações:**  
  - Requer ajustes contínuos para evitar inanição de processos de baixa prioridade.

**Fluxograma do Algoritmo de Prioridade:**
```mermaid
flowchart TD
    A[Início]
    B[Receber Processo com Prioridade]
    C[Inserir na Estrutura de Priorização]
    D[Selecionar Processo com Maior Prioridade]
    E[Processar Processo]
    F[Processo Concluído?]
    G[Remover Processo]
    H[Aplicar Aging se Necessário]
    I[Fila Vazia?]
    J[Fim]

    A --> B
    B --> C
    C --> D
    D --> E
    E --> F
    F -- Sim --> G
    F -- Não --> H
    H --> I
    I -- Não --> D
    I -- Sim --> J
```

---

#### 2.1.5. Escalonamento Multinível com Feedback 🔀💡 <a name="215-escalonamento-multinivel-com-feedback-"></a>
- **Descrição:**  
  Utiliza múltiplas filas com políticas de escalonamento distintas; processos podem ser realocados entre filas conforme seu comportamento (uso de CPU).
- **Complexidade Algorítmica:**  
  - Realocação dinâmica: **O(n log n)**
- **Casos de Uso:**  
  - Sistemas operacionais modernos que gerenciam cargas variadas.
- **Observações:**  
  - Oferece flexibilidade e adaptação dinâmica, embora a implementação seja complexa.

**Fluxograma Escalonamento Multinível com Feedback:**
```mermaid
flowchart TD
    A[Início]
    B[Receber Processo]
    C[Inserir na Fila Inicial]
    D[Executar Processo]
    E[Verificar Uso de CPU]
    F[Uso Alto?]
    G[Realocar para Fila de Alta Prioridade]
    H[Uso Baixo?]
    I[Realocar para Fila Inferior]
    J[Processo Concluído?]
    K[Remover Processo]
    L[Retornar para a Fila Atualizada]
    M[Fila Vazia?]
    N[Fim]

    A --> B
    B --> C
    C --> D
    D --> E
    E -- Sim (Alto) --> F
    F -- Sim --> G
    E -- Não (Baixo) --> I
    G --> J
    I --> J
    J -- Sim --> K
    J -- Não --> L
    L --> M
    M -- Não --> D
    M -- Sim --> N
```

---


## 3. Como Executar o Projeto <a name="3-como-executar-o-projeto"></a>

Este simulador foi implementado em Rust e utiliza a biblioteca [colored](https://crates.io/crates/colored) para uma saída interativa e colorida no terminal. 

### Passos:
1. **Clone o repositório:**
   ```bash
   git clone https://github.com/lucenfort/scheduling_simulator.git
   cd scheduling_simulator
   ```
2. **Compile e execute o projeto:**
   ```bash
   cargo run
   ```
3. **Interaja com o menu:**
   O simulador exibirá um menu interativo no terminal.

---

## 4. Conclusão <a name="4-conclusao"></a>

A evolução dos algoritmos de gerenciamento de processos apresenta métodos que variam em complexidade e aplicabilidade conforme os requisitos do sistema. 

Este projeto demonstra, de forma interativa e educativa, os fundamentos teóricos e os algoritmos de escalonamento, proporcionando uma compreensão prática do funcionamento de cada método.

---

## Licença

Este projeto está licenciado sob a [MIT License](LICENSE).
