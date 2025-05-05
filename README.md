# roll

![GitHub License](https://img.shields.io/github/license/peasee/roll) ![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/peasee/roll/ci.yml)

An open source, high performance, polling platform - built in Rust.

Project name suggested by [@farooqkz](https://github.com/farooqkz) - "Rust + Poll" :heart:

# Table of Contents
1. [Performance](#performance)
2. [V2 Backend](#backend)
    - [Why not AWS?](#aws)
3. [Getting Started](#getting-started)
    - [Config format](#config)

# Performance<a name="performance"></a>

Benchmarked using `npx autocannon -c 100 -d 10 http://localhost:3081/api/poll/nisWgKF-Fzx-FAN1O7gMP/options`, indicating maximum possible performance for users who have already loaded the poll interface:

```
Running 10s test @ http://localhost:3081/api/poll/nisWgKF-Fzx-FAN1O7gMP/options
100 connections


┌─────────┬──────┬──────┬───────┬───────┬─────────┬─────────┬────────┐
│ Stat    │ 2.5% │ 50%  │ 97.5% │ 99%   │ Avg     │ Stdev   │ Max    │
├─────────┼──────┼──────┼───────┼───────┼─────────┼─────────┼────────┤
│ Latency │ 4 ms │ 5 ms │ 8 ms  │ 10 ms │ 5.14 ms │ 2.15 ms │ 117 ms │
└─────────┴──────┴──────┴───────┴───────┴─────────┴─────────┴────────┘
┌───────────┬─────────┬─────────┬─────────┬─────────┬────────┬────────┬─────────┐
│ Stat      │ 1%      │ 2.5%    │ 50%     │ 97.5%   │ Avg    │ Stdev  │ Min     │
├───────────┼─────────┼─────────┼─────────┼─────────┼────────┼────────┼─────────┤
│ Req/Sec   │ 15,527  │ 15,527  │ 18,671  │ 19,119  │ 18,366 │ 968.2  │ 15,524  │
├───────────┼─────────┼─────────┼─────────┼─────────┼────────┼────────┼─────────┤
│ Bytes/Sec │ 1.94 MB │ 1.94 MB │ 2.33 MB │ 2.39 MB │ 2.3 MB │ 121 kB │ 1.94 MB │
└───────────┴─────────┴─────────┴─────────┴─────────┴────────┴────────┴─────────┘

Req/Bytes counts sampled once per second.
# of samples: 10

184k requests in 10.07s, 23 MB read
```

This benchmark uses the same `autocannon` command, but targeting the full `http://localhost:3081/poll/wxJLlx5BYuTSrujpmSAQT` route:

```
Running 10s test @ http://localhost:3081/poll/wxJLlx5BYuTSrujpmSAQT
100 connections


┌─────────┬──────┬──────┬───────┬───────┬────────┬─────────┬───────┐
│ Stat    │ 2.5% │ 50%  │ 97.5% │ 99%   │ Avg    │ Stdev   │ Max   │
├─────────┼──────┼──────┼───────┼───────┼────────┼─────────┼───────┤
│ Latency │ 5 ms │ 5 ms │ 8 ms  │ 11 ms │ 5.3 ms │ 1.73 ms │ 71 ms │
└─────────┴──────┴──────┴───────┴───────┴────────┴─────────┴───────┘
┌───────────┬─────────┬─────────┬─────────┬─────────┬──────────┬─────────┬─────────┐
│ Stat      │ 1%      │ 2.5%    │ 50%     │ 97.5%   │ Avg      │ Stdev   │ Min     │
├───────────┼─────────┼─────────┼─────────┼─────────┼──────────┼─────────┼─────────┤
│ Req/Sec   │ 15,455  │ 15,455  │ 17,823  │ 18,607  │ 17,536.8 │ 973.4   │ 15,454  │
├───────────┼─────────┼─────────┼─────────┼─────────┼──────────┼─────────┼─────────┤
│ Bytes/Sec │ 16.8 MB │ 16.8 MB │ 19.3 MB │ 20.2 MB │ 19 MB    │ 1.06 MB │ 16.8 MB │
└───────────┴─────────┴─────────┴─────────┴─────────┴──────────┴─────────┴─────────┘

Req/Bytes counts sampled once per second.
# of samples: 10

175k requests in 10.08s, 190 MB read
```

And finally, `autocannon` on the API options route but with `--workers 4`:

```
Running 10s test @ http://localhost:3081/api/poll/wxJLlx5BYuTSrujpmSAQT/options
100 connections
4 workers

-
┌─────────┬──────┬──────┬───────┬──────┬─────────┬────────┬───────┐
│ Stat    │ 2.5% │ 50%  │ 97.5% │ 99%  │ Avg     │ Stdev  │ Max   │
├─────────┼──────┼──────┼───────┼──────┼─────────┼────────┼───────┤
│ Latency │ 1 ms │ 3 ms │ 6 ms  │ 6 ms │ 3.53 ms │ 1.6 ms │ 47 ms │
└─────────┴──────┴──────┴───────┴──────┴─────────┴────────┴───────┘
┌───────────┬─────────┬─────────┬─────────┬─────────┬──────────┬──────────┬─────────┐
│ Stat      │ 1%      │ 2.5%    │ 50%     │ 97.5%   │ Avg      │ Stdev    │ Min     │
├───────────┼─────────┼─────────┼─────────┼─────────┼──────────┼──────────┼─────────┤
│ Req/Sec   │ 16,879  │ 16,879  │ 22,527  │ 32,223  │ 23,681.6 │ 4,646.31 │ 16,868  │
├───────────┼─────────┼─────────┼─────────┼─────────┼──────────┼──────────┼─────────┤
│ Bytes/Sec │ 2.11 MB │ 2.11 MB │ 2.82 MB │ 4.03 MB │ 2.96 MB  │ 581 kB   │ 2.11 MB │
└───────────┴─────────┴─────────┴─────────┴─────────┴──────────┴──────────┴─────────┘

Req/Bytes counts sampled once per second.
# of samples: 41

254k requests in 10.04s, 31.7 MB read
```

There's some overhead to running the server within Docker, and probably some limitations in container-host transfer limiting the absolute maximum performance here. These benchmarks were all run on a server with 2x Intel Xeon E5-2450 v2 CPUs for 32 threads total, and 96GB of memory.

# V2 Backend<a name="backend"></a>

I've recently gone on an adventure to update the backend of this polling service to be a Rust backend, using `axum` as a web server.
The service now also uses a Docker Compose file to make it easy to spin up your own instance of this service.

Why did I migrate to Rust as a backend?

1. It's much, _much_ faster than NodeJS. Using `autocannon` with 8 worker threads on a 2x Intel Xeon E5-2450 v2 CPUs with 32 threads total achieved around 140,000 requests per second. A single worker thread achieved 18,000 requests per second. Granted, this includes the removal of writes/reads to redis, and does not take into account other overheads like network stacks, but impressive nonetheless.
2. Rust is now one of my primary languages, although I've never used it as a web server. I thought it would be a fun exercise to try updating the backend.
3. All benefits (and cons) of the entire Rust language - type and memory safety, traits, etc.

I also removed redis as a requirement, and replaced it with a simple in-memory `BTreeMap` because chances are you don't care about the results of your poll for any longer than 30 minutes and if the server restarts while you're running your poll you'll probably just remake the poll.
Removing redis also helped drastically increase the requests per second throughput.

## But why not AWS?<a name="aws"></a>

"Oh, you want scaling performance to the moon? Why didn't you just make this in AWS?"

I'm not rich. That's all. Making this kind of application scale using something like DynamoDB/SQS/Lambda/API Gateway at a high load rate of 15 million votes per month (about 100 polls per day with 5000 votes) has a comparative cost of about $200 USD per month.

Sure, it'll scale to essentially infinity loads, but this is only an averaged rate of about 350 requests / second, maybe peaking at the creation of polls around 1-2k requests / second.

A $15 USD per month VPS will give you 10-20x this capacity, at a fraction of the cost.

# Getting Started<a name="getting_started"></a>

Create your `.env` file following the example provided. Optionally, create a reCAPTCHA key and set it. If you do not provide a reCAPTCHA key, captcha checks are not performed during poll creation or voting.

Once your `.env` file is created, start the service like:

```
docker compose up -d
```

And go to town! Setup any other proxies or forwarding to your desire afterwards.

## Config Format<a name="config"></a>

This service uses a `.env` file as its configuration file, and expects a full `.env` file to look like:

```
API_HOST=http://localhost:3081
PORT=3081
RECAPTCHA_SITE_KEY=<recaptcha site key>
RECAPTCHA_SECRET_KEY=<recaptcha secret key>
HOST=<recaptcha host>
```

* `API_HOST` is the base URI that your API server would be available on, and is baked into the frontend client when it is built for all web requests.
* `PORT` is the port the service runs on
* `RECAPTCHA_SITE_KEY` is your reCAPTCHA site key, and is optional. Exclude this field entirely if you don't provide one
* `RECAPTCHA_SECRET_KEY` is your reCAPTCHA secret key, and is optional. Exclude this field entirely if you don't provide one
* `HOST` is the host domain used for reCAPTCHA request verification, and should match the hostname that your frontend is served from. Improperly setting this may result in `Invalid token` errors from reCAPTCHA verification.
