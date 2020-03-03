use std::error::Error;
use structopt::StructOpt;

/// Parse a single key-value pair
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error>>
where
    T: std::str::FromStr,
    T::Err: Error + 'static,
    U: std::str::FromStr,
    U::Err: Error + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

#[derive(StructOpt, Debug)]
pub enum Command {
    /// The generic version of the other commands
    IssueCommand {
        #[structopt(short, long = "prop", parse(try_from_str = parse_key_val), name = "key=value")]
        properties: Vec<(String, String)>,
        command: String,
        message: String,
    },
    /// Set and environment variable for future actions in the job
    ///
    /// Creates or updates an environment variable for any actions running next in a job.
    /// The action that creates or updates the environment variable does not have access to the
    /// new value, but all subsequent actions in a job will have access. Environment variables are
    /// case-sensitive and you can include punctuation.
    SetEnv {
        /// The name of the variable to set
        key: String,
        /// The value of the variable to set
        value: String,
    },
    /// Like set-env but exports an existing environment variable
    Export {
        /// The environment variable key
        key: String,
    },
    /// Set an output parameter.
    ///
    /// Sets an action's output parameter.
    /// Optionally, you can also declare output parameters in an action's metadata file.
    SetOutput {
        /// Name of the output to set
        name: String,
        /// Value to store
        value: String,
    },
    /// Add a system path
    ///
    /// Prepends a directory to the system `PATH` variable for all subsequent actions in the
    /// current job. The currently running action cannot access the new path variable.
    AddPath {
        /// An absolute or relative path. Relative paths automatically get expanded to their
        /// absolute value.
        path: String,
    },
    /// Gets whether Actions Step Debug is on or not.
    ///
    /// If the exit status of that command is zero then the action step debug is on.
    IsDebug,
    /// Set a debug message
    ///
    /// Creates a debug message and prints the message to the log. You can optionally provide a
    /// filename (file), line number (line), and column (col) number where the warning occurred.
    ///
    /// You must create a secret named `ACTIONS_STEP_DEBUG` with the value `true` to see the debug
    /// messages set by this command in the log.
    Debug {
        #[structopt(short, long)]
        file: Option<String>,
        #[structopt(short, long)]
        line: Option<u64>,
        #[structopt(short, long)]
        col: Option<u64>,
        /// Debug message
        message: String,
    },
    /// Set a warning message
    ///
    /// Creates a warning message and prints the message to the log. You can optionally provide a
    /// filename (file), line number (line), and column (col) number where the warning occurred.
    Warning {
        #[structopt(short, long)]
        file: Option<String>,
        #[structopt(short, long)]
        line: Option<u64>,
        #[structopt(short, long)]
        col: Option<u64>,
        /// Issue message
        message: String,
    },
    /// Set an error message
    ///
    /// Creates an error message and prints the message to the log. You can optionally provide a
    /// filename (file), line number (line), and column (col) number where the warning occurred.
    Error {
        #[structopt(short, long)]
        file: Option<String>,
        #[structopt(short, long)]
        line: Option<u64>,
        #[structopt(short, long)]
        col: Option<u64>,
        /// Issue message
        message: String,
    },
    /// Mask a value in log
    ///
    /// Masking a value prevents a string or variable from being printed in the log. Each masked
    /// word separated by whitespace is replaced with the `*` character. You can use an
    /// environment variable or string for the mask's value.
    AddMask {
        /// Value of the secret
        value: String,
    },
    /// Gets the value of an input. The value is also trimmed.
    GetInput {
        name: String,
        #[structopt(short, long)]
        required: bool,
    },
    /// Begin an output group.
    ///
    /// Output until the next `groupEnd` will be foldable in this group
    StartGroup {
        /// The name of the output group
        name: String,
    },
    /// End an output group.
    EndGroup,
    /// Saves state for current action, the state can only be retrieved by this action's post job execution.
    SaveState {
        /// Name of the state to store
        name: String,
        /// Value to store
        value: String,
    },
    /// Gets the value of an state set by this action's main execution.
    GetState {
        /// Name of the state to get
        name: String,
    },
}

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(subcommand)]
    command: Command,
}

