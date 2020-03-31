# action-cli - GitHub actions without JavaScript

Some weird people (like me) enjoy writing Bash more than JavaScript.

This little tool wraps common tasks that one would do with GitHub Actions and
is currently only supported by https://github.com/actions/toolkit/

This first version wraps all of the logging commands from
https://help.github.com/en/actions/reference/development-tools-for-github-actions#logging-commands

## Installation

action-cli can easily be installed in a pipeline and only adds ~1s to it:

```yaml
name: 'action-cli'
on: ["push"]
jobs:
  self-test:
    name: Self test
    runs-on: ubuntu-latest
    steps:
      - uses: numtide/action-cli@v0.6.0
      - run: action-cli warning --file Cargo.toml --line 2 --col 2 "Ooops"
```

## Usage

Here are all the commands available once the CLI is installed:

`$ action-cli --help`
```
action-cli 0.6.0

USAGE:
    action-cli <SUBCOMMAND>

FLAGS:
    -h, --help       
            Prints help information

    -V, --version    
            Prints version information


SUBCOMMANDS:
    add-mask         Mask a value in log
    add-path         Add a system path
    debug            Set a debug message
    end-group        End an output group
    error            Set an error message
    export           Like set-env but exports an existing environment variable
    get-input        Gets the value of an input. The value is also trimmed
    get-state        Gets the value of an state set by this action's main execution
    help             Prints this message or the help of the given subcommand(s)
    is-debug         Gets whether Actions Step Debug is on or not
    issue-command    The generic version of the other commands
    post-comment     Creating comment based on issues and pull requests
    save-state       Saves state for current action, the state can only be retrieved by this action's post job
                     execution
    set-env          Set and environment variable for future actions in the job
    set-output       Set an output parameter
    start-group      Begin an output group
    stop-commands    Stop and start log commands
    warning          Set a warning message
```

## License

This work is licensed under the Apache License 2.0.
See [LICENSE](LICENSE) for more details.

## Sponsors

This work has been sponsored by [NumTide](https://numtide.com).

[![NumTide](https://avatars3.githubusercontent.com/u/20373834?s=200&v=4)](https://numtide.com)

