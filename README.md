# NeatProto

NeatProto is an interface description language.
It allows you to design an interface and automatically generate corresponding code for: Rust (... more coming soon!).

### Why would I need it?

Imagine you have a client application written in C#, and a server application written in Rust.
You will probably want to send some messages between those two, and that means that you need
a way to define the structure that will be sent over the network in both, C# and Rust.

The easiest way would be to maintain two implementations and manually synchronize them whenever
you make any change, but that's very inefficient and error-prone. Instead, you can simply write a NeatProto file and
then generate corresponding C# and Rust code automatically.

### What does a proto file look like?

The syntax was inspired by modern programming languages, so it should be very familiar to most programmers.

```
alias Uuid = string;

struct PlayerInfo {
    uuid: Uuid;
    name: string;
    healthPoints: float;
}

enum CommandType {
    Spawn = 1,
    Despawn = 2,
}
```

## Features

* Structures.
* Enums.
* Type aliases.
* 128-bit integer types.

See the <a href="#Comparison">Comparison</a> section to see how NeatProto compares to other popular formats.

## Targets

* Rust
    - NeatProto compiler is available as a crate, so you can easily integrate it with your Rust project using a
      build script.

## Comparison

|                     | NeatProto | Protocol Buffers | Flatbuffers | Apache Thrift |
|---------------------|:---------:|:----------------:|:-----------:|:-------------:|
| Structures          |     ✅     |        ✅         |      ✅      |       ✅       |
| Enums               |     ✅     |        ✅         |      ✅      |       ✅       |
| Type aliases        |     ✅     |        ❌         |      ❌      |       ✅       |
| 1:1 binary format*  |    🚧     |        ❌         |      ❌      |       ❌       |
| 128-bit integers    |     ✅     |        ❌         |      ❌      |       ❌       |
| Embedded protocol** |     ❌     |        ✅         |      ❌      |       ❌       |

✅ = Implemented.
🚧 = In Progress / Planned.
❌ = Not Implemented.

\* - All other formats include some metadata or padding in serialized binary data - NeatProto does not. \
\** - Some other formats allow you to store the protocol as part of serialized data - NeatProto requires you to know
the structure up front in order to know how to deserialize it.

## Grammar

```abnf
identifier      = ALPHA *(ALPHA / DIGIT / "_")
literal         = identifier / DIGIT

block           = "{" *block-node "}"
block-node      = *alias *structure-field
root-block      = *block-node 
alias           = "alias" SP identifier *SP "=" *SP identifier *SP ";"
structure       = "struct" SP identifier *SP "{" *structure-field *SP "}"
structure-field = identifier *SP ":" *SP identifier *SP ";"
enum            = "enum" SP identifier *SP "{" [identifier *(["=" *SP literal] *SP "," *SP identifier) [","] *SP] "}"
```
