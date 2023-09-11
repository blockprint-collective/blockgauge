blockgauge
==========

This is a microservice to accompany [blockprint][] and [blockdreamer][].

The aim of the gauge is to measure `blockprint`'s accuracy live using synthetic blocks from
`blockdreamer`. It does this by receiving blocks from `blockdreamer` via `POST /classify`, and then
preparing and sending them to `blockprint` for classification. The classifications are then
compared to the known identities of the nodes, recorded by the gauge and served on the
`GET /accuracy` endpoint.

## Installation

A Docker image is available on the GitHub container registry:

```
docker pull ghcr.io/blockprint-collective/blockgauge
```

Or you can build from source:

```
cargo build --release
```

## Configuration

Blockgauge needs to be pointed at a `blockprint` API server and a Lighthouse node.

```
Measure the accuracy of a blockprint instance using synthetic blocks

Usage: blockgauge [OPTIONS] --lighthouse-url <URL> --blockprint-url <URL>

Options:
      --lighthouse-url <URL>  Lighthouse node to fetch block reward data from
      --blockprint-url <URL>  Blockprint instance to use for classifying blocks
      --listen-address <IP>   Address to listen on [default: 127.0.0.1]
      --port <N>              Port to listen on [default: 8002]
  -h, --help                  Print help
  -V, --version               Print version
```

## API endpoints

- `POST /classify`: accept a JSON `ClassifyRequest` containing blocks to classify.
- `GET /accuracy`: return a JSON `Summary` containing information about the classified blocks.

## Response Structure

See an example response on https://api.blockprint.sigp.io/confusion

[blockprint]: https://github.com/sigp/blockprint
[blockdreamer]: https://github.com/blockprint-collective/blockdreamer
