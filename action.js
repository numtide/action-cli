const child_process = require('child_process');
const fs = require('fs');
const http = require('http');
const os = require('os');
const path = require('path');

// Get the version from the Cargo.toml file
const cargoToml = fs.readFileSync(path.join(__dirname, 'Cargo.toml'));
const version = /version *= *"([^"]+)"/.exec(cargoToml)[1];

// Build the download URL
const baseURL = "https://github.com/numtide/action-cli/releases/download/v" + version;
const system = os.platform() + "-" + os.arch()

let filename;
switch(system) {
  case "linux-x64":
    filename = "action-cli-x86_64-unknown-linux-musl.tar.gz";
    break;
  case "darwin-x64":
    filename = "action-cli-x86_64-apple-darwin.tar.gz";
    break;
  case "win32-x64":
    filename = "action-cli-x86_64-pc-windows-msvc.zip";
    break;
  default:
    throw "System " + system + " not supported";
}
const url = baseURL + "/" + filename;

// Use the first PATH entry as the destination
const binDir = process.env.PATH.split(path.delimiter)[0];

// And finally download and unpack
console.log("Installing", url, "to", binDir);

switch(system) {
  case "linux-x64":
  case "darwin-x64":
    child_process.execSync(`curl -fL ${url} | tar -xzC ${binDir}`)
    break;
  // TODO
  //case "win32-x64":
  default:
    throw "System " + system + " not supported";
}

child_process.execSync('action-cli --help >&2')
console.log();
console.log(`action-cli v${version} installed`);
