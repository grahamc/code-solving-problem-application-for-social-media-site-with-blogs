# Minimally Functional HTTP 1.0 Proxy

A threaded proxy with one thread per connection, and thread reuse.

Binds to 127.0.0.1:8000.

## main.rs

 - creates threads, n per backend
 - pushes most of the work to `do_proxy` in `proxy.rs`

## proxy.rs

 - parses the incoming request headers (HttpRequestParser), connects
   and forwards the request headers to the backend
 - streams the body to the backend
 - parses the response headers (HttpResponseParser)
 - returns the response headers to the client
 - streams the response body to the client

 # Notes

  - Does not implement some finer points on the spec (like header
    folding...)

# apache bench

Against 4 Python3 simple http servers, serving a small static file.


    [nix-shell:~]$ ab -t 10 -r -c80  -n 4000 http://127.0.0.1:8000/
    This is ApacheBench, Version 2.3 <$Revision: 655654 $>
    Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
    Licensed to The Apache Software Foundation, http://www.apache.org/

    Benchmarking 127.0.0.1 (be patient)
    Completed 400 requests
    Completed 800 requests
    Completed 1200 requests
    Completed 1600 requests
    Completed 2000 requests
    Completed 2400 requests
    Completed 2800 requests
    Completed 3200 requests
    Completed 3600 requests
    Completed 4000 requests
    Finished 4000 requests


    Server Software:        SimpleHTTP/0.6
    Server Hostname:        127.0.0.1
    Server Port:            8000

    Document Path:          /
    Document Length:        4096 bytes

    Concurrency Level:      80
    Time taken for tests:   0.977 seconds
    Complete requests:      4000
    Failed requests:        0
    Write errors:           0
    Total transferred:      17228000 bytes
    HTML transferred:       16384000 bytes
    Requests per second:    4092.19 [#/sec] (mean)
    Time per request:       19.549 [ms] (mean)
    Time per request:       0.244 [ms] (mean, across all concurrent requests)
    Transfer rate:          17211.99 [Kbytes/sec] received

    Connection Times (ms)
                  min  mean[+/-sd] median   max
    Connect:        0    0   0.1      0       1
    Processing:     1   19   2.2     19      29
    Waiting:        1   19   2.2     19      29
    Total:          2   19   2.1     19      29

    Percentage of the requests served within a certain time (ms)
      50%     19
      66%     20
      75%     20
      80%     21
      90%     21
      95%     22
      98%     23
      99%     24
     100%     29 (longest request)
