# kala-repr

## 0 Indirection

1 word

undefined, null, false, true, i28

Slot = Value

Not captured, ephemeral, not escaping

## 1 Indirection

3 words

Object, Array, Function

*Slot = Value

TODO: need to be decomposed and inlined

Not captured, ephemeral, not escaping

## 2 Indirection

3 words

Reference

All types captured and needs to be heap allocated

