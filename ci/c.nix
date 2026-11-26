# The workflow that checks and deploys C, emitted to .github/workflows/c.yml and never edited by hand:
#   nix eval --json --file ci/c.nix | jq . > .github/workflows/c.yml
#
# "oh, c needs to move to private actually"
# "make ci also run wasm tests and clippy and checks"
#
# 「[Fable 5.2] A pull request and a push to main check c/: it compiles, clippy says nothing,」
# 「[Fable 5.2] and its tests pass, all as wasm32, as C runs. A push to main then deploys c/」
# 「[Fable 5.2] to C, the Worker c on Cloudflare. The laye it runs is the commit c/Cargo.toml names.」
let
  checkout = { uses = "actions/checkout@v5"; };
  # C is Rust built for wasm32: wrangler's build runs worker-build (c/wrangler.jsonc).
  rust = components: {
    uses = "dtolnay/rust-toolchain@stable";
    "with" = { targets = "wasm32-unknown-unknown"; } // components;
  };
  inC = name: run: {
    inherit name run;
    working-directory = "c";
  };
in
{
  name = "c";
  on = {
    push.branches = [ "main" ];
    pull_request = { };
  };
  jobs.check = {
    runs-on = "ubuntu-latest";
    steps = [
      checkout
      (rust { components = "clippy"; })
      (inC "c/ compiles as wasm32" "cargo check --target wasm32-unknown-unknown --all-targets")
      (inC "clippy says nothing" "cargo clippy --target wasm32-unknown-unknown --all-targets -- -D warnings")
      {
        uses = "taiki-e/install-action@v2";
        "with".tool = "wasm-pack";
      }
      # 「[Fable 5.2] wasm-pack fetches the wasm-bindgen c/Cargo.lock names, and runs the tests under node.」
      (inC "its tests pass as wasm32" "wasm-pack test --node")
      # 「[Fable 5.2] What C hands on at laye's own name, and what it answers itself (c/src/front.js).」
      (inC "what C hands on is what it should" "node --test src/front.test.js")
    ];
  };
  jobs.deploy = {
    needs = [ "check" ];
    "if" = "github.event_name == 'push'";
    runs-on = "ubuntu-latest";
    concurrency = {
      group = "c";
      cancel-in-progress = false;
    };
    steps = [
      checkout
      (rust { })
      {
        name = "Deploy c/ to C";
        uses = "cloudflare/wrangler-action@v3";
        "with" = {
          apiToken = "\${{ secrets.CLOUDFLARE_API_TOKEN }}";
          accountId = "1a473b8ef17b2da06c81cfddb5da691a";
          wranglerVersion = "4.156.0";
          workingDirectory = "c";
        };
      }
    ];
  };
}
