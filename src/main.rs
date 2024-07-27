use anyhow::Result;
use ic_agent::export::Principal;
use ic_agent::Agent;
use ic_agent::identity::Secp256k1Identity;
// use ic_utils::canister::Canister;
use tokio;
use candid::{CandidType, Decode, Deserialize, Encode};
use rand::Rng;

#[derive(Clone, CandidType, Deserialize, Debug)]
struct Todo {
    id: u64,
    text: String,
    completed: bool,
}

#[derive(CandidType)]
struct AddTodoArgs {
    todos: Vec<String>,
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

pub async fn create_agent(url: &str, is_mainnet: bool) -> Result<Agent> {
    let agent = Agent::builder()
        .with_url(url)
        .with_identity(Secp256k1Identity::from_pem_file("/Users/debjit/.config/dfx/identity/terminal/identity.pem")?)
        .build()?;
    if !is_mainnet {
        agent.fetch_root_key().await?;
    }
    Ok(agent)
}

pub async fn get_todos() -> Vec<u64> {
    let results = Vec::<u64>::new();
    results
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let agent = create_agent("https://ic0.app", true).await.unwrap();
    println!("Agent created");
    
    // let canister = Canister::builder()
    //     .with_agent(&agent)
    //     .with_canister_id("62nkg-7yaaa-aaaan-qmpla-cai")
    //     .build()
    //     .unwrap();
    
    let principal = Principal::from_text("62nkg-7yaaa-aaaan-qmpla-cai").unwrap();
    println!("Principal created\n");
    // panic!("test");

    let randomness = rand::thread_rng().gen_range(0..1000);

    let response = agent.update(&principal, "addTodos")
        .with_arg(Encode!(&vec![format!("Test todo {}", randomness).to_string()]).unwrap())
        .call_and_wait()
        .await.unwrap();
    println!("made addTodo call with randomness: {}", randomness);
    let count = Decode!(&response, u64).unwrap();
    println!("count: {:?}\n", count);

    let response = agent.query(&principal, "getTodo")
        .with_arg(Encode!(&((count-1) as u64)).unwrap())
        .call()
        .await.unwrap();
    println!("made getTodo call");
    let todo = Decode!(&response, Option<Todo>).unwrap().unwrap();
    println!("todo: {:?}", todo);
}
