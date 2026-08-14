# vaulet-registry

The reserved claim vocabulary for [Vaulet](https://vaulet.id) (ADR 0046).

A credential type says what it carries in its own words. Two shops declare
`points`, meaning different things, and nothing on the wire tells them apart —
so a claim name is either **reserved**, meaning one thing everywhere, or it is
**somebody's own** and carries their identifier:

```
birthdate                          reserved, universal
org.iso.18013.5.1:document_number  reserved, defined by ISO
th.co.codefin:loyalty_tier         Codefin's own
```

**Reserved is what this repository lists, not what a name looks like.** A
standard namespace and a private one are the same shape — `org.iso.18013.5.1`
and `th.co.codefin` are both reverse-DNS — so nothing can be told from the
string. That is why the list is public, and why it is a repository rather than
an endpoint: a vocabulary one vendor controls is one nobody else should build
on.

## What is here

| file | what it holds |
| --- | --- |
| `claims/universal.json` | words usable bare, spelled as OpenID Connect Core §5.1 spells them |
| `claims/namespaces.json` | reserved namespaces — **pointers** to vocabularies others publish, and **definitions** for the few nobody has written down |
| `claims/refused.json` | identifier prefixes nobody may be allocated |

**A pointer does not copy the words.** `org.iso.18013.5.1` is ISO's, and a copy
of ISO's list here is a copy that goes stale without anybody noticing. What is
recorded is that the namespace is reserved and who defines it.

## What is not here

**Nobody's private claims.** They are theirs, offered to no one as a shared
word, and a list of every customer's internal vocabulary would be maintained
for ever and read by nobody. An identifier is unique because the issuer that
allocated it says so.

## Adding a word

Open a pull request adding it to `claims/universal.json` or to a `definition`
namespace, with a kind and one sentence of meaning — **all three**, because a
name agreed without a kind lets one issuer write a date and another write Thai
text into the same field and still not be readable together.

A word is reserved when it means the same thing to everybody. `account` in a
bank and `account` in a game are not the same word; those belong under a
namespace, not bare.

**Reserving a word does not make its value true.** Anybody may put `birthdate`
on a card they sign themselves. Whether it is believed comes from who assessed
it and who vouches for them.
