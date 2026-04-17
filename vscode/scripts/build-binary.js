const fs = require("fs");
const path = require("path");
const { spawnSync } = require("child_process");

const extensionRoot = path.resolve(__dirname, "..");
const repoRoot = path.resolve(extensionRoot, "..");

const platformMap = {
  aix: "aix",
  darwin: "darwin",
  freebsd: "freebsd",
  linux: "linux",
  openbsd: "openbsd",
  win32: "win32"
};

const archMap = {
  arm64: "arm64",
  arm: "arm",
  ia32: "ia32",
  x64: "x64"
};

const platform = platformMap[process.platform];
const arch = archMap[process.arch];

if (!platform || !arch) {
  throw new Error(`Unsupported platform for bundled qualirs binary: ${process.platform}-${process.arch}`);
}

const result = spawnSync("cargo", ["build", "--release", "--bin", "qualirs"], {
  cwd: repoRoot,
  stdio: "inherit"
});

if (result.status !== 0) {
  process.exit(result.status ?? 1);
}

const executableName = process.platform === "win32" ? "qualirs.exe" : "qualirs";
const source = path.join(repoRoot, "target", "release", executableName);
const targetDir = path.join(extensionRoot, "bin", `${platform}-${arch}`);
const target = path.join(targetDir, executableName);

fs.mkdirSync(targetDir, { recursive: true });
fs.copyFileSync(source, target);

if (process.platform !== "win32") {
  fs.chmodSync(target, 0o755);
}

console.log(`Bundled ${target}`);
