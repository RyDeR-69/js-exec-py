use pyo3_stub_gen::{Result, TypeInfo};

fn main() -> Result<()> {
    let mut stub = js_exec::stub_info()?;

    for (_, module) in stub.modules.iter_mut() {
        for (_, class) in module.class.iter_mut() {
            if let Some(new) = class.new.take() {
                let new_method = pyo3_stub_gen::generate::MethodDef {
                    name: "__init__",
                    args: new.args.clone(),
                    r#return: TypeInfo::unqualified(class.name),
                    doc: "",
                    is_static: false,
                    is_class: false,
                };

                // Insert the new method at the beginning of the methods vector
                class.methods.insert(0, new_method);
            }
        }
    }
    stub.generate()?;
    // try to run command "ruff format" to format the generated code
    let result = std::process::Command::new("ruff")
        .arg("format")
        .arg("python")
        .status();
    if let Ok(status) = result {
        if !status.success() {
            eprintln!("Failed to run ruff format");
        }
    }
    Ok(())
}
