const fs = require("fs");
const path = require("path");

const extensionRoot = path.resolve(__dirname, "..");
const repoRoot = path.resolve(extensionRoot, "..");
const cargoTomlPath = path.join(repoRoot, "Cargo.toml");
const packageJsonPath = path.join(extensionRoot, "package.json");
const packageLockPath = path.join(extensionRoot, "package-lock.json");

function readCargoPackageVersion(cargoToml) {
  const lines = cargoToml.split(/\r?\n/);
  let inPackageSection = false;

  for (const line of lines) {
    if (/^\s*\[.*\]\s*$/.test(line)) {
      inPackageSection = /^\s*\[package\]\s*$/.test(line);
      continue;
    }

    if (!inPackageSection) {
      continue;
    }

    const version = line.match(/^\s*version\s*=\s*"([^"]+)"\s*$/);

    if (version) {
      return version[1];
    }
  }

  throw new Error("Could not find package version in Cargo.toml");
}

function writeJson(filePath, value) {
  fs.writeFileSync(filePath, `${JSON.stringify(value, null, 2)}\n`);
}

const cargoToml = fs.readFileSync(cargoTomlPath, "utf8");
const cargoVersion = readCargoPackageVersion(cargoToml);

const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, "utf8"));
packageJson.version = cargoVersion;
writeJson(packageJsonPath, packageJson);

if (fs.existsSync(packageLockPath)) {
  const packageLock = JSON.parse(fs.readFileSync(packageLockPath, "utf8"));
  packageLock.version = cargoVersion;

  if (packageLock.packages?.[""]) {
    packageLock.packages[""].version = cargoVersion;
  }

  writeJson(packageLockPath, packageLock);
}

console.log(`Synced VS Code extension version to ${cargoVersion}`);
