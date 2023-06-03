use shrs::ShellBuilder;
use shrs_insulter::InsulterPlugin;

fn main() {
    let myshell = ShellBuilder::default()
        .with_plugin(InsulterPlugin::new(vec![], 1., true))
        .build()
        .unwrap();

    myshell.run();
}
