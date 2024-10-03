#[derive(Debug, Clone)]
struct Data {
    value: i32,
}
impl Data {
    fn adjuster<F: Fn(&mut Data) -> &mut Data>(&mut self, f: F) -> &mut Data {
        f(self)
    }
}

fn add(data: &mut Data) -> &mut Data {
    data.value += 1;
    data
}

fn mult(data: &mut Data) -> &mut Data {
    data.value *= 2;
    data
}

fn main() {
    let mut d = Data { value: 10 };
    d.adjuster(|e| add(e))
        .adjuster(|e| add(e))
        .adjuster(|e| add(e))
        .adjuster(|e| mult(e));

    println!("{:#?}", d);
}
