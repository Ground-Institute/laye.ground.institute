// node --test src/front.test.js

import assert from "node:assert/strict";
import { test } from "node:test";
import { handedOn } from "./front.js";

const LAYE = "laye.ground.institute";
const at = (href) => new URL(href);

test("at laye's name, C answers who administers it", () => {
  assert.equal(handedOn(at("https://laye.ground.institute/laye/administrator/teranos"), LAYE), false);
});

test("at laye's name, everything else is handed on as it came", () => {
  for (const path of ["/", "/laye/me", "/laye/passkey/signin/begin", "/auth/token", "/laye/invitations", "/health", "/peer"]) {
    assert.equal(handedOn(at(`https://laye.ground.institute${path}`), LAYE), true, path);
  }
});

test("at its own name, C is the peer it always was", () => {
  for (const path of ["/", "/health", "/peer", "/laye/administrator/teranos", "/laye/me"]) {
    assert.equal(handedOn(at(`https://c.sbvh.nl${path}`), LAYE), false, path);
  }
});

test("a C that is told no name for laye hands nothing on", () => {
  assert.equal(handedOn(at("https://laye.ground.institute/laye/me"), undefined), false);
  assert.equal(handedOn(at("https://laye.ground.institute/laye/me"), ""), false);
});
