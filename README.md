## PH Image server build in Rust

> See [Makefile](Makefile) to get list of commands

To run simple load testing you could use [k6](https://k6.io/) with predefined script:

```bash
k6 run load-testing-k6-script.js -u 1000 -d 1m
```

My results on `Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz`:

```bash
checks.........................: 100.00% ✓ 380403      ✗ 0     
     data_received..................: 14 MB   237 kB/s
     data_sent......................: 36 MB   589 kB/s
     http_req_blocked...............: avg=200.26µs min=0s      med=1µs      max=39.64ms  p(90)=2µs      p(95)=2µs     
     http_req_connecting............: avg=197.25µs min=0s      med=0s       max=38.65ms  p(90)=0s       p(95)=0s      
     http_req_duration..............: avg=474.75ms min=19.64ms med=454.15ms max=922.08ms p(90)=515.76ms p(95)=648.6ms 
       { expected_response:true }...: avg=474.75ms min=19.64ms med=454.15ms max=922.08ms p(90)=515.76ms p(95)=648.6ms 
     http_req_failed................: 0.00%   ✓ 0           ✗ 126801
     http_req_receiving.............: avg=16.96µs  min=10µs    med=15µs     max=854µs    p(90)=21µs     p(95)=31µs    
     http_req_sending...............: avg=27.89µs  min=4µs     med=8µs      max=10.97ms  p(90)=10µs     p(95)=13µs    
     http_req_tls_handshaking.......: avg=0s       min=0s      med=0s       max=0s       p(90)=0s       p(95)=0s      
     http_req_waiting...............: avg=474.7ms  min=18.57ms med=454.13ms max=922.06ms p(90)=515.73ms p(95)=648.56ms
     http_reqs......................: 126801  2096.365214/s
     iteration_duration.............: avg=474.98ms min=20.72ms med=454.2ms  max=922.14ms p(90)=515.84ms p(95)=648.69ms
     iterations.....................: 126801  2096.365214/s
     vus............................: 1000    min=1000      max=1000
     vus_max........................: 1000    min=1000      max=1000
```