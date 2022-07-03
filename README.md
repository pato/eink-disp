## Eink-disp

Eink-disp provides an abstraction layer for an eink display and a server-side
binary for serving eink bitmaps.

It is indended to be run on either the LAN for local access (or at your own
risk on the internet). The only requirement is that the target host has access
to the internet (to fetch data) and is accessible from the eink-display's
platform.

## Hardware

For my development purposes I am using a [paper.d eink display](https://paperd.ink)
with is a 4.2" eink display with an ESP32.

I am running the server-side binary on a locally accessible server.

## Running 

You can compile using `cargo build --release` and then run the resultant
`server` binary.

Alternatively you can use the provided `build-linux.sh` to use Docker to
compile a linux binary from within MacOS.
