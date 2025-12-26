use regex::Regex;

pub fn is_valid_select_query(query: &str) -> bool {
    let re = Regex::new(r#"^SELECT\s+.+?\s+FROM\s+[a-zA-Z_][a-zA-Z0-9_]*[^;]*;$"#).expect("Regex should compile, but an error occurred");
    let sanitized_query = query.trim().replace("\n", " ");
    re.is_match(&sanitized_query)
}

pub fn is_valid_postgres_connection_string(connection_string: &str) -> bool {
    let re = Regex::new(r#"^postgres(?:ql)?://[^:]+:[^@]+@[^/]+/[^?]+(?:\?.*)?$"#).expect("Regex should compile, but an error occurred");
    let sanitized_string = connection_string.trim();
    re.is_match(&sanitized_string)
}