## Quick Install

To quickly install the latest verse CLI, run:

```bash
curl -O https://raw.githubusercontent.com/Salai-Kowshikan/V.E.R.S.E/main/CLI/install-verse.sh
chmod +x install-verse.sh
./install-verse.sh
```

This will download and run the install script, which fetches the latest binary and installs it to `/usr/bin` (requires sudo).

# V.E.R.S.E


Backend Setup
venv\Scripts\activate
pip install -r requirements.txt
uvicorn app:app --reload


For API Docs

url/api/docs
url/api/redoc
 

# CLI Build and Install

## Building the verse CLI tarball

From the `CLI` directory, run:

```bash
./build-release.sh
```

This will build the binary and create a tarball at `CLI/release/latest/verse.tar.gz`.

## Installing the verse CLI

To install the latest verse CLI system-wide, run:

```bash
./install-verse.sh
```

This will download the latest release tarball from GitHub and install the `verse` binary to `/usr/bin` (requires sudo).

After installation, you can run the CLI with:

```bash
verse
```