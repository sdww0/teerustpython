use crate::bytecode::FrozenModule;
use std::collections::HashMap;
use std::string::String;
use std::vec::Vec;
use std::boxed::Box;
use std::vec;
use std::format;
use std::string::ToString;
pub fn get_module_inits() -> HashMap<String, FrozenModule> {
    let mut modules = HashMap::new();

    macro_rules! ext_modules {
        ($($t:tt)*) => {
            modules.extend(py_compile_bytecode!($($t)*));
        };
    }

    ext_modules!(
        source = "initialized = True; print(\"Hello world!\")\n",
        module_name = "__hello__",
    );

    // Python modules that the vm calls into, but are not actually part of the stdlib. They could
    // in theory be implemented in Rust, but are easiest to do in Python for one reason or another.
    // Includes _importlib_bootstrap and _importlib_bootstrap_external
    ext_modules!(dir = "vm/Lib/python_builtins/");

    #[cfg(not(feature = "freeze-stdlib"))]
    {
        // core stdlib Python modules that the vm calls into, but are still used in Python
        // application code, e.g. copyreg
        ext_modules!(dir = "vm/Lib/core_modules/");
    }
    // if we're on freeze-stdlib, the core stdlib modules will be included anyway
    #[cfg(feature = "freeze-stdlib")]
    {
        ext_modules!(dir = "../Lib/");
    }

    modules
}
