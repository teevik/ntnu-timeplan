use ntnu_timeplan_api::router::rspc_router;
use std::path::PathBuf;

fn main() {
    rspc_router()
        .export_ts(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("./bindings.ts"))
        .unwrap()
}
