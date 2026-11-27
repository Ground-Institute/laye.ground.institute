// "laye owns identity, laye owns login, laye owns A2A"
// "laye needs to keep being there when qntx is down"
// "ok, so i want C to be the one"
// "can't we have it work in a way that we dont need to build the same parser twice?"

// 「[Fable 5.2] Every request at laye's own name is handed on, as it came, to the laye」
// 「[Fable 5.2] behind C. Who administers laye is laye's to say, from what ground says」
// 「[Fable 5.2] of it (Ground-Institute/laye, governed_by): C reads no pbt.」

// 「[Fable 5.2] Whether a request is handed on: it came for laye's name. At any other」
// 「[Fable 5.2] name C is the peer it always was.」
export function handedOn(url, laye) {
  return !!laye && url.hostname === laye;
}
