# The workflow that deploys C, emitted to .github/workflows/c.yml and never edited by hand:
#   nix eval --json --file ci/c.nix | jq . > .github/workflows/c.yml
#
# "oh, c needs to move to private actually"
#
# A push to main deploys c/ to C, the Worker c on Cloudflare. The laye it runs
# is the commit c/Cargo.toml names.
{
  name = "c";
  on.push.branches = [ "main" ];
  concurrency = {
    group = "c";
    cancel-in-progress = false;
  };
  jobs.deploy = {
    runs-on = "ubuntu-latest";
    steps = [
      { uses = "actions/checkout@v5"; }
      # C is Rust built for wasm32: wrangler's build runs worker-build (c/wrangler.jsonc).
      {
        uses = "dtolnay/rust-toolchain@stable";
        "with".targets = "wasm32-unknown-unknown";
      }
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
