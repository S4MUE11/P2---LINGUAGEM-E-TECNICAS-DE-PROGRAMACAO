
use ex2_solucao_2::{maximum_value, Item};

fn main() {
    let items = [
        Item { weight: 5, value: 10 },
        Item { weight: 4, value: 40 },
        Item { weight: 6, value: 30 },
        Item { weight: 3, value: 50 },
    ];

    let max_weight = 10;

    let result = maximum_value(max_weight, &items);

    println!("Valor máximo da mochila: {}", result);
}

