# Probe

Probe is an encoding format deisgned for portability and schemaless serialization. Probe is a subset of CBOR(RFC 8949).

## Motivation

Cosmos ecosystem is centered around Protobuf, a schema-based serialization protocol with extensible message format. Problems with protobuf is that it requires a schema, so it is hard to make it compatible with JS territory, and it cannot handle opaque object as first class object. CBOR is a JSON-based serialization format that is mainly influenced by Messagepack. CBOR and messagepack are both schemaless, binary format, but the difference is that CBOR is optimized for compact implementation instead of compact serialization - with the major type tag information, the implementation can easily switch-jump into appropriate code point.

## Specification

Probe takes only the deterministic part of CBOR, meaning that it does not include:
- Floating point support
- Non-deterministic encoding on various integer encoding size
- etc etc

Here are the major types from the [CBOR specification](https://www.rfc-editor.org/rfc/rfc8949.html#name-specification-of-the-cbor-e). The exact implementation for these types follows the CBOR determinism restrictions.

### Major type 0: Unsigned Integer

Encoding for uints. Maximum 2^64-1. Additional information rule follows the specification, except only the most compact form is allowed.

### Major type 1: Negative Integer

Encoding for negative ints. Maximum -2^64. Additional information rule follows the specification, except only the most compact form is allowed.

### Major type 2: Bytestring

Encoding for arbitrary bytestrings. Additional information rule follows the specifcation, except for the indefinite-length support.

### Major type 3: UTF-8 string

Encoding for human readable string. Additional information rule follows the specifcation, except for the indefinite-length support.

### Major type 4: Array

Encoding for heterogeneous array. Additional information rule follows the specification, except for the indefinite-length support.

### Major type 5: Map

Encoding for json-style map. 

### Major type 6: Tag

Encoding for single-byte tag on arbitrary data item. 

### Major type 7: Simple values

Encoding for simple values. Additional information represents the fixed set of values, one of true, false, null, undefined. Floating point values are NOT SUPPORTED.