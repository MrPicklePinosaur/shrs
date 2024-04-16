use shrs::prelude::*;
use shrs_analytics::AnalyticsPlugin;

fn main() {
    let myshell = ShellBuilder::default()
        .with_plugin(AnalyticsPlugin::new())
        .build()
        .unwrap();

    myshell.run().unwrap();
}
