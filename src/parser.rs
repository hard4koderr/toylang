include!(concat!(env!("OUT_DIR"), "/grammar.rs"));

#[cfg(test)]
mod tests {
    use parser::*;

    #[test]
    fn assign_int() {
        assert!(statement("let number = 5;").is_ok());
        assert!(statement("let number=5;").is_ok());
        assert!(statement("let number =5;").is_ok());
        assert!(statement("let number= 5;").is_ok());
    }

    #[test]
    fn eval_int() {
        assert_eq!(
            expression("1").unwrap(),
            Expr::Literal(Value::Num(1.0))
        );
    }

    #[test]
    fn assign_float() {
        assert!(statement("let number = 13.2;").is_ok());
    }

    #[test]
    fn eval_float() {
        assert_eq!(
            expression("123.456").unwrap(),
            Expr::Literal(Value::Num(123.456))
        );
    }

    #[test]
    fn eval_arrays() {
        assert!(expression("foo[0]").is_ok());
        assert!(expression("foo[bar]").is_ok());

        assert!(expression("[1,2,3][0]").is_ok());
        assert!(expression("[[1,2], [foo, bar]][0][1]").is_ok());
        assert!(expression("foo[0][1]").is_ok());

        assert!(expression("[bar][0]").is_ok());
        assert!(expression("[bar][2 + baz]").is_ok());
    }

    #[test]
    fn eval_bools() {
        assert_eq!(
            expression("true").unwrap(),
            Expr::Literal(Value::Boolean(true))
        );

        assert_eq!(
            expression("false").unwrap(),
            Expr::Literal(Value::Boolean(false))
        );
    }

    #[test]
    fn typecast() {
        assert!(expression("5 as string").is_ok());
        assert!(expression("5 + 5 as string").is_ok());
        assert!(expression("5 as string + 5").is_ok());
        assert!(expression("ARGV[1] as num").is_ok());
        assert!(expression("![true][0] as string").is_ok());
    }

    #[test]
    fn declare_function() {
        assert!(statement("func print_one() { println 1; }").is_ok());
        assert!(statement("func print_num(n: num) { println n; }").is_ok());
        assert!(statement(r#"func print_num_and_str(n: num, s: string) { println n, " ", s; }"#).is_ok());
    }

    #[test]
    fn call_function() {
        assert!(expression("some_func(1)").is_ok());
        assert!(expression("some_func(1, 2)").is_ok());
        assert!(expression("3 + square(5) * 3").is_ok());
        assert!(ast("func foo() { }\nprintln foo();").is_ok());
    }

    #[test]
    fn return_statement() {
        assert!(statement("return 1;").is_ok());
    }

    #[test]
    fn get_length() {
        assert!(expression(r#"length("test" as array)"#).is_ok());
    }
}
