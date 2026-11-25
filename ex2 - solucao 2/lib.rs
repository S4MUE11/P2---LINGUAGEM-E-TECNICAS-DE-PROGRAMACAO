
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
