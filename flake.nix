{
  description = "A Rust implementation of fzf using the Nucleo crate";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }: 
    let version = "0.0.6"; in 
    flake-utils.lib.eachDefaultSystem (system: 
      with import nixpkgs { system = system; };
      rec {
        packages = {
          default = rustPlatform.buildRustPackage {
            pname = "rfz";
  	    version = version;
  	    src = ./.;
  	    # cargoBuildFlags = "-p app";
  	    cargoLock = {
  	      lockFile = ./Cargo.lock;
  	    };
  	    nativeBuildInputs = [ pkg-config ];
  	    PKG_CONFIG_PATH = "${openssl.dev}/lib/pkgconfig";
  	  };
        };
        devShells = {
          default = mkShell {
            name = "rfz-dev";
  
  	    packages = [
              cargo
  	      clippy
              rustc
  	    ];
  	  };
        };
	apps = {
	  default = {
	    type = "app";
	    program = "${packages.default}/bin/rfz";
	  };
	};
      }
    );
}
