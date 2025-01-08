const { platform, arch } = process;
// ripari-ignore lint/style/useNodejsImportProtocol: would be a breaking change, consider bumping node version next major version
const { execSync } = require("child_process");

function isMusl() {
	let stderr;
	try {
		stderr = execSync("ldd --version", {
			stdio: ["pipe", "pipe", "pipe"],
		});
	} catch (err) {
		stderr = err.stderr;
	}
	if (stderr.indexOf("musl") > -1) {
		return true;
	}
	return false;
}

const PLATFORMS = {
	win32: {
		x64: "@simora/cli-win32-x64/ripari.exe",
		arm64: "@simora/cli-win32-arm64/ripari.exe",
	},
	darwin: {
		x64: "@simora/cli-darwin-x64/ripari",
		arm64: "@simora/cli-darwin-arm64/ripari",
	},
	linux: {
		x64: "@simora/cli-linux-x64/ripari",
		arm64: "@simora/cli-linux-arm64/ripari",
	},
	"linux-musl": {
		x64: "@simora/cli-linux-x64-musl/ripari",
		arm64: "@simora/cli-linux-arm64-musl/ripari",
	},
};

const binName =
	platform === "linux" && isMusl()
		? PLATFORMS?.["linux-musl"]?.[arch]
		: PLATFORMS?.[platform]?.[arch];

if (binName) {
	let binPath;
	try {
		binPath = require.resolve(binName);
	} catch {
		console.warn(
			`The Ripari CLI postinstall script failed to resolve the binary file "${binName}". Running Ripari from the npm package will probably not work correctly.`,
		);
	}
} else {
	console.warn(
		"The Ripari CLI package doesn't ship with prebuilt binaries for your platform yet. " +
			"You can still use the CLI by cloning the simora/ripari repo from GitHub, " +
			"and follow the instructions there to build the CLI for your platform.",
	);
}
