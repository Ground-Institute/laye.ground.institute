// "laye owns identity, laye owns login, laye owns A2A"
// "laye needs to keep being there when qntx is down"
// "ok, so i want C to be the one"

// What C answers itself at laye's own name. Every other request there is
// handed on, as it came, to the laye behind C.
const HELD = ["/laye/administrator/"];

// Whether a request is handed on: it came for laye's name, and C holds no
// answer to it. At any other name C is the peer it always was.
export function handedOn(url, laye) {
  if (!laye || url.hostname !== laye) return false;
  return !HELD.some((held) => url.pathname.startsWith(held));
}
