# create-py-app
CLI Tool to create a basic python repository structure



## Usage
```bash
create-py-app automatically sets up a new python project structure for you.

Usage: create-py-app.exe <COMMAND>

Commands:
  create
    -n, --name <NAME> # REQUIRED
    -p, --python-version <PYTHON_VERSION> # OPTIONAL

  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version
```

Running the installer provided in the release will add the `create-py-app` command to your PATH. You can then run the command from anywhere on your system.

Currently, this version only supports running in the directory you want to create the project in. It will generate a directory named whatever you passed in for `--name` with all of the content pre-populated for you. Future versions will support specifying a directory to create the project in.

### Examples
```bash
Usage: create-py-app.exe create --name <NAME> --python-version <PYTHON_VERSION>

# explicit
create-py-app create --name my-new-app --python-version 3.8

# auto-discover python version
create-py-app create --name my-new-app
```


# Installation
Go to the releases tab & select the latest release. Download the installer, 'create-py-app-installer.exe', run it. The installer will add the `create-py-app` command to your PATH & install the necessary files to your system.

## Post beta:
- [ ] Add unit tests
- [ ] Add CI/CD for auto-builds & release cycles
- [ ] Add support for GitHub template files too.
- [ ] Add support for specifying an installation location.
