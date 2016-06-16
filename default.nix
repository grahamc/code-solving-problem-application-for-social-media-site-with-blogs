{ unstable ? import ../../.nix-defexpr/channels/unstable {},
  pkgs ? import <nixpkgs> {}
}:
let
    simpleServer = port: (pkgs.stdenv.mkDerivation rec {
      name = "simpleServer-${toString port}";

      serverscript = ''
        #!${pkgs.bash}/bin/bash
        cd $out
        ${pkgs.python3}/bin/python3 -m http.server ${toString port}
      '';
      buildCommand = ''
        mkdir $out
        echo "${serverscript}" > $out/start
        chmod +x $out/start
        echo "<h1>${toString port}</h1>" > $out/index.html
      '';
    });
in
{
  test = unstable.stdenv.mkDerivation {
    name = "rust-test";

    buildInputs = with unstable; [
      cargo
      rustfmt
      python35
      python35Packages.flake8
    ];

    shellHook = with unstable; ''
      set -e

      pushd http-proxy
      for i in `find . -name '*.rs'`; do
        echo "Rustfmt: $i";
        ${rustfmt}/bin/rustfmt $i
      done

      cargo test
      popd

      pushd log-parser
      for i in `find . -name '*.py'`; do
        echo "flake8: $i"
        ${python35Packages.flake8}/bin/flake8 $i
      done
      python3 -m unittest discover
      popd

      set +e
    '';
  };

  shell = unstable.stdenv.mkDerivation {
    name = "rust-env";

    buildInputs = with unstable; [
      rustc
      cargo
      rustfmt
      which
    ];

    shellHook = with unstable; ''
      echo "${rustc.out}"
      echo "${rustc.doc}"
      echo "${cargo}"
      echo "${rustfmt}"
    '';
  };

  proxy-backends = pkgs.stdenv.mkDerivation rec {
    name = "http-proxy-backends";


    honchofile = pkgs.writeText "Procfile" ''
      web8001: ${simpleServer 8001}/start
      web8002: ${simpleServer 8002}/start
      web8003: ${simpleServer 8003}/start
      web8004: ${simpleServer 8004}/start
    '';

    shellHook = ''
      exec ${pkgs.honcho}/bin/honcho -f ${honchofile} start
    '';
  };
}
