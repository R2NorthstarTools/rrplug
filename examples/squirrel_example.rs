use rrplug::{
    bindings::squirreldatatypes::SQClosure,
    high::squirrel::{call_sq_object_function, SQHandle},
    prelude::*,
};

pub struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    const PLUGIN_INFO: PluginInfo =
        PluginInfo::new(c"example", c"EXAMPLLEE", c"EXAMPLE", PluginContext::all());

    fn new(_reloaded: bool) -> Self {
        register_sq_functions(great_person);
        register_sq_functions(call_with_random_number);
        register_sq_functions(sum);

        Self {}
    }
}

entry!(ExamplePlugin);

// if it returns an error the function will throw a error in the sqvm which can be caught at the call site
#[rrplug::sqfunction(VM = "SERVER", ExportName = "GreatPerson")]
fn great_person(function: SQHandle<SQClosure>) -> Result<String, rrplug::errors::CallError> {
    // non type safe way of getting a return from a function
    // this could be changed if the crate gets some attention
    let name = call_sq_object_function(sqvm, sq_functions, function)?;

    log::info!("hello, {}", name);

    Ok(name)
}

#[rrplug::sqfunction(VM = "SERVER", ExportName = "CallWithRandomNumber")]
fn call_with_random_number(mut function: SquirrelFn<(i32, String)>) {
    const TOTALY_RANDOM_NUMBER: i32 = 37;

    _ = function.call(
        sqvm,
        sq_functions,
        (TOTALY_RANDOM_NUMBER, TOTALY_RANDOM_NUMBER.to_string()),
    );
}

#[rrplug::sqfunction(VM = "SERVER", ExportName = "Sum")]
fn sum(vec: Vector3, numbers: Vec<f32>) -> f32 {
    vec.x + vec.y + vec.z + numbers.into_iter().sum::<f32>()
}
