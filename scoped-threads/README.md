# Scoped Threads

In Rust, threads can be scoped using it. That allows the reference to be used and released after the scope.
When using regular thread, the lifetime of the thread is the lifetime of the program and reference will be passed to move closure and cannot be referenced again in the following code (we can use clone BUT clone is expensive on heap)

Scoped Threads have their lifetime scoped to the closure they are created within, so it can be used without cloning.

We don't have to manually perform .join() in the scoped threads, when the closure ends it will join() it automatically.
