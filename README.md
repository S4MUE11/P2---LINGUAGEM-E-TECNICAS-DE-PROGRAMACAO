# Avaliação Prática 2 (P2): Exercism em Rust

Resolução de dois exercícios da trilha de Rust da plataforma Exercism, cada um com duas abordagens algoritmicamente distintas, totalizando 4 soluções explicadas e testadas.

## Informações acadêmicas

* **Instituição:** Faculdade AEMS
* **Curso:** Engenharia da Computação, 6º período
* **Disciplina:** Linguagens e Técnicas de Programação / Técnicas de Programação
* **Professor:** Bruno Gabriel
* **Formato:** Em dupla
* **Data limite de entrega:** 24/11/2026
* **Integrantes:** Samuel Dias Medeiros e Robert William

## Exercícios escolhidos

| Nível | Exercício | Abordagem 1 | Abordagem 2 |
| :--- | :--- | :--- | :--- |
| Fácil | Kindergarten Garden | Acesso direto por índices (laços imperativos) | Programação funcional com iteradores |
| Médio | Knapsack | Programação dinâmica | Força bruta (recursão) |

## Estrutura do repositório

```text
.
├── README.md
├── facil-kindergarten-garden/
│   ├── solucao-01-indices/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   └── solucao-02-iteradores/
│       ├── Cargo.toml
│       └── src/lib.rs
└── medio-knapsack/
    ├── solucao-01-programacao-dinamica/
    │   ├── Cargo.toml
    │   └── src/lib.rs
    └── solucao-02-forca-bruta/
        ├── Cargo.toml
        └── src/lib.rs
```

Cada solução é um projeto Cargo independente, com a suíte de testes oficial do Exercism.

## Como executar os testes

Pré-requisito: Rust e Cargo instalados.

```bash
# entre na pasta da solução desejada, por exemplo:
cd medio-knapsack/solucao-01-programacao-dinamica

# executa todos os testes (o Exercism marca a maioria com #[ignore])
cargo test -- --include-ignored
```

---

## 1. Exercício fácil: Kindergarten Garden

### Enunciado resumido

Uma turma de jardim de infância tem 12 crianças (Alice, Bob, Charlie, David, Eve, Fred, Ginny, Harriet, Ileana, Joseph, Kincaid e Larry). Cada uma planta 4 copos, 2 em cada uma das duas linhas do diagrama, distribuídos em ordem alfabética. As plantas são representadas por letras:

| Letra | Planta |
| :---: | :--- |
| G | grass |
| C | clover |
| R | radishes |
| V | violets |

Dado o diagrama e o nome de uma criança, o programa retorna as plantas dela, lendo primeiro a linha mais próxima da janela.

```text
VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV
```

* **Alice:** violets, radishes, violets, radishes
* **Bob:** clover, grass, clover, clover

### Ideia central

A posição da criança na lista alfabética (0 a 11) multiplicada por 2 indica a coluna inicial dos seus copos. Em cada linha do diagrama, pegam-se 2 letras a partir dessa coluna e cada letra é traduzida para o nome da planta.

**Exemplo (Bob):** posição 1, coluna inicial 1 x 2 = 2. Linha 1: `CG` (clover, grass). Linha 2: `CC` (clover, clover).

### Solução 01: acesso direto por índices (laços imperativos)

```rust
pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let students = [
        "Alice", "Bob", "Charlie", "David",
        "Eve", "Fred", "Ginny", "Harriet",
        "Ileana", "Joseph", "Kincaid", "Larry",
    ];

    let student_index = students.iter()
        .position(|&name| name == student)
        .unwrap();

    let start = student_index * 2;

    let mut result = Vec::new();

    for row in diagram.lines() {
        for plant in row.chars().skip(start).take(2) {
            let name = match plant {
                'G' => "grass",
                'C' => "clover",
                'R' => "radishes",
                'V' => "violets",
                _ => unreachable!(),
            };

            result.push(name);
        }
    }

    result
}
```

#### Como funciona

* `students` guarda os nomes em ordem alfabética.
* `.position(...)` encontra o índice da criança; `.unwrap()` extrai o valor (falha se o nome não existir).
* `start = student_index * 2` calcula a coluna inicial.
* O laço externo percorre as linhas (`diagram.lines()`); o interno pega 2 caracteres com `.skip(start).take(2)`.
* O `match` traduz cada letra e `result.push(name)` acumula em um `Vec` mutável.

### Solução 02: programação funcional com iteradores

```rust
pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let students = [
        "Alice", "Bob", "Charlie", "David",
        "Eve", "Fred", "Ginny", "Harriet",
        "Ileana", "Joseph", "Kincaid", "Larry",
    ];

    let start = students.iter()
        .position(|&name| name == student)
        .unwrap() * 2;

    diagram.lines()
        .flat_map(|row| {
            row.chars()
                .skip(start)
                .take(2)
                .map(|plant| match plant {
                    'G' => "grass",
                    'C' => "clover",
                    'R' => "radishes",
                    'V' => "violets",
                    _ => unreachable!(),
                })
        })
        .collect()
}
```

#### Como funciona

* O cálculo de `start` é encadeado em uma única expressão.
* `flat_map` transforma cada linha em um iterador de plantas e "achata" tudo em uma sequência única.
* `.map(...)` traduz cada letra para o nome da planta.
* `.collect()` monta o `Vec` final, sem variável mutável.

### Comparação das abordagens

