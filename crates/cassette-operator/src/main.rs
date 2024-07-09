mod ctx;

use ark_core_k8s::manager::Ctx;
use tokio::join;

pub(crate) mod consts {
    pub const NAME: &str = "cassette-operator";
    pub const NAMESPACE: &str = NAME;
}

#[tokio::main]
async fn main() {
    join!(
        self::ctx::cassette::Ctx::spawn_crd(),
        self::ctx::component::Ctx::spawn_crd(),
    );
}
