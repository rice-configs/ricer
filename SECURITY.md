<!--
SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
SPDX-License-Identifier: MIT
-->

# Security Policy

## Reporting a Vulnerability

Please feel free to [draft a GitHub advisory][cve-draft], and I will work with
you to disclose and or resolve the issue responsibly.

If this doesn't seem like the right approach or there are questions, please feel
free to reach out to me via the email used in my commits, and in the SPDX
copyright tags at the top of every file in the codebase. Currently, that email
is \<<jasonpena@awkless.com>\>.

## My Public Key

All tagged releases and commits authored by me for Ricer are signed by my GPG
public key \<<jasonpena@awkless.com>\>. You can verify the authenticity of a
tagged release with [git verify tag][git-verify-tag] or a commit with
[git verify commit][git-verify-commit]. Although you may want to verify tags
moreso than commits, because not all commits will be signed.

My public key is avaliable at [keys.openpgp.org][my-pub-key]. The following is
the fingerprint of my public key (spaces added for readability):

```
8A14 E0DF A45D F309 72BA  BA29 A39C 4170 CBB2 3146
```

If you see this fingerprint change, or [git verify tag][git-verify-tag] and/or
[git verify commit][git-verify-commit] tells you that any signed tag is invalid,
then feel free to [draft a GitHub advisory][cve-draft], or send an email to me
at \<<jasonpena@awkless.com>\>.

[cve-draft]: https://github.com/rice-configs/ricer/security/advisories/new
[git-verify-tag]: https://git-scm.com/docs/git-verify-tag
[git-verify-commit]: https://git-scm.com/docs/git-verify-commit
[my-pub-key]: https://keys.openpgp.org/search?q=jasonpena%40awkless.com