| Aspecto | Solução 01 (imperativa) | Solução 02 (funcional) |
| :--- | :--- | :--- |
| Estilo | Laços `for` aninhados | Cadeia de iteradores (`flat_map`, `map`, `collect`) |
| Mutabilidade | Usa `let mut result` e `push` | Sem estado mutável |
| Construção do resultado | Passo a passo, explícita | Declarativa, em uma expressão |
| Leitura | Mais direta para quem vem de outras linguagens | Mais concisa e idiomática em Rust |
| Complexidade | Proporcional ao nº de linhas do diagrama (no máximo 2 linhas e 4 plantas lidas por criança) | Mesma |

---

## 2. Exercício médio: Knapsack

### Enunciado resumido

Lhakpa, uma sherpa, tem uma lista de itens, cada um com peso e valor (todos estritamente positivos), e uma mochila com peso máximo. Ela só pode levar uma unidade de cada item. O objetivo é escolher os itens que maximizam o valor total sem ultrapassar a capacidade.

**Exemplo:** capacidade 10 e itens (peso, valor) = (5,10), (4,40), (6,30), (4,50). A melhor escolha é o 2º e o 4º itens, com valor total 90.

Este é o clássico problema da mochila 0/1.

### Solução 01: programação dinâmica

```rust
#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let capacity = max_weight as usize;
    let mut dp = vec![0; capacity + 1];

    for item in items {
        let weight = item.weight as usize;

        if weight <= capacity {
            for current_weight in (weight..=capacity).rev() {
                dp[current_weight] = dp[current_weight]
                    .max(dp[current_weight - weight] + item.value);
            }
        }
    }

    dp[capacity]
}
```

#### Como funciona

* `dp[w]` guarda o maior valor possível com capacidade `w`. Começa tudo em 0.
* Para cada item, o laço interno percorre as capacidades de `capacity` até `weight`, de trás para frente (`.rev()`).
* Em cada capacidade, escolhe-se o maior entre não levar o item (`dp[w]`) e levar o item (`dp[w - weight] + value`).
* A resposta é `dp[capacity]`.

**Por que o laço é invertido?** Indo de trás para frente, `dp[w - weight]` ainda contém o valor da rodada anterior, sem o item atual. Assim, o mesmo item não é usado duas vezes. Em ordem crescente, o código resolveria a mochila ilimitada.

#### Execução do exemplo (vetor `dp`, posições 0 a 10, após cada item)

| Após o item | `dp[0..10]` |
| :--- | :--- |
| início | 0 0 0 0 0 0 0 0 0 0 0 |
| (5, 10) | 0 0 0 0 0 10 10 10 10 10 10 |
| (4, 40) | 0 0 0 0 40 40 40 40 40 50 50 |
| (6, 30) | 0 0 0 0 40 40 40 40 40 50 70 |
| (4, 50) | 0 0 0 0 50 50 50 50 90 90 90 |

**Complexidade:** tempo O(n x C) e espaço O(C), onde n é o número de itens e C a capacidade.

### Solução 02: força bruta (recursão)

```rust
#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    fn calcular(items: &[Item], index: usize, remaining: u32) -> u32 {
        if index == items.len() {
            return 0;
        }

        let sem_item = calcular(items, index + 1, remaining);

        if items[index].weight > remaining {
            return sem_item;
        }

        let com_item = items[index].value
            + calcular(
                items,
                index + 1,
                remaining - items[index].weight,
            );

        sem_item.max(com_item)
    }

    calcular(items, 0, max_weight)
}
```

#### Como funciona

* A função interna `calcular` recebe a lista de itens, o índice do item em análise e o peso restante na mochila.
* **Caso base:** se todos os itens foram analisados (`index == items.len()`), o valor é 0.
* Para cada item há duas decisões: não levar (`sem_item`) ou levar (`com_item`, somando o valor do item à melhor escolha com o espaço que sobra).
* Se o item é mais pesado que o espaço restante, só a opção "não levar" é possível.
* Retorna-se o maior valor entre as duas opções.

A função explora todas as combinações possíveis de itens (árvore de decisões binária). Por usar `remaining - items[index].weight` somente depois de verificar que o item cabe, não há subtração com overflow em `u32`.

**Complexidade:** tempo O(2^n) e espaço O(n) (profundidade da recursão). Para listas grandes, é inviável.

### Comparação das abordagens

| Aspecto | Solução 01 (programação dinâmica) | Solução 02 (força bruta) |
| :--- | :--- | :--- |
| Estratégia | Tabela que guarda o melhor valor por capacidade | Recursão que testa todas as combinações |
| Estrutura de dados | `Vec` (`dp`) | Pilha de chamadas recursivas |
| Tempo | O(n x C) | O(2^n) |
| Espaço | O(C) | O(n) |
| Reaproveita subproblemas | Sim | Não (recalcula os mesmos casos) |
| Vantagem | Eficiente para entradas grandes | Simples de entender e de provar correta |

---

## Recursos idiomáticos de Rust utilizados

* **Empréstimo (borrowing):** `&str` e `&[Item]` são recebidos por referência, sem tomar posse dos dados.
* **Iteradores:** `iter`, `position`, `chars`, `skip`, `take`, `lines`, `flat_map`, `map` e `collect`.
* **Pattern matching:** `match` para traduzir letras em plantas.
* **Imutabilidade por padrão:** `mut` só onde necessário (`result` e `dp`).
* **Funções aninhadas:** `calcular` fica encapsulada dentro de `maximum_value`.
* **Derivação de traits:** `#[derive(Debug)]` na struct `Item`.
* **Conversões explícitas de tipo:** `as usize` para indexar vetores.

## Vídeo de apresentação

**Link do vídeo:** (adicionar o link do Google Drive ou YouTube não listado)

## Licença

Projeto acadêmico, desenvolvido para a disciplina de Linguagens e Técnicas de Programação da Faculdade AEMS. Os enunciados dos exercícios pertencem ao Exercism.
