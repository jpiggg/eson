![Eson](docs/assets/headline.png)

# Eson

Eson is a Rust and WebAssembly serializer for JavaScript values. It provides a compact, JavaScript-oriented text format that can represent several values plain JSON cannot preserve, including `undefined`, `NaN`, infinities, and BigInts.

The project was previously called Braser. The package and public API are now named `eson`.

## Status

Eson is experimental. The Node.js target is usable for controlled workloads, but the format and error behavior are not yet a drop-in replacement for JSON. Read the limitations below before using it for untrusted or cross-language data.

## Installation

For a published package:

```sh
npm install eson
```

To build the Node.js package from this repository:

```sh
wasm-pack build -t nodejs -d pkg-node --release
```

The generated package is written to `pkg-node/` and exposes the CommonJS entry point used by Node.js.

## Node.js API

The public API has two functions:

- `eson.stringify(value)` returns an Eson string.
- `eson.parse(text)` returns the JavaScript object represented by an Eson document.

```js
const eson = require('eson');

const source = {
  name: 'Ada',
  missing: undefined,
  notANumber: NaN,
  positiveInfinity: Infinity,
  largeInteger: 123456789n,
};

const text = eson.stringify(source);
const restored = eson.parse(text);

console.log(text);
console.log(restored.missing === undefined); // true
console.log(Number.isNaN(restored.notANumber)); // true
console.log(restored.largeInteger); // 123456789n
```

The parser currently requires a top-level object:

```js
eson.parse('{message:"hello", count:42}')
// { message: 'hello', count: 42 }
```

Arrays and nested objects are supported as values, but a document cannot itself be a top-level array or scalar.

## Eson format

Eson uses a JSON-like syntax with a few extensions:

```text
{
  message: "hello",
  enabled: true,
  missing: undefined,
  empty: null,
  notANumber: NaN,
  positiveInfinity: Infinity,
  negativeInfinity: -Infinity,
  integer: 123456789n,
  values: [1, 2, 3]
}
```

Object keys may be simple unquoted identifiers made from ASCII letters. Quoted keys are available for more complex names. Single- and double-quoted string values, comments, optional commas, and whitespace are accepted by the grammar.

## Values

| Value | Parse | Stringify | Notes |
|---|---:|---:|---|
| String | Yes | Yes | Escape decoding is currently not identical to JSON. |
| Number | Yes | Yes | The grammar accepts ordinary integers and decimals, not exponent notation. |
| BigInt | Yes | Yes | Test values carefully; large values may lose precision during stringify. |
| `Infinity`, `-Infinity` | Yes | Yes | Preserved by Eson. |
| `NaN` | Yes | Yes | Preserved by Eson. |
| `undefined` | Yes | Yes | Preserved as a property value. |
| Boolean | Yes | Yes |  |
| `null` | Yes | Yes |  |
| Object | Yes | Yes | Plain objects are supported. |
| Array | Yes | Yes | Arrays are supported as nested values. |
| Date, Map, Set, RegExp | No | Not safely | These object values may silently serialize as `{}`. |
| Function, Symbol | No | Not safely | These values can cause a WebAssembly trap. |
| Circular references | No | No | Stringifying a cycle can overflow the stack. |

The table describes the current implementation, not an idealized feature list. Unsupported-value behavior should be treated as an API limitation until typed errors are implemented.

## Eson compared with JSON

Plain JSON does not preserve several JavaScript values:

```js
JSON.parse(JSON.stringify({ missing: undefined, value: NaN }));
// { value: null }
```

Eson can preserve those values in its supported object and value paths:

```js
const value = { missing: undefined, value: NaN };
const restored = eson.parse(eson.stringify(value));

Object.hasOwn(restored, 'missing'); // true
restored.missing === undefined; // true
Number.isNaN(restored.value); // true
```

This richer value model is Eson's primary reason to exist. It is not currently a performance advantage.

## Performance

The benchmark suite in [`eson-test`](../eson-test/bench/BENCHMARKS_EXPLAINED.md) compares Eson with native JSON and Node's binary `v8.serialize`/`deserialize` implementation.

The measured results show that, for ordinary JSON-like API payloads, Eson is substantially slower than both alternatives. In the supplied run:

- JSON parsing was approximately 3 to 4 ns per byte on larger size cases.
- Eson parsing was approximately 90 to 96 ns per byte.
- JSON stringifying was approximately 3 ns per byte.
- Eson stringifying was approximately 76 to 82 ns per byte.
- V8 binary serialization was close to JSON on these workloads.

These figures are workload- and machine-specific. Run the correctness gate before timing and compare results from the same machine and benchmark run. The benchmark suite also reports noise, memory deltas, format sizes, cold-start behavior, and unsupported-value facts.

## When to use Eson

Eson may be useful when all of the following are true:

- the producer and consumer are JavaScript environments using the same Eson behavior;
- preserving values such as `undefined`, `NaN`, or BigInt matters;
- human-readable text is useful; and
- the payload size and throughput are acceptable for the application.

Use JSON when interoperability, mature error handling, streaming tools, or throughput are more important. Use V8 serialization when both endpoints are Node.js and a binary format is acceptable.

## Limitations

- Parsing and stringifying are currently all-at-once operations; there is no streaming API.
- The parser requires a top-level object.
- Invalid input can result in a WebAssembly trap rather than a normal catchable JavaScript error.
- Escape sequences are not decoded exactly like JSON, so escape-heavy inputs need special care.
- Some unsupported objects silently become `{}`.
- Functions and symbols are not portable data and may trap during stringify.
- BigInt behavior above the tested range must be verified by the caller.
- The Node.js benchmark is not a browser benchmark; browser startup and glue behavior differ.

## Development

Run the Rust tests with:

```sh
cargo test
```

Build the Node.js WebAssembly package with:

```sh
wasm-pack build -t nodejs -d pkg-node --release
```

The benchmark project provides the Node smoke test and benchmark commands. See [`eson-test`](../eson-test/bench/README.md) for how to run them and [`BENCHMARKS_EXPLAINED.md`](../eson-test/bench/BENCHMARKS_EXPLAINED.md) for how to interpret the results.

## License

No license is currently declared in this repository.