# bifrost

`bifrost` is a an experiment in serverless application development.

The fundamental idea is to allow computation to be split between the client side and the server side without the need to explicitly write server code.

`bifrost` does this using conditional compilation. `bifrost` applications are compiled into two executables:
1. The client-side application itself
2. A `wasm32-wasi` executable in which remote computations are performed

Applications using `bifrost` can implement `Op`s, which are units of computation invoked on the client side, but executed on the server side. Client-side invocations of `Op`s is done via a `Dispatcher`, which makes an RPC call to perform the computation remotely.

On the other end of such RPC calls is `heimdall`, which is a REST service via which `bifrost` remote executables can be run. Remote executables can be uploaded to a running `heimdall` service, and subsequently `heimdall` can be used to perform computations defined in included `Op`s.
