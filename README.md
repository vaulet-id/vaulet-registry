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

## Versions

Tagged, and the number means something (ADR 0047):

| | |
| --- | --- |
| **major** | a word's meaning changes, or one is withdrawn |
| **minor** | words are added |
| **patch** | wording of a meaning, a source, a comment |

Consumers pin a tag. A deployment can then say which vocabulary it was
enforcing when it signed something, which is the one question a register is
asked when there is a dispute — and the difference between "new words" and "a
word means something else now" is legible without reading a diff.

**Nothing is deleted.** A word that was wrong is deprecated with a
`superseded_by`: credentials carrying it cannot be recalled.

## What gets a word in

One of two things, and neither is taste:

- **a citation to a published standard** — it enters as a pointer, or as a
  definition spelled to match, and the citation is recorded in the entry
- **two independent issuers who would use it** — different organisations, not
  two products of one. A word only one party writes belongs in that party's own
  namespace, where it costs nobody anything

Who decides is this repository's owner, today. Saying so is better than
implying a process that does not exist; the bar above is what makes a refusal
about the word rather than about the person.

## Four layers, in order

A name is resolved by asking, in this order, and the first answer wins:

1. **OpenID Connect Core / SD-JWT VC** — `birthdate`, `given_name`, `email`.
   Not our vocabulary; this repository cites it and records the kinds so a
   value can be checked.
2. **Somebody else's standard** — `org.iso.18013.5.1` (mDL),
   `eu.europa.ec.eudi.pid.1` (EUDI). Recorded as a pointer, never copied.
3. **This register** — the few words nobody else has written down, and the
   aliases for spellings that predate all of it.
4. **An organisation's own namespace** — `th.co.codefin:loyalty_tier`. Not
   listed here, and never will be.

**Layers 1 to 3 all carry their words here**, so a console can offer them.
Layers 1 and 2 are copies and say so — `authoritative: false`, with a `source`.
Where a copy and the standard disagree, the standard wins, which is why a
namespace recorded as a pointer accepts a word this file does not list: a copy
running behind is far likelier than an issuer being wrong.

**Some names are refused outright.** `iss`, `exp`, `cnf`, `vct` and the rest of
the registered JWT and SD-JWT VC fields belong to the signature, not to what is
being said — an attribute called `exp` is written beside the expiry a verifier
reads. Prefixes that would read as somebody else's vocabulary are refused as
identifiers for the same reason.

**Which layer answered is worth knowing.** A verifier can implement layer 1 and
2 once and read every issuer in the world; layer 3 works everywhere this
software runs; layer 4 means something only if you know who issued the card.

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
