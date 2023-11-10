pub mod node;
use node::node::Node;
fn main() {
    println!("Hello world!!");
    let mut vault = Node::new();
    vault.store("Hello".to_string(), "Somsubro".to_string());
    vault.store("Hello".to_string(), "Banerjee".to_string());
    vault.store("Password".to_string(), "SuperSecretPassword".to_string());
    vault.store("Login ID".to_string(), "EmailID".to_string());
    vault.store("email_id".to_string(), "ABCD@Gmail.com".to_string());

    let replica_set = unsafe { vault.replicate(3) };

    for instance in replica_set {
        println!("{:#?}", instance.id)
    }

}