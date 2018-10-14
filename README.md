# Git Multimail Webhooks

[Style guideline for Go packages - rakyll](https://rakyll.org/style-packages/)
[Standard Go Project Layout](https://github.com/golang-standards/project-layout)

[net/smtp's Sendmail](https://golang.org/pkg/net/smtp/#SendMail)
[Mailgun client](https://github.com/mailgun/mailgun-go)

[Push Events](https://docs.gitlab.com/ee/user/project/integrations/webhooks.html#push-events)

[go-gitlab](https://github.com/xanzy/go-gitlab)

## Summary

One paragraph explanation of the feature.

## Motivation

Why are we doing this? What use cases does it support? What is the expected outcome?

## Guide-level explanation

For implementation-oriented RFCs (e.g. for compiler internals), this section should focus on how compiler contributors should think about the change, and give examples of its concrete impact. For policy RFCs, this section should provide an example-driven introduction to the policy, and explain its impact in concrete terms.

## Reference-level explanation

This is the technical portion of the RFC. Explain the design in sufficient detail that:

The section should return to the examples given in the previous section, and explain more fully how the detailed proposal makes those examples work.

## Drawbacks

## Rationale and alternatives

## Prior art

Git bundles a few options for e-mailing people whenever a push happens to repo, e.g.:
[post-receive-email](https://github.com/git/git/blob/master/contrib/hooks/post-receive-email) in git
[git-multimail](https://github.com/git-multimail/git-multimail) (also part of [git/contrib](https://github.com/git/git/blob/master/contrib/hooks/multimail/git_multimail.py))

GitHub Related
Facebook's [mention-bot](https://github.com/facebookarchive/mention-bot)
Mention bot mentions potential reviewers on pull requests, using git-blame information, and configuration.

Gitlab Related
Gitlab's [Email on Push](https://docs.gitlab.com/ee/user/project/integrations/emails_on_push.html) sends you a push for
every commit to a repo.

[Herald](https://secure.phabricator.com/book/phabricator/article/herald/)
[HeraldAdapter.php](https://sourcegraph.com/github.com/phacility/phabricator@master/-/blob/src/applications/herald/adapter/HeraldAdapter.php)

Other semi-related stuff
[GitHub CODEOWNERS](https://help.github.com/articles/about-codeowners/)
[Gitlab CODEOWNERS](https://docs.gitlab.com/ee/user/project/code_owners.html)

## Unresolved questions
