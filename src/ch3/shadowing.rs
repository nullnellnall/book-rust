fn main() {
    // シャドーイングを使わない例 --- (*1)
    {
        let mut v = 300; // vをミュータブルにする
        v = v + 5;
        println!("{}", v);
    }
    // シャドーイングを使う例 --- (*2)
    {
        let v = 300; // vはイミュータブルでOK
        let v = v + 5;
        println!("{}", v);
    }
}