fn escape_data<T: AsRef<str>>(s: T) -> String {
    s.as_ref().replace("%", "%25").replace("\r", "%0D").replace("\n", "%0A")
}

fn escape_property<T: AsRef<str>>(s: T) -> String {
    escape_data(s).replace(":", "%3A").replace(",", "%2C")
}

// https://github.com/actions/toolkit/blob/3261dd988308cb227481dc6580026034e5780160/packages/core/src/command.ts
fn issue_command<T, U>(command: T, message: U, properties: Vec<(String, String)>) -> String
where
    T: AsRef<str>,
    U: AsRef<str>,
{
    let mut cmd_str = format!("::{}", command.as_ref());

    if properties.len() > 0 {
        let joined_props = properties.iter().map(|(key, value)|
            format!("{}={}", key, escape_property(value))
        ).collect::<Vec<String>>().join(",");

        cmd_str = format!("{} {}", cmd_str, joined_props);
    }

    cmd_str = format!("{}::{}", cmd_str, escape_data(message.as_ref()));

    cmd_str
}

fn log_command<T, U>(command: T, message: U, file: Option<String>, line: Option<u64>, col: Option<u64>) -> String
where
    T: AsRef<str>,
    U: AsRef<str>,
{
    let mut params = Vec::new();
    if let Some(file) = file {
        params.push(("file".to_owned(), file.to_owned()))
    }
    if let Some(line) = line {
        params.push(("line".to_owned(), format!("{}", line)))
    }
    if let Some(col) = col {
        params.push(("col".to_owned(), format!("{}", col)))
    }
    issue_command(command, message, params)
}

fn issue<T, U>(command: T, message: U) -> String
where
    T: AsRef<str>,
    U: AsRef<str>,
{
    issue_command(command, message, vec!())
}

fn main() {
    let opt = Opt::from_args();

    let out = match opt.command {
        Command::IssueCommand { command, message, properties } => issue_command(&command[..], message, properties),
        Command::SetEnv { key, value } => {
            issue_command("set-env", value, vec!(("name".to_owned(), key)))
        },
        Command::Export { key } => {
            match std::env::var(key.clone()) {
                Ok(val) => issue_command("set-env", val, vec!(("name".to_owned(), key))),
                Err(e) => panic!(e)
            }
        },
        Command::SetOutput { name, value } => {
            issue_command("set-output", value, vec!(("name".to_owned(), name)))
        },
        Command::AddPath { path } => {
            match std::fs::canonicalize(path) {
                Ok(path) => issue("add-path", path.to_string_lossy().into_owned()),
                Err(e) => panic!(e),
            }
        },
        Command::AddMask { value } => {
            issue("add-mask", value )
        },
        Command::GetInput { name, required } => {
            let key = format!("INPUT_{}", name.replace(" ", "_").to_ascii_uppercase());
            match std::env::var(key) {
                Ok(val) => val,
                Err(e) => if required { panic!(e) } else { "".to_owned() }
            }
        },
        Command::IsDebug => {
            match std::env::var("RUNNER_DEBUG") {
                Ok(val) => val,
                Err(e) => panic!(e)
            }
        },
        Command::Debug { message, file, line, col } => {
            log_command("debug", message, file, line, col)
        },
        Command::Warning { message, file, line, col } => {
            log_command("warning", message, file, line, col)
        },
        Command::Error { message, file, line, col } => {
            log_command("error", message, file, line, col)
        },
        Command::StartGroup { name } => {
            issue("group", name)
        },
        Command::EndGroup => {
            issue("endgroup", "".to_owned())
        },
        Command::SaveState { name, value } => {
            issue_command("save-state", value, vec!(("name".to_owned(), name)))
        },
        Command::GetState { name } => {
            let key = format!("STATE_{}", name);
            match std::env::var(key) {
                Ok(val) => val,
                Err(_) => "".to_owned(),
            }
        },
    };

    println!("{}", out)
}
