
#[cfg(test)]
mod unit_test { 
    
    use auth_service::models::account::Account;

    const FIRST_NAME: &str = "john";
    const LAST_NAME: &str = "john";
    const EMAIL: &str = "john@johntest.com";

    #[test]
    fn test() { 

        let mut account = Account::new(EMAIL);
        account.first_name.insert(FIRST_NAME.to_string());
        account.last_name.insert(LAST_NAME.to_string());
        
        
        assert_eq!(FIRST_NAME.to_string(), account.first_name.unwrap());
        assert_eq!(LAST_NAME.to_string(), account.last_name.unwrap());


    }



}