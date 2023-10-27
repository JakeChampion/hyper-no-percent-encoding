Making requests with characters outside the allowed set without percent-encoding those characters results in hyper not running the service handler and instead returning a response of HTTP/1.1 400 Bad Request.

Steps to reproduce:
```
> git clone git@github.com:JakeChampion/hyper-no-percent-encoding.git
> cargo run --release
Listening on http://127.0.0.1:3000

# in another terminal window we will first make a request that works and then the one which fails
❯ curl "http://127.0.0.1:3000/" -v
*   Trying 127.0.0.1:3000...
* Connected to 127.0.0.1 (127.0.0.1) port 3000
> GET / HTTP/1.1
> Host: 127.0.0.1:3000
> User-Agent: curl/8.4.0
> Accept: */*
>
< HTTP/1.1 200 OK
< content-length: 21
< date: Fri, 27 Oct 2023 10:29:06 GMT
<
* Connection #0 to host 127.0.0.1 left intact
You requested path: /

❯ curl "http://127.0.0.1:3000/<" -v
*   Trying 127.0.0.1:3000...
* Connected to 127.0.0.1 (127.0.0.1) port 3000
> GET /< HTTP/1.1
> Host: 127.0.0.1:3000
> User-Agent: curl/8.4.0
> Accept: */*
>
< HTTP/1.1 400 Bad Request
< content-length: 0
< date: Fri, 27 Oct 2023 10:29:51 GMT
<
* Connection #0 to host 127.0.0.1 left intact
```

In your first terminal you will see output for requests that our handler has handler, and you will see it has not printed anything for the `curl "http://127.0.0.1:3000/<" -v` request, because it never got called for that request.

```
❯ cargo run --release
   Compiling toki-meow v0.1.0 (/Users/jakechampion/temp/toki-meow)
    Finished release [optimized] target(s) in 1.63s
     Running `target/release/toki-meow`
Listening on http://127.0.0.1:3000
echo fn is running for a request to /
```
