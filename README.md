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

### `POST /classify`

This API endpoint is for submitting blocks for classification. They will be recorded
in-memory by blockgauge, which then serves accuracy data on `/accuracy` (see below). If you are
using `blockdreamer`, it automatically sends data in `blockgauge`'s format when using the setting
`extra_data = true`.

If you would like to make your own requests to this endpoint, the data structure is:

```
{
    "names": [string],
    "labels": [string],
    "blocks": [BeaconBlock]
}
```

For example:

```json
{
    "names": ["lighthouse-node-1", "teku-node-1"],
    "labels": ["Lighthouse", "Teku"],
    "blocks": [
        {
            "slot": "123",
            "proposer_index":
        }
}
```

The contents of the `BeaconBlock` can be mostly garbage, blockprint only really cares about the
contained attestations. However, all `BeaconBlock` fields must be present.

This corresponds to the `ClassifyRequest` struct in blockdreamer's code.

Response:

- On success: HTTP 200 OK with JSON data from Lighthouse's `POST /lighthouse/analysis/block_rewards` API.
  You can save this data on disk for training a `blockprint` classifier. Blockdreamer comes with a
  configuration option to do this for you (`results_dir = "/path/"`).
- On failure: error status code.

### `GET /accuracy`

Return a JSON `Summary` (see code) containing information about the classified blocks.

## Response Structure

See an example response on https://api.blockprint.sigp.io/confusion

[blockprint]: https://github.com/sigp/blockprint
[blockdreamer]: https://github.com/blockprint-collective/blockdreamer
