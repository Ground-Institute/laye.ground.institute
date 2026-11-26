// "use standard normal software and packages so we have to most default boring sentry setup"

// "i prefer the shape of OTEL."

// 「[Opus 5.5] C's Worker and Durable Object, wrapped by @sentry/cloudflare as its
// docs show: withSentry on a plain handler, instrumentDurableObjectWithSentry on a
// plain DurableObject class. worker-build exports its classes and their instances
// as Proxies, which Sentry's wrappers cannot set their context on, so the Rust
// Peer is held inside a plain class rather than wrapped itself.」

import * as Sentry from "@sentry/cloudflare";
import { DurableObject } from "cloudflare:workers";
import { Peer as RustPeer } from "../build/index.js";

const options = (env) => ({
  dsn: env.SENTRY_DSN,
  environment: "c-stack",
  tracesSampleRate: 1.0,
  enableLogs: true,
  sendDefaultPii: false,
});

class HeldPeer extends DurableObject {
  constructor(ctx, env) {
    super(ctx, env);
    this.peer = new RustPeer(ctx, env);
  }

  fetch(request) {
    return this.peer.fetch(request);
  }
}

export const Peer = Sentry.instrumentDurableObjectWithSentry(options, HeldPeer);

export default Sentry.withSentry(options, {
  async fetch(request, env) {
    const peer = env.PEER.get(env.PEER.idFromName("c"));
    if (new URL(request.url).pathname === "/health") {
      const said = await (await peer.fetch("https://c/peer")).json();
      return Response.json({ c: "up", peer: said });
    }
    return peer.fetch(request);
  },
});
