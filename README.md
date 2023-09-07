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

## Example Response

Example accuracy data from `/accuracy` is:

```json
{
  "Lighthouse": {
    "num_blocks": 53,
    "num_correct": 53,
    "misclassifications": {}
  },
  "Nimbus": {
    "num_blocks": 53,
    "num_correct": 47,
    "misclassifications": {
      "nimbus-subscribe-none": {
        "Teku": 6
      }
    }
  }
}
```

[blockprint]: https://github.com/sigp/blockprint
[blockdreamer]: https://github.com/blockprint-collective/blockdreamer
