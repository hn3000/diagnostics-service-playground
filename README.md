# Diagnostics Server Playground

We attempted to implement the project outlined below and wanted to first try 
out some architectural options.

We started using actix-web and synchronous writing of log files and then
attempted to change the code so it would write the logged messages using
async-std versions of the std library, but ran into trouble: when the received
message is larger than a small limit the log file is filled with copies of that
message until we shut down the process, it never stopped spinning the CPU in 
the minutes we spent trying to figure out what had just happened.
(We did not attempt to find the exact limit at which this occurs -- the smaller
examples in test.http work fine, the larger ones trigger the behaviour.)

The sqlx variant also gave us trouble because we could not figure out how to
get Utc::now() converted into a database timestamp. 
We used Sqlite, for easier testing. Maybe it works better with Postgres?

The current versions are very straight-forward and completely lack:
- locking against multiple writers on one file
- rate limiting as sketched below
- some validation of data or sensible limits

## Building

We built everything with cargo. Docker files are also provided with
a pretty okay multi-step build that can probably still be optimized.
We attempt to install dependencies first and copy and compile actual source 
afterwards in order to profit from docker's caching of intermediate build
results.

## The Problem

We want to implement a web-service to allow some frontend code to
log diagnostics messages.

Examples:

- time for initialization of the page
- unexpected error messages
- access time for backend services
- ...

Features (i.e. MVP):

- simple structured payload:
  - message identification code
  - client ID (optional, should be random but relatively stable to identify client-specific problems)
  - severity
  - payload (text / stacktrace)

- rate limited access per IP / random client ID


Later:

- CSRF Tokens for authorization
- additional data fields
- switchable logging targets
- size limits for log files

Not our concern:

- automatic deletion after X days
(i.e. can probably be solved by log rotation tools)

- monitoring
(i.e. can be solved using other tools)


Rationale:

- rate limiting is okay, because I only need qualitative data about the
  state of my clients, i.e. if the rate limit engages, I probably have
  a problem that needs to be solved, the data lost because of the
  rate limit should not add valuable information

- client ID is necessary to see whether 1 client or 100 clients have a problem
- the ID can rotate daily / weekly because that would not prevent me from
  counting the clients per day / per week

- CSRF later because the rate limit should offer sufficient protection at first

- switchable logging targets not at first, lets see what we get first,
  implement a database later, if necessary or useful (i.e. YAGNI)
