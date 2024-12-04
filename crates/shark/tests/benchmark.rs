#[cfg(test)]
mod tc;

#[cfg(test)]
mod unit {

    use std::collections::HashMap;

    use shark::specs::sip::{parse_token, parse_token_with_cache};

    use crate::{arch_finish, arch_start};

    #[test]
    fn test_certificate() {
        let token1 = "sip:test@10.0.2.15:5060";
        let token2 = "sip:sip.cybercity.dk";
        let token3 = "sip:user@example.com:5060;transport=udp?subject=project&priority=urgent";

        let count = 100000;
        arch_start!("parse_each");
        for _ in 0..count {
            parse_token(token1);
            parse_token(token2);
            parse_token(token3);
        }
        arch_finish!("parse_each");

        arch_start!("mapping");
        for _ in 0..count {
            parse_token_with_cache(token1);
            parse_token_with_cache(token2);
            parse_token_with_cache(token3);
        }
        arch_finish!("mapping");
    }
}
