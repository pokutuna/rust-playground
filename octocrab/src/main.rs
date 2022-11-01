// use octocrab::Octocrab;

fn main() {
    let task = async {
        let octocrab = octocrab::instance();
        let repo = octocrab.repos("rust-lang", "rust").get().await;
        return repo;
    };

    let rt = tokio::runtime::Runtime::new().unwrap();
    let repo = rt.block_on(task);
    match repo {
        Ok(r) => println!("{}", r.full_name.unwrap()),
        _ => (),
    }
}
