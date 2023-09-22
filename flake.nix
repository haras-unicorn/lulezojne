{
  description = "Image to ANSI color converter heavily inspired by lule";

  inputs = {
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    {
      nixosModules.default = { pkgs, lib, config, ... }:
        with lib;
        let
          cfg = config.programs.lulezojne;
        in
        {
          options.programs.lulezojne = {
            enable = mkEnableOption (mdDoc "lulezojne");
          };

          config = mkIf cfg.enable {
            environment.systemPackages = [
              self.packages.${pkgs.system}.default
            ];
          };
        };

      homeManagerModules.default = { pkgs, lib, config, ... }:
        with lib;
        let
          cfg = config.programs.lulezojne;
          tomlFormat = pkgs.formats.toml { };
        in
        {
          options.programs.lulezojne = {
            enable = mkEnableOption (mdDoc "lulezojne");

            config =
              mkOption {
                type = tomlFormat.type;
                default = { };
                example = literalExpression ''
                  {
                    plop = [
                      {
                        template = '''
                          {
                            "red": "{{ ansi.main.red }}"
                          }
                        ''';
                        "in" = "~/.config/someprogram/colors.json";
                      }
                    ]
                  }
                '';
                description = ''
                  Lulezojne configuration.
                '';
              };
          };

          config = mkIf cfg.enable
            {
              home.packages = [
                self.packages.${pkgs.system}.default
              ];

              xdg.configFile."lulezojne/config.toml" = mkIf (cfg.config != { }) {
                source = tomlFormat.generate "lulezojne-config" cfg.config;
              };
            };
        };
    } //
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs.outPath {
          config = { allowUnfree = true; };
          inherit system;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            llvmPackages.clangNoLibcxx
            llvmPackages.lldb
            rustc
            cargo
            clippy
            rustfmt
            rust-analyzer
            cargo-edit
          ];
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "lulezojne";
          version = "0.1.0";
          src = self;
          cargoHash = "sha256-xAxR84YrAqikBd2fmVw9IGaec/FAgGtn6hXWDW6Rt3g=";
          meta = {
            description = "Image to ANSI color converter heavily inspired by lule";
            homepage = "https://github.com/haras-unicorn/lulezojne";
            license = pkgs.lib.licenses.mit;
          };
        };
      }
    );
}
