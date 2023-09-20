use std::process::Command;

/**
 * Execute an command and return result of stderr (Err) or stdout (Ok)
 */
pub fn exec(command: &mut Command) -> Result<String, String> {
    let out = command.output();

    if out.is_err() {
        return Result::Err(out.unwrap_err().to_string());
    }

    return Result::Ok(String::from_utf8(out.unwrap().stdout).unwrap());
}

//Struct std::process::Command. A process builder, 
//providing fine-grained control over how a new process should be spawned.
// Command can be reused to spawn multiple processes.

// use std::process::exec;
//This library is used for extending Command in order to execute programs more easily.