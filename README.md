# Errol

![Errol](/static/errol.jpg)

- [Errol - Pottermore](https://www.pottermore.com/explore-the-story/errol)

[GitHub Push Events](https://developer.github.com/v3/activity/events/types/#pushevent)
[Gitlab Push Events](https://docs.gitlab.com/ee/user/project/integrations/webhooks.html#push-events)

[Diesel ORM](http://diesel.rs/guides/getting-started/)
[Rust Project Layout](https://doc.rust-lang.org/cargo/reference/manifest.html#the-project-layout)
[Rocket Web Framework](https://rocket.rs/)
[Lettre: For Sending E-mails](https://github.com/lettre/lettre)
[Gitlab Rust Client](https://gitlab.kitware.com/utils/rust-gitlab)
[GitHub Rust Client](https://github.com/mgattozzi/github-rs)

## Summary

Errol allows you to subscribe to new issues, reviews, and commits that match
any fields you're interested in.

## Motivation

This was originally inspired by some modifications I did to git-multimail, to
only receive e-mails for commits based on certain paths modified. Soon people
were asking for support for additional fields in the push.

During my research into other solutions I discovered [Herald](herald) and
[mention-bot](mention-bot), which allow much smarter and more powerful rules.

Some use cases we'd like to support would be:

- Send me an e-mail for
  - every commit to a specific branch in a specific repo
  - every commit by a certain person
  - every commit that edits a certain file
  - every commit that edits a file that matches a regex

The expected short term outcome is being able to receive e-mails based on rules,
and the longer term outcome is supporting things like mention-bot or other ways
to subscribe folks to issues or reviews on GitHub and Gitlab.

## Guide-level explanation

For implementation-oriented RFCs (e.g. for compiler internals), this section should focus on how compiler contributors should think about the change, and give examples of its concrete impact. For policy RFCs, this section should provide an example-driven introduction to the policy, and explain its impact in concrete terms.

## Reference-level explanation

This is the technical portion of the RFC. Explain the design in sufficient detail that:

The section should return to the examples given in the previous section, and explain more fully how the detailed proposal makes those examples work.

## Drawbacks

## Rationale and alternatives

## Prior art

Git bundles a few options for e-mailing people whenever a push happens to repo, e.g.:
[post-receive-email](https://github.com/git/git/blob/master/contrib/hooks/post-receive-email)
[git-multimail](https://github.com/git-multimail/git-multimail) (also part of [git/contrib](https://github.com/git/git/blob/master/contrib/hooks/multimail/git_multimail.py))

Facebook's [mention-bot](mention-bot) uses static rules and analyses Git blame information
to suggest folks to review a pull request on GitHub.

Gitlab's [Email on Push](gitlab-email-on-push) sends you a push for
every commit to a repo.

[Herald](herald) is definitely the most powerful project I've seen so far, when it
comes to custom rules to let someone know about changes to Phabricator Tasks, Commits,
and more Phabricator Objects, and potentially trigger other custom work.

[HeraldAdapter.php](https://sourcegraph.com/github.com/phacility/phabricator@master/-/blob/src/applications/herald/adapter/HeraldAdapter.php)

Other semi-related stuff
[GitHub CODEOWNERS](https://help.github.com/articles/about-codeowners/)
[Gitlab CODEOWNERS](https://docs.gitlab.com/ee/user/project/code_owners.html)

## Unresolved questions

[gitlab-email-on-push]: https://docs.gitlab.com/ee/user/project/integrations/emails_on_push.html
[herald]: https://secure.phabricator.com/book/phabricator/article/herald/
[mention-bot]: https://github.com/facebookarchive/mention-bot
