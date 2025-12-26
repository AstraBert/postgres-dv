use regex::Regex;

pub fn is_valid_select_query(query: &str) -> bool {
    let re = Regex::new(r#"^SELECT\s+.+?\s+FROM\s+[a-zA-Z_][a-zA-Z0-9_]*[^;]*;$"#)
        .expect("Regex should compile, but an error occurred");
    let sanitized_query = query.trim().replace("\n", " ");
    re.is_match(&sanitized_query)
}

pub fn is_valid_postgres_connection_string(connection_string: &str) -> bool {
    let re = Regex::new(r#"^postgres(?:ql)?://[^:]+:[^@]+@[^/]+/[^?]+(?:\?.*)?$"#)
        .expect("Regex should compile, but an error occurred");
    let sanitized_string = connection_string.trim();
    re.is_match(sanitized_string)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_valid_select_query() {
        assert!(is_valid_select_query(
            "SELECT something FROM some_table WHERE some_quality = 'hello' ORDER BY something_else LIMIT 1;"
        ));
        assert!(!is_valid_select_query("SELECT hello;"));
        assert!(!is_valid_select_query(
            "DELETE table WHERE somthing = 'something else';"
        ));
    }

    #[test]
    fn test_is_valid_postgres_connection_string() {
        assert!(is_valid_postgres_connection_string(
            "postgresql://user:password@host.com/database?param1=value1&param2=value2"
        ));
        assert!(is_valid_postgres_connection_string(
            "postgres://user:password@host.com/database"
        ));
        assert!(is_valid_postgres_connection_string(
            "postgresql://user:password@localhost:5432/postgres"
        ));
        assert!(!is_valid_postgres_connection_string(
            "postgresql://userpassword@host.com/database"
        ));
        assert!(!is_valid_postgres_connection_string(
            "sqlite:///Users/user/databases/file.db"
        ));
        assert!(!is_valid_postgres_connection_string(
            "postgresql://user:password/database"
        ));
    }
}
