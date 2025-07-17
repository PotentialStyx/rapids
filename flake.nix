{
	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

		rust-overlay = {
			url = "github:oxalica/rust-overlay";
			inputs.nixpkgs.follows = "nixpkgs";
		};

		flake-utils.url = "github:numtide/flake-utils";
	};

	outputs = {
		self,
		nixpkgs,
		rust-overlay,
		flake-utils,
	}:
		flake-utils.lib.eachDefaultSystem (system: let
				overlays = [(import rust-overlay)];
				pkgs =
					import nixpkgs {
						inherit system overlays;
					};

				rustToolchain =
					pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
			in {
				devShells.default =
					pkgs.mkShell {
						buildInputs = with pkgs; [
							rustToolchain
							cargo-watch
							bun
						];

						# Environment variables
						RUST_LOG = "info";
						RUSTDOCFLAGS = "--default-theme=ayu";

						RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
					};
			});
}
