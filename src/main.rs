use plog::{ok, warn, error, debug, info};

fn main() {
    let vec: Vec<_> = (0..10)
        .collect();
    ok!("it's working! {vec:?}");
    info!("processing `vec`! {vec:?}");
    debug!("well it's working! {vec:?}");
    warn!("well it's working! {vec:?}");
    error!("well it's working! {vec:?}");
}
