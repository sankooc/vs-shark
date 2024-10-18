#[cfg(test)]
mod unit {
    use core::common::filter::{evaluate_expression, Parser};
    use std::collections::HashMap;

    #[test] 
    fn test_filter_express() {
        {
            let input = "(tcp.ip == 123.32.2.1)";
            let mut parser = Parser::new(input);
            if let Err(_) = parser.parse() {
                assert!(false)
            }
        }
        {
            let input = "tcp.ip==123.32.2.1&&cpp=1";
            let mut parser = Parser::new(input);
            match parser.parse() {
                Ok(expr) => {
                    println!("{:#?}", expr);
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        {
            let input = "tcp.ip ==123.32.2.1 && (act.count >= 1 || ppc.c < 12)";
            let mut parser = Parser::new(input);
            match parser.parse() {
                Ok(expr) => {
                    // println!("{:#?}", expr);
                }
                Err(e) => println!("Error: {}", e),
            }
        }

        let mut data: HashMap<String, String> = HashMap::new();
        data.insert("tcp.ip".to_string(), "123.32.2.1".to_string());
        data.insert("act".to_string(), "out".to_string());

        let input = "tcp.ip == 123.32.2.1 && act=out";
        let mut parser = Parser::new(input);
        match parser.parse() {
            Ok(expr) => {
                // println!("{:#?}", expr);
                let result = evaluate_expression(&expr, &data);
                println!("Match result: {}", result);
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
