# Repro

This fails:

```
cargo build
```

(some files are created, but in my 'real' code, it fails too catastrophically to continue.)

This works

```
protoc -I=protobufs --python_out=dest protobufs/uses.proto protobufs/imports/thing.proto
```
