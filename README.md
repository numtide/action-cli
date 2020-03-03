# actions-cli - GitHub actions without JavaScript

Some weird people enjoy writing Bash more than JavaScript.

This little tool wraps common tasks that one would do with GitHub actions and
is currently only supported by https://github.com/actions/toolkit/

This first version wraps all of the logging commands from https://help.github.com/en/actions/reference/development-tools-for-github-actions#logging-commands
## Usage

TODO: add one-liner installation

`$ actions-cli --help`
```
action 0.1.0

USAGE:
    actions-cli <SUBCOMMAND>

FLAGS:
    -h, --help       
            Prints help information

    -V, --version    
            Prints version information


SUBCOMMANDS:
    add-path         Prepends input_path to the PATH (for future actions)
    debug            Writes debug message to user log
    end-group        End an output group
    error            Adds an error issue
    export           Exports an existing env variable for future actions in the job
    get-input        Gets the value of an input. The value is also trimmed
    get-state        Gets the value of an state set by this action's main execution
    help             Prints this message or the help of the given subcommand(s)
    info             Writes info to log. This is a bit useless, it's just prints to stdout
    is-debug         Gets whether Actions Step Debug is on or not
    issue-command    The generic version of the other commands
    save-state       Saves state for current action, the state can only be retrieved by this action's post job
                     execution
    set-output       Sets the value of an output
    set-secret       Registers a secret which will get masked from logs
    start-group      Begin an output group
    warning          Adds a warning issue
```

## Example

`$ actions-cli add-path ~/.local/bin`
```
::add-path::/home/zimbatm/.local/bin
```

