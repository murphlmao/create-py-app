# {{ name }}

## Dependencies:
{{ python_version }}

## How To Run:
First setup your environment:
```bash
cd src # this is your running context

# setup virtual environment to work in
python -m venv venv
source venv/bin/activate
pip install --upgrade pip
pip install -r requirements.txt
```

Then run the main script:
```bash
python -m {{ name }}.main
```

Alternatively, if you don't want the `./src/` directory as your primary running context, you can do this:
```bash
export PYTHONPATH=$(pwd)/..:$PYTHONPATH
cd src/{{ name }}
python -m main
```


### Dev Mode
To run in dev mode, you can export DEV as a boolean value to determine your dev environment command: \
Linux:
```bash
export DEV=true # non-case sensitive
```

Windows equivalent:
```powershell
$env:DEV = "true"
```

Dev mode will create a `_` directory in `./src/{{ name }}/_/` that will act as the 'installation' directory of your project.
This is useful for debugging & testing one environment without worrying about other filesystem changes.