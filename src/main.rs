use macros::Identify;

trait Identify {
    fn type_name(&self) -> &'static str;
    fn friendly_type_name(&self) -> &'static str;
}

#[derive(Identify)]
#[friendly_name("foo type :)")]
struct FooType;

#[derive(Identify)]
#[friendly_name("bar type :D")]
struct BarType;

fn main() {
    let f = FooType;
    let b = BarType;

    println!("f is a {}", f.type_name());
    println!("f is a {}", f.friendly_type_name());
    println!("b is a {}", b.type_name());
    println!("b is a {}", b.friendly_type_name());
}
