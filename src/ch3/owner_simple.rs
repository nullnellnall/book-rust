fn main() {
    let g1 = String::from("穏やかな心は体に良い"); // --- (*1)
    let g2 = g1; // 所有権をg2に移動 --- (*2)
    println!("{}", g2); // --- (*3)
}

