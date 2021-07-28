use clap::{App, load_yaml};
use kube::{api::{Api, ListParams, ResourceExt}, Client};
use k8s_openapi::api::core::v1::Pod;
use futures::executor::block_on;

fn main() {

    let res = run();
    //block_on(res);
}

#[tokio::main]
async fn run() -> Result<(), kube::Error> {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
    let label = matches.value_of("label").expect("");

    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::namespaced(client, "kafka");
    let lp = ListParams::default().labels(label);
    for p in pods.list(&lp).await? {
        print!("Found Pod: {}\n", p.name());
    }

    Ok(())
}
