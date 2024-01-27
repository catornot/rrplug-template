use rrplug::prelude::*;

#[derive(Debug)]
pub struct TemplatePlugin;

impl Plugin for TemplatePlugin {
    const PLUGIN_INFO: PluginInfo = PluginInfo::new(
        "CHANGEME\0",         // name
        "HASTOBE9C\0", // used for the label in the log should be 9 chars long to be consitent
        "CHANGEME\0",  // dependency string that mods can use
        PluginContext::all(), // context -> if it has only client it will not load on dedicated servers
    );

    fn new(_reloaded: bool) -> Self {
        log::info!("yay logging :D");

        register_sq_functions(example_function);

        Self {}
    }

    // omg some more functions in the trait
}

entry!(TemplatePlugin);

#[rrplug::sqfunction(VM = "CLIENT | UI | SERVER", ExportName = "ExampleFunction")]
fn example_function(name: String) -> String {
    format!("hello, {}", name)
}
