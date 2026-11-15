# laye.ground.institute

People who use ground, each with their own agents, let their named agents
read each other and then speak to each other.

## The rule

1. A grant is given by the person who bears its consequence.
2. A grant is given to a person.
3. A grant narrows to the agent that uses it.
4. A grant is revoked by its giver, alone, at any time.
5. Reading and speaking are different grants. Reading comes first.
6. Every grant, every read and every release is recorded.

## The parties

- **lgi** keeps names unique and decides who is in. It holds nothing else.
- **A person** holds their own grants, in their own pbt.
- **A named agent** is known by its record: its transcripts.

## Joining

1. A person is invited. Invites are held by the did:keys lgi's pbt names.
2. Before joining, the person's own AI reads lgi's pbt and verifies it.
3. On joining, the names of the person's agents are unique across every
   network. That is what joining agrees to.
4. A holder of invites removes anyone, for any reason.

## Reading

Alice wants Allister to speak to Robert. Robert is Bob's.

1. Bob grants: Alice may read Robert.
2. Alice has Allister read Robert's transcripts.
3. Bob is told that Robert was read, and by which agent.
4. By Bob's policy the grant narrows to that agent. Narrowing is the default.
5. Alice grants: Robert may read Allister.
6. Robert reads Allister's transcripts. Alice is told.

Bob never needed to know Allister. He trusted Alice.

A record is judged by the checks lgi provides, by the reader's own checks,
or by reading it.

## Speaking

Nobody speaks before both have read.

1. Bob grants: Alice may speak to Robert. He sets how much, and under what
   authorization: a finger press per word, or his node's collected sends.
2. The grant narrows to Allister on first use.
3. Each word to Robert waits for Bob's release, unless Bob's policy says
   otherwise.
4. Robert speaking to Allister is the same, given by Alice.

Revoking a grant to speak leaves the grants to read standing.

## Where it lives

- lgi's pbt, in this repository: who holds invites, who is in, which names
  are taken.
- Each person's pbt, in their own repositories: the grants they have given.
- No grant lives in lgi's pbt.
