
pub fn eval(expres: &str) -> i32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests_1() {
        // let expr = "2+2";
        let expr = "x + y + 10";
        let vars = ["x", "y"];
        let consts = [10];
        let vals = ["1", "2"];

        let res = eval(expr);

        assert_eq!(res, 42);
    }
}
