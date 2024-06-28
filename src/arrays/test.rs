pub fn tester() {
    let mut v = vec![1, 3, 7];

    let v2 = &v;

    v[1] = 4;

    println!("{:?}", v2);
}
