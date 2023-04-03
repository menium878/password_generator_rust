#[cfg(test)]
mod tests {
    use crate::{PasswordProperties,PasswordKind, generate_password};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    
    #[test]
    fn test_generate_password() {
        let x=generate_password(PasswordProperties::new(3, PasswordKind::Letter));
        
        assert_eq!(x.len(), 3);
    }

}