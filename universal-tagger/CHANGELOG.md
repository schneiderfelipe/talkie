# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Documentation

 - <csr-id-6c17508fe43de5a16c0035ac56a0233a6bffbfbd/> indicate panics in detect
 - <csr-id-1e9ba0e6ab8171574b28d9d9924289746a8e6d23/> add example
 - <csr-id-efcfd8ad3633d94d81620d73dd0aeac75af872d8/> add docs for Language

### New Features

 - <csr-id-3cbf10ddce03656fcdcf5f8741dd5e1a713234c1/> update public API
 - <csr-id-3b92215996440567d7fed25006c33b1dbc30468b/> give better compile errors
 - <csr-id-91270ec00f59017e7e150d2264e3520affe7d25a/> support lingua as backend as well
 - <csr-id-ddd738b0d076c3698f242f5ab2fefb6a08119521/> start the unicode-segmentation module
 - <csr-id-5236cbbf2eaacf515e4af9779a446b8564b68806/> support some debugging
 - <csr-id-55e30eaf29dc5f8e888f40a344ba80231cbaf3ae/> only test for languages we know
 - <csr-id-a5f21267a1e9b10d3c837a8b899f911c7fbd0f7a/> convert between language types
 - <csr-id-c01649c3c0bf05781a0d87a5a1a40218d7af5661/> add a set of languages as an enum
 - <csr-id-9c836241f57439da1f7be15392a2af1ecbfc87a6/> support reliable language detection through whatlang
 - <csr-id-8e849b377e0a5680f9f2b08330e8f585bc74ad30/> add universal-tagger crate

### Other

 - <csr-id-c1cc1315d60c07ec2d9212fd631b9061ceb72b6c/> put all languages behind features
 - <csr-id-a24bc8e8ef2106cf06b31f227aefd0213b42e6c6/> add optional dependency for lingua
 - <csr-id-0bc36625d3119b2f102319caff05045dd24360d8/> make whatlang optional

### Performance

 - <csr-id-ff96b8a8c9db56a87730be6a7ab3efe452e47009/> inline functions

### Refactor

 - <csr-id-e763277f40d46d7fa3e11df1ae09b0d109c22f26/> produce vector outside function
 - <csr-id-3068a657e4d966b15177a911119547c04180e3de/> use Self

### Test

 - <csr-id-8257942037e1be1f8f2ddcc3d3678a244a8de731/> add a failing test

### New Features (BREAKING)

 - <csr-id-69a6015067ada06498a83b87f63ffa0cf4742957/> prefer lingua if either one can be used
 - <csr-id-82c70e499914e489fbc26240d9d2d85e49f1dffe/> must use result of detect
 - <csr-id-715ca3091d270bc54989b19e9dd96e41c76a951a/> update API
 - <csr-id-d94ae9989aace5dfba95ae13da36c718a3d4c69c/> panic if not enough languages
 - <csr-id-3a756280f90d6144ef580f42a5bb17e2bc0eac29/> support allow and deny

### Refactor (BREAKING)

 - <csr-id-b72110eafaa9561b63cbbc4d4b1827808a4a60c3/> rework the detection system

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 26 commits contributed to the release.
 - 26 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - update public API ([`3cbf10d`](https://github.com/schneiderfelipe/talkie/commit/3cbf10ddce03656fcdcf5f8741dd5e1a713234c1))
    - inline functions ([`ff96b8a`](https://github.com/schneiderfelipe/talkie/commit/ff96b8a8c9db56a87730be6a7ab3efe452e47009))
    - indicate panics in detect ([`6c17508`](https://github.com/schneiderfelipe/talkie/commit/6c17508fe43de5a16c0035ac56a0233a6bffbfbd))
    - prefer lingua if either one can be used ([`69a6015`](https://github.com/schneiderfelipe/talkie/commit/69a6015067ada06498a83b87f63ffa0cf4742957))
    - must use result of detect ([`82c70e4`](https://github.com/schneiderfelipe/talkie/commit/82c70e499914e489fbc26240d9d2d85e49f1dffe))
    - add example ([`1e9ba0e`](https://github.com/schneiderfelipe/talkie/commit/1e9ba0e6ab8171574b28d9d9924289746a8e6d23))
    - update API ([`715ca30`](https://github.com/schneiderfelipe/talkie/commit/715ca3091d270bc54989b19e9dd96e41c76a951a))
    - panic if not enough languages ([`d94ae99`](https://github.com/schneiderfelipe/talkie/commit/d94ae9989aace5dfba95ae13da36c718a3d4c69c))
    - put all languages behind features ([`c1cc131`](https://github.com/schneiderfelipe/talkie/commit/c1cc1315d60c07ec2d9212fd631b9061ceb72b6c))
    - give better compile errors ([`3b92215`](https://github.com/schneiderfelipe/talkie/commit/3b92215996440567d7fed25006c33b1dbc30468b))
    - produce vector outside function ([`e763277`](https://github.com/schneiderfelipe/talkie/commit/e763277f40d46d7fa3e11df1ae09b0d109c22f26))
    - support lingua as backend as well ([`91270ec`](https://github.com/schneiderfelipe/talkie/commit/91270ec00f59017e7e150d2264e3520affe7d25a))
    - add optional dependency for lingua ([`a24bc8e`](https://github.com/schneiderfelipe/talkie/commit/a24bc8e8ef2106cf06b31f227aefd0213b42e6c6))
    - make whatlang optional ([`0bc3662`](https://github.com/schneiderfelipe/talkie/commit/0bc36625d3119b2f102319caff05045dd24360d8))
    - use Self ([`3068a65`](https://github.com/schneiderfelipe/talkie/commit/3068a657e4d966b15177a911119547c04180e3de))
    - start the unicode-segmentation module ([`ddd738b`](https://github.com/schneiderfelipe/talkie/commit/ddd738b0d076c3698f242f5ab2fefb6a08119521))
    - support some debugging ([`5236cbb`](https://github.com/schneiderfelipe/talkie/commit/5236cbbf2eaacf515e4af9779a446b8564b68806))
    - support allow and deny ([`3a75628`](https://github.com/schneiderfelipe/talkie/commit/3a756280f90d6144ef580f42a5bb17e2bc0eac29))
    - rework the detection system ([`b72110e`](https://github.com/schneiderfelipe/talkie/commit/b72110eafaa9561b63cbbc4d4b1827808a4a60c3))
    - add docs for Language ([`efcfd8a`](https://github.com/schneiderfelipe/talkie/commit/efcfd8ad3633d94d81620d73dd0aeac75af872d8))
    - add a failing test ([`8257942`](https://github.com/schneiderfelipe/talkie/commit/8257942037e1be1f8f2ddcc3d3678a244a8de731))
    - only test for languages we know ([`55e30ea`](https://github.com/schneiderfelipe/talkie/commit/55e30eaf29dc5f8e888f40a344ba80231cbaf3ae))
    - convert between language types ([`a5f2126`](https://github.com/schneiderfelipe/talkie/commit/a5f21267a1e9b10d3c837a8b899f911c7fbd0f7a))
    - add a set of languages as an enum ([`c01649c`](https://github.com/schneiderfelipe/talkie/commit/c01649c3c0bf05781a0d87a5a1a40218d7af5661))
    - support reliable language detection through whatlang ([`9c83624`](https://github.com/schneiderfelipe/talkie/commit/9c836241f57439da1f7be15392a2af1ecbfc87a6))
    - add universal-tagger crate ([`8e849b3`](https://github.com/schneiderfelipe/talkie/commit/8e849b377e0a5680f9f2b08330e8f585bc74ad30))
</details>

