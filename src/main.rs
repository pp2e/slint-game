slint::include_modules!();

use std::path::PathBuf;
use slint::ComponentFactory;
use i_slint_core::component_factory::FactoryContext;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

use spin_on;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
#[tokio::main]
pub async fn main() {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();
    
    let ui = AppWindow::new().unwrap();

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    let ui_handle = ui.as_weak();
    ui.on_compile_ui(move | text: slint::SharedString | {
        let ui = ui_handle.unwrap();
        
        let mut compiler = slint_interpreter::ComponentCompiler::default();

        let compiled = spin_on::spin_on(compiler.build_from_source(text.to_string(), PathBuf::default()));

        let compiled = compiled.unwrap();
        
        let factory = ComponentFactory::new(move |ctx: FactoryContext| {
        	let instance = compiled.create_embedded(ctx).unwrap();
        	Some(instance)
        });
        
        ui.set_factory(factory);
    });

    ui.run().unwrap();
}
