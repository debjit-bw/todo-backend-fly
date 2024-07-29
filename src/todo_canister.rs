// pub mod todo_canister {
    use anyhow::Result;
    use ic_agent::export::Principal;
    use ic_agent::Agent;
    use ic_agent::identity::Secp256k1Identity;
    use serde::Serialize;
    use tokio::{self, sync::OnceCell};
    use candid::{CandidType, Decode, Deserialize, Encode};
    use rand::Rng;

    #[derive(Clone, CandidType, Deserialize, Debug, Serialize)]
    pub struct Todo {
        id: u64,
        text: String,
        completed: bool,
    }

    #[derive(CandidType)]
    struct AddTodoArgs {
        todos: Vec<String>,
    }

    #[derive(CandidType, Deserialize, Serialize)]
    pub struct ToggleResult {
        state: bool,
        error: Option<String>,
    }

    #[derive(CandidType, Deserialize)]
    struct  AddTodosResponse {
        count: u64,
    }

    #[derive(CandidType)]
    struct GetTodoArgs {
        id: u64,
    }

    #[derive(CandidType, Deserialize)]
    struct GetTodoResponse {
        record: Option<Todo>,
    }

    #[derive(CandidType)]
    struct GetPaginatedTodosArgs {
        offset: u64,
        limit: u64,
    }

    // println!("Hello, world!");
    static AGENT: OnceCell<Agent> = OnceCell::const_new();
    static PRINCIPAL: OnceCell<Principal> = OnceCell::const_new();
    // println!("Agent created");

    pub fn initialize() -> Result<()> {
        println!("Initializing agent");
        let agent = Agent::builder()
            .with_url("https://ic0.app")
            .with_identity(Secp256k1Identity::from_pem_file("./identity.pem")?)
            // .with_identity(Secp256k1Identity::from_pem_file("/usr/local/bin/identity.pem")?)
            .build()?;

        AGENT.set(agent).map_err(|_| anyhow::anyhow!("Agent already initialized"))?;
        println!("Agent initialized");

        println!("Creating principal");
        let principal = Principal::from_text("62nkg-7yaaa-aaaan-qmpla-cai").unwrap();
        PRINCIPAL.set(principal).map_err(|_| anyhow::anyhow!("Principal already initialized"))?;
        println!("Principal created");

        Ok(())
    }

    // pub fn create_agent(url: &str) -> Option<Agent> {
    //     let agent = Agent::builder()
    //         .with_url(url)
    //         .with_identity(Secp256k1Identity::from_pem_file("/Users/debjit/.config/dfx/identity/terminal/identity.pem").expect("Unable to read pem file"))
    //         .build().expect("Unable to create agent");
    //     Some(agent)
    // }

    pub async fn get_todos() -> Vec<Todo> {
        let response = AGENT.get().expect("AGENT not initialized").query(&PRINCIPAL.get().expect("PRINCIPAL not set"), "getPaginatedTodos")
            .with_arg(Encode!(&(0 as u64), &(100 as u64)).unwrap())
            .call()
            .await.unwrap();
        Decode!(&response, Vec<Todo>).unwrap()
    }

    pub async fn add_todo() -> u64 {
        let randomness = rand::thread_rng().gen_range(0..1000);
        let response = AGENT.get().expect("AGENT not initialized").update(&PRINCIPAL.get().expect("PRINCIPAL not set"), "addTodos")
        .with_arg(Encode!(&vec![format!("Test todo {}", randomness).to_string()]).unwrap())
            .call_and_wait()
            .await.unwrap();
        Decode!(&response, u64).unwrap()
    }

    pub async fn get_todo_by_id(id: u64) -> Option<Todo> {
        let response = AGENT.get().expect("AGENT not initialized").query(&PRINCIPAL.get().expect("PRINCIPAL not set"), "getTodo")
            .with_arg(Encode!(&GetTodoArgs { id }).unwrap())
            .call()
            .await.unwrap();
        Decode!(&response, GetTodoResponse).unwrap().record
    }

    pub async fn toggle_todo_by_id(todo_id: u64) -> ToggleResult {
    // pub async fn toggle_todo_by_id(Path(todo_id): Path<u64>) -> Option<Todo> {
        let response = AGENT.get().expect("AGENT not initialized").update(&PRINCIPAL.get().expect("PRINCIPAL not set"), "toggleTodo")
            .with_arg(Encode!(&(todo_id as u64)).unwrap())
            .call_and_wait()
            .await.unwrap();
        Decode!(&response, ToggleResult).unwrap()
    }

    // #[tokio::main]
    // async fn main() {
        
        // let canister = Canister::builder()
        //     .with_agent(&agent)
        //     .with_canister_id("62nkg-7yaaa-aaaan-qmpla-cai")
        //     .build()
        //     .unwrap();
        // let agent = create_agent("https://ic0.app").unwrap();
        
        // let principal = Principal::from_text("62nkg-7yaaa-aaaan-qmpla-cai").unwrap();
        // println!("Principal created\n");
        // panic!("test");

        // let randomness = rand::thread_rng().gen_range(0..1000);

        // let response = agent.update(&principal, "addTodos")
        //     .with_arg(Encode!(&vec![format!("Test todo {}", randomness).to_string()]).unwrap())
        //     .call_and_wait()
        //     .await.unwrap();
        // println!("made addTodo call with randomness: {}", randomness);
        // let count = Decode!(&response, u64).unwrap();
        // println!("count: {:?}\n", count);

        // let response = agent.query(&principal, "getTodo")
        //     .with_arg(Encode!(&((count-1) as u64)).unwrap())
        //     .call()
        //     .await.unwrap();
        // println!("made getTodo call");
        // let todo = Decode!(&response, Option<Todo>).unwrap().unwrap();
        // println!("todo: {:?}", todo);
    // }
// }