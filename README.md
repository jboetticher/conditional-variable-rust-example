# Conditional Variable in Rust
A condition variable can block a thread without wasting CPU time while waiting for a specific condition to be fulfilled.  

https://doc.rust-lang.org/stable/std/sync/struct.Condvar.html

For example, a consumer thread must wait on the condition of there being items to consume.  

If that didn't help, here's a better example that uses producers and consumers but with real-world objects.  

## Crab Cook Example
Let's say you're trying to cook some crabs! 🦀  

You have one thread prepare the crabs for cooking. You have another thread actually cook the crab.  

In this case, the preparing thread is the producer. The cooking thread is the consumer.  

The cooking thread has a pot that can contain 3 crabs. It is most cost efficient to cook 3 crabs at the same time, so your cooking thread will wait for there to be 3 crabs prepared before attempting to cook any.  

3 crabs being prepared is the condiiton to wait for! So take a look at how main.rs implements these condition variables.  

## Optimizations
The crab cooking is just an example. A friendly user by the name of trentj pointed out that this is closer to a multi-producer-single-consumer problem: 

https://doc.rust-lang.org/std/sync/mpsc/
