{
  description = "A terminal slideshow tool";

  inputs = {
    flakebox = {
      url = "github:rustshop/flakebox?rev=47f7fe6aa0951ee984800f3e339c0c54f4a4e862";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, flake-utils, flakebox }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        projectName = "presenterm";

        flakeboxLib = flakebox.lib.${system} {
          config = {
            github.ci.buildOutputs = [ ".#ci.${projectName}" ];
          };
        };

        buildPaths = [
          "build.rs"
          "Cargo.toml"
          "Cargo.lock"
          ".cargo"
          "src"
          "themes"
          "bat"
          "executors.yaml"
        ];

        buildSrc = flakeboxLib.filterSubPaths {
          root = builtins.path {
            name = projectName;
            path = ./.;
          };
          paths = buildPaths;
        };

        multiBuild =
          (flakeboxLib.craneMultiBuild { }) (craneLib':
            let
              craneLib = (craneLib'.overrideArgs {
                pname = projectName;
                src = buildSrc;
                nativeBuildInputs = [ ];
              });
            in
            {
              ${projectName} = craneLib.buildPackage { };
            });
      in
      {
        packages.default = multiBuild.${projectName};

        legacyPackages = multiBuild;

        devShells = flakeboxLib.mkShells { };
      }
    );
}
