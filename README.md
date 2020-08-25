`tlrepo` provides `ThreadLocalRepo`, a convenient way to share a
`git2::Repository` object between threads.

A standard `git2::Repository` object does not support sharing among threads
without some form of synchronization. `tlrepo::ThreadLocalRepo` provides a
convenient way to reopen the same repository on each thread, caching the opened
repository thread-locally for reuse.

You can create a `ThreadLocalRepo` by calling `ThreadLocalRepo::new`, or by
using the extension trait `tlrepo::RepositoryExt` to call `.thread_local()` on
an existing `git2::Repository`. You can share the `ThreadLocalRepo` across
threads, calling `.get()` to get a `git2::Repository` object to work with.
