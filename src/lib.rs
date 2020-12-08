//! `tlrepo` provides `ThreadLocalRepo`, a convenient way to share a `git2::Repository` object
//! between threads.
//!
//! A standard `git2::Repository` object does not support sharing among threads without some form
//! of synchronization. `tlrepo::ThreadLocalRepo` provides a convenient way to reopen the same
//! repository on each thread, caching the opened repository thread-locally for reuse.
//!
//! You can create a `ThreadLocalRepo` by calling `ThreadLocalRepo::new`, or by using the extension
//! trait `tlrepo::RepositoryExt` to call `.thread_local()` on an existing `git2::Repository`.
#![deny(missing_docs)]
use std::path::PathBuf;

use git2::Repository;
use thread_local::ThreadLocal;

/// An object providing a thread-local copy of a `git2::Repository` for each thread.
pub struct ThreadLocalRepo {
    tl: ThreadLocal<Repository>,
    path: PathBuf,
}

impl ThreadLocalRepo {
    /// Create a `ThreadLocalRepo` that opens the repository at the specified path on each thread.
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            tl: ThreadLocal::new(),
        }
    }

    /// Get the `git2::Repository` for this thread. Returns an error if the open fails.
    ///
    /// Note that the cache of thread-local objects never gets pruned. If you're running on a
    /// long-running thread or a thread pool, call this method. If you're running on a short-lived
    /// thread, call `get_uncached` instead.
    pub fn get(&self) -> Result<&Repository, git2::Error> {
        self.tl.get_or_try(|| Repository::open(&self.path))
    }

    /// Get a new `git2::Repository`, and don't save it in the thread-local cache. Returns an error
    /// if the open fails.
    ///
    /// The cache of thread-local objects never gets pruned. If, over the lifetime of your process,
    /// you run an unbounded number of threads that call `get` and subsequently exit, the
    /// thread-local cache will grow without bound. In such threads, use `get_uncached` to open a
    /// repository that won't get cached.
    pub fn get_uncached(&self) -> Result<Repository, git2::Error> {
        Repository::open(&self.path)
    }
}

/// Extension trait for `git2::Repository`, to create a `ThreadLocalRepo` with the path to the
/// repository.
pub trait RepositoryExt {
    /// Get a `ThreadLocalRepo` that reopens the path to this `git2::Repository` on each thread.
    fn thread_local(&self) -> ThreadLocalRepo;
}

impl RepositoryExt for Repository {
    fn thread_local(&self) -> ThreadLocalRepo {
        ThreadLocalRepo::new(self.path().to_path_buf())
    }
}
