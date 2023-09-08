#[derive(Debug)]
pub struct Spore {}

pub const NAME: &'static str = "Spore";

pub fn fn1() {
    println!("calling fn1 in {}", NAME);
    fn3();
}

// crate public
pub(crate) fn fn2() {
    println!("calling pub crate fn2...");
    fn3();
}

fn fn3() -> bool {
    println!("only visiable in spore module");
    true
}

#[cfg(test)]
mod tests {
    use super::fn3;

    fn test_fn3() {
        assert_eq!(fn3(), true);
    }
}
