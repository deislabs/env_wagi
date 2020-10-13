# env_wagi: Example WAGI program

_WAGI is the easiest way to get started doing cloud-side WASM web apps._

WAGI (Web Assembly Gateway Interface) is an adaptation of CGI for WebAssembly WASI modules. This example uses the [WAGI server](https://github.com/deislabs/wagi).

This repo contains a Rust implementation of a WAGI program that shows off the basic WAGI features.

## Building

You can test the output of this command using `cargo run`.

However, when building it for WAGI, you need to compile this as Web Assembly with WASI support:

```
$ cargo build --target wasm32-wasi --release
```

(We recommend using `--release` because it considerably reduces the size.)

The resulting binary will be written to `target/wasm32-wasi/release/hello_wagi.wasm`.

## The `modules.toml` Settings

For this module to work, your `module.toml` should look like this:

```toml
[[module]]
# Mount this to example.com/env
route = "/env"
# Point to the directory that 'cargo build' wrote the WASM file
module = "/path/to/env_wagi/target/wasm32-wasi/release/env_wagi.wasm"
# Mount the source code of this repo
volumes = { "/" = "/path/to/env_wagi" }
# Set an environment variable
environment.TEST_NAME = "test value"
```

Note that the `volumes` mounts this source code repo to the `/` of the WASM runtime. We use this to list files, and also to read the contents of the `LICENSE.txt` file.

### Using Curl to Execute

Here is an example using `curl` to access `localhost:3000` and also override the `HOST` header. The `-vvv` flag sets Curl's verbosity level.

```console
$ curl -vvv -H "HOST:foo.example.com" localhost:3000/env?greet=matt\&foo=bar
*   Trying 127.0.0.1...
* TCP_NODELAY set
* Connected to localhost (127.0.0.1) port 3000 (#0)
> GET /env?greet=matt&foo=bar HTTP/1.1
> Host:foo.example.com
> User-Agent: curl/7.64.1
> Accept: */*
>
< HTTP/1.1 200 OK
< x-foo-header: Bar
< content-type: text/plain
< content-length: 1902
< date: Tue, 13 Oct 2020 21:39:05 GMT
<
### Arguments ###
arg: /env
arg: greet=matt
arg: foo=bar

### Env Vars ###
REMOTE_ADDR = 127.0.0.1
X_MATCHED_ROUTE = /env
HTTP_HOST = foo.example.com
SERVER_PORT = 80
SCRIPT_NAME = /Users/technosophos/Code/Rust/env_wagi/target/wasm32-wasi/release/env_wagi.wasm
CONTENT_LENGTH = 0
CONTENT_TYPE =
TEST_NAME = test value
SERVER_SOFTWARE = WAGI/1
REMOTE_USER =
GATEWAY_INTERFACE = CGI/1.1
SERVER_NAME = foo.example.com
HTTP_USER_AGENT = curl/7.64.1
AUTH_TYPE =
PATH_TRANSLATED = /env
PATH_INFO = /env
HTTP_ACCEPT = */*
SERVER_PROTOCOL = http
REQUEST_METHOD = GET
REMOTE_HOST = localhost
X_FULL_URL = http://foo.example.com/env?greet=matt&foo=bar
QUERY_STRING = greet=matt&foo=bar

### STDIN ###

### Files ###
Cargo.toml
target
Cargo.lock
README.md
.gitignore
.git
LICENSE.txt
src

### Read LICENSE.txt ###
 The MIT License (MIT)

Copyright (c) Microsoft Corporation. All rights reserved.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* Connection #0 to host localhost left intact
* Closing connection 0
```