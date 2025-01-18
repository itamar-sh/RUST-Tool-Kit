# Rust: Asynchronous Programming with Tokio

There isn't a builtin async with rust, we have crates like tokio that give us async abilities.

` async ` - Function or method that works in parallel.
` await ` - A mark of a method's call. The mark tells we must wait for the method to finish before continue.
` #[tokio::main] ` - A wrapper that mark the a method as the main method to start async running with tokio.

## Spawning tasks example
We create a comparison here between async and non-async code.
` std::thread ` is a builtin way of tells the current thread to do something.

` tokio::task::spawn_blocking ` Executes a blocking (synchronous) function in a dedicated thread pool for blocking tasks, separate from the runtime's async task scheduler.

This prevents the blocking operation from interfering with the main async task scheduler, which is crucial for maintaining the responsiveness of the Tokio runtime. Adds overhead for thread pool switching

` tokio::spawn(async_call(id)) ` Async function, spawns an asynchronous task that runs on the Tokio runtime's async task scheduler. Lightweight, optimized for async.

Both ` tokio::task::spawn_blocking ` and ` tokio::spawn(async_call(id)) ` runs in parallel.
but the level of parallelism and how they execute depend on the context of the Tokio runtime:

spawn_blocking is a separate Thread Pool up to 512 threads, however Async Runtime Scheduler uses mostly the number of CPU cores in the machine. Both will create and use threads for each call.

We will use spawn_blocking if we afraid of thread starvation when one thread is heavily then the other so we don't want the thread to be part of the Async Runtime Scheduler.

## Asynchronous Primitives - like Mutex
Established parameters that are used to synchronize the running of asynchronous threads.

Like Mutex, Barrier and Semaphore.

Notify is used to send notifications between threads.

Barrier waits for some amount of threads to arrive before allowing anyone to continue.

Mutex allows for only one to access the data - a lock.

Arc is an object which is a reference to Asynchronous Primitives - passing him to the threads allows them to request access to the data behind the primitive like mutex with ` arc_variable.lock() `.

Semaphore let only x users get access to the resource simultanously. Arc of Semaphore uses different function to request access ` arc_variable.acquire() `.

## Notify
Acts both as transmitter and a receiver.

Create thread-safe clones of your notify instance and pass them to the threads. Using Arc as before.

Wait to receive message by ` notified() ` and send message by ` notify_one() ` or  ` notify_waiters() `

## Barrier
Barrier has ` is_leader() ` function that help us know if the first to arrive to the barrier has passed the barrier.

Before we create new threads we wait for a notify signal from the previos Barrier leader to tell us the old barrier is open. Then new threads will be created and will wait one second to let the previous Barrier time to close.

## Reader Writers Lock
We have in Rust a dedicated Lock called ` RwLock ` that give access to only one writer and many readers.

We didn't dig to the case where there is writers starvation by many readers, I assume once a writer wants to write new readers can't start read until the writer start to write and finish.

## Channels
We didn't cover this section yet. But we should know that notifications are not the only way to send data between threads, there are another ways like Channels.