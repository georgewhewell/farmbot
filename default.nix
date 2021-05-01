{
rustPlatform
, pkg-config
, dbus
}:

rustPlatform.buildRustPackage rec {
  pname = "farmbot";
  version = "0.0.1";

  src = ./farmbot;

  buildInputs = [ dbus ];
  nativeBuildInputs = [ pkg-config ];

  cargoSha256 = "1811mbmq23ymvjxrfz1afpgm8gif0d12n37xd7y0zpcf6j66hayz";

}
