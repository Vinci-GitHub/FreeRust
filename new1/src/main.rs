enum Item {
    Inventory(String),
    // Noneには項目が無いことを表す
    None,
}

struct BagOfHolding {
    item: Item,
}
