use std::string::String;

/// We write the docs here to be returned when `/api/v1/docs` is hit.
pub fn docs() -> String{
    return "
    Welcome to RustyVault Docs! minimal documentation:

    1) GET : '/'.
        use '/' to check if RustyWeb API is active and listening activelty to localhost at port 8080

    2) GET : '/api/v1/docs'.
        use '/api/v1/docs' to browse to the minimal docs for development.

    3) GET : '/api/v1/health'.
        use '/api/v1/health' to monitor health of the application.

    4) POST : '/api/v1/vault/create'.
        use the '/api/v1/vault/create' to create a new vault.
        Body : name, replicas (pass as a raw json body).    
    ".to_string();
}