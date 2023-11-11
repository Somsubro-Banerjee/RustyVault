use std::collections::HashMap;
use uuid::Uuid;
/// Define the structure of each node:
/// Each node will have one ID for it's identificaiton and one for storing the Data.
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Node {
    pub name: String,
    pub id: String,
    pub data: HashMap<String, String>,
    pub status: bool,
}

/// implement methods to store data and initialize a new node 
/// declare a funciton 'new' to initialize the Node with an ID.
/// declare a function 'store' that accepts a key value pair and create a entru in the data secition of the node.
/// declare a function 'retrieve' that returns the value of the query 'key' being passed as a parameter.
impl Node { 

    /// function `new()` creates a new Node (Vault)
    /// it returns a Result of `Node` (Self) or `Error` in form of String.
    /// ```
    /// pub fn new(vault_name: String) -> std::result::Result<Self, String> {
    /// let node = Node {
    ///    name: vault_name.clone(),
    ///    id: Uuid::new_v4().to_string(),
    ///    data: HashMap::new(),
    ///    status: true
    /// };
    /// if node.name == vault_name.clone(){
    ///    Ok(node)
    /// } else {
    ///    Err("Some error occurred while creating vault".to_string())
    ///   }
    /// }
    /// ```
    /// if the new vault is created then it's `name` should match the `vault_name` passed in the `new(vault_name: String)` function's paramater.
    ///  # Example
    /// ```
    ///  let mut node1 = Node::new();
    ///  let mut node2 = Node::new();
    /// ```
    pub fn new(vault_name: String) -> std::result::Result<Self, String> {
        let node = Node {
            name: vault_name.clone(),
            id: Uuid::new_v4().to_string(),
            data: HashMap::new(),
            status: true
        };
        if node.name == vault_name.clone(){
            Ok(node)
        } else {
            Err("Some error occurred while creating vault".to_string())
        }
    }

    /// function `store()` creates a new entry in the `data` of the Node
    /// ```
    /// pub fn store(&mut self, key: String, value: String) -> String {
    ///     self.data.insert(key.clone(), value);
    ///     return format!("Created Entry with key: {} and value: {:?}", &key, self.data.get(&key))
    /// }
    /// ```
    ///  # Example
    /// ```
    ///  node1.store("Hello".to_string(), "Sam".to_string());
    ///  node2.store("Hello".to_string(), "Brother".to_string());
    /// ```
    pub fn store(&mut self, key: String, value: String) -> String {
        self.data.insert(key.clone(), value);
        return format!("Created Entry with key: {} and value: {:?}", &key, self.data.get(&key))
    }

    /// function `update()` updates an existing entry in the hashmap with the provided `key`
    /// ```
    /// pub fn update(&mut self, key: String, new_value: String) -> Option<&String>{
    ///     self.data.remove_entry(&key);
    ///     self.data.insert(key.clone(), new_value);
    ///     return self.data.get(&key);
    /// }
    /// ```
    ///  # Example
    /// ```
    ///  node1.update("Hello".to_string(), "Stranger".to_string()).expect("Failed to update the node1 value");
    /// ```
    pub fn update(&mut self, key: String, new_value: String) -> Option<&String>{
        self.data.remove_entry(&key);
        self.data.insert(key.clone(), new_value);
        return self.data.get(&key);
    }

    /// function `retrieve()` gets you the secret for the `key` provided in the parameter
    /// ```
    /// pub fn retrieve(&self, key: String) -> Option<&String> {
    ///     return self.data.get(&key);
    /// }
    /// ```
    ///  # Example
    /// ```
    /// println!("{}",node1.retrieve("Hello".to_string()).expect("Failed to retreave value for node2"));
    /// ```
    pub fn retrieve(&self, key: String) -> Option<&String> {
        return self.data.get(&key);
    }

    /// function `replicate()` creates a new instance of the Nodes to make it available parallely <br>
    /// The function returns a vector of Nodes which are basically new instances.
    /// all the nodes have a differnt UUID. 
    /// ```
    /// pub unsafe fn replicate(&self, instance: usize,) -> Vec<Node> {
    /// let mut replicated_nodes: Vec<Node> = Vec::new();
    /// for _ in 0..instance {
    ///    let replicated_node = Node {
    ///        name: self.name.clone(),
    ///        id: Uuid::new_v4().to_string(),
    ///        data: self.data.clone(),
    ///        status: true
    ///       };
    ///    replicated_nodes.push(replicated_node);
    ///    }
    ///  replicated_nodes
    /// }
    /// ```
    /// ## "UNSAFE" this function is still under development please use with care!!! ##
    /// 
    /// # Example
    /// ```
    /// let mut vault = Node::new();
    /// vault.store("Hello".to_string(), "Somsubro".to_string());
    /// vault.store("Hello".to_string(), "Banerjee".to_string());
    /// vault.store("Password".to_string(), "SuperSecretPassword".to_string());
    /// vault.store("Login ID".to_string(), "EmailID".to_string());
    /// vault.store("email_id".to_string(), "ABCD@Gmail.com".to_string());
    /// let replica_set = unsafe { vault.replicate(3) };
    /// for instance in replica_set {
    ///     println!("{:#?}", instance)
    /// }
    /// ```
    pub unsafe fn replicate(&self, instance: usize,) -> Vec<Node> {
        let mut replicated_nodes: Vec<Node> = Vec::new();
        for _ in 0..instance {
            let replicated_node = Node {
                name: self.name.clone(),
                id: Uuid::new_v4().to_string(),
                data: self.data.clone(),
                status: true
            };
            replicated_nodes.push(replicated_node);
        }
        replicated_nodes
    }
}
