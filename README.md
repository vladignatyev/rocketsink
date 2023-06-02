**I created this repository as an illustration to [the question I posted on Rust Lang Community](https://users.rust-lang.org/t/minimal-rocket-app-generates-probably-enormous-amount-of-heap-allocations/90368).**

I wondered how the performance of my Rocket application could be increased and how to eliminate possible performance bottlenecks.
[Rocket](https://rocket.rs/) is a web framework for Rust language that makes it simple to write fast web applications.

# Motivation
What I'm going to create is a fast microservice that accepts a structured data via HTTP requests and seamlessly upload this data into Redis Streams or Kafka. Example use-case would be a web endpoint for uploading events from various mobile devices or other services.

# Why Rocket.rs?
Worth to say that Rocket does pretty well by providing high-level typed abstractions to handle HTTP requests and connections to RDBMS.
The official documentation contains explaining examples. Despite that the current version of the framework is just a 0.5, Rocket has passed the 6 years of active maintaining by Rust community and has a massive feature set purposed for making modern web backends conservative to consumed server resources. Those are mostly the reasons made me choose Rocket as a framework for this task.

# Expectations and outcomes
I **expected to achieve 120K-150K requests/second** on a single 16-core machine with slightly declining as a size of data in request grows due to request deserialization load.

What I get is **60K reqests/second** on my laptop featured with 13th-gen Intel i7 14-core processor with 16GB of RAM.
The performance benchmark is accomplished with `wrk` utility. The full source code of the performance test is available in the `test_run.sh` script. To run test, you will need Redis server running on 6379 port.
