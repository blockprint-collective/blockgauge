blockgauge
==========

This is a microservice to accompany [blockprint][] and [blockdreamer][].

The aim of the gauge is measure `blockprint`'s accuracy live using synthetic blocks from
`blockdreamer`. It does this by receiving blocks from `blockdreamer` via `POST /classify`, and then
preparing and sending them to `blockprint` for classification. The classifications are then
compared to the known identities of the nodes, recorded by the gauge and served on the
`GET /accuracy` endpoint.

Example accuracy data is:

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
[blockdreamer]: https://github.com/michaelsproul/blockdreamer
