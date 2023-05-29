use shrs::ShellConfigBuilder;
use shrs_insulter::InsulterPlugin;

fn main() {
    let myshell = ShellConfigBuilder::default()
        .with_plugin(InsulterPlugin::new(vec![], 1., true))
        .build()
        .unwrap();

    myshell.run();
}
