# trafficserver-ffi
Raw Rust bindings for the
[Apache Traffic Server](https://trafficserver.apache.org/)
plugin API.

## Building the bindings

Building the Traffic Server API bindings uses
[traffic_layout](https://docs.trafficserver.apache.org/en/latest/appendices/command-line/traffic_layout.en.html)
to locate the TrafficServer installation paths, so if you need to generate
bindings for a specific version of Traffic Server, make sure that the correct
[traffic_layout](https://docs.trafficserver.apache.org/en/latest/appendices/command-line/traffic_layout.en.html)
program is first in your `PATH`.

For example:
```
trafficserver-ffi jpeach$ PATH=/opt/ats/bin:$PATH cargo build
   Compiling dtoa v0.4.2
   Compiling regex v0.2.9
   Compiling quote v0.3.15
   Compiling termcolor v0.3.5
   Compiling serde v1.0.30
   Compiling utf8-ranges v1.0.0
   Compiling peeking_take_while v0.1.2
   Compiling ucd-util v0.1.1
   Compiling libc v0.2.39
   Compiling bitflags v1.0.1
   Compiling libloading v0.4.3
   Compiling bindgen v0.33.1
   Compiling quick-error v1.2.1
   Compiling void v1.0.2
   Compiling num-traits v0.2.1
   Compiling lazy_static v1.0.0
   Compiling cfg-if v0.1.2
   Compiling itoa v0.3.4
   Compiling strsim v0.7.0
   Compiling glob v0.2.11
   Compiling unicode-width v0.1.4
   Compiling ansi_term v0.11.0
   Compiling unicode-xid v0.1.0
   Compiling vec_map v0.8.0
   Compiling regex-syntax v0.5.2
   Compiling unreachable v1.0.0
   Compiling log v0.4.1
   Compiling memchr v1.0.2
   Compiling memchr v2.0.1
   Compiling atty v0.2.8
   Compiling which v1.0.5
   Compiling humantime v1.1.1
   Compiling textwrap v0.9.0
   Compiling proc-macro2 v0.2.3
   Compiling thread_local v0.3.5
   Compiling clang-sys v0.21.2
   Compiling nom v3.2.1
   Compiling aho-corasick v0.6.4
   Compiling clap v2.31.1
   Compiling quote v0.4.2
   Compiling syn v0.12.14
   Compiling cexpr v0.2.3
   Compiling serde_json v1.0.11
   Compiling serde_derive_internals v0.20.0
   Compiling env_logger v0.5.5
   Compiling serde_derive v1.0.30
   Compiling trafficserver-ffi v7.1.2 (file:///Users/jpeach/src/trafficserver-ffi)
    Finished dev [unoptimized + debuginfo] target(s) in 34.43 secs
```
