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
    commands: CommandType[4];
}

enum CommandType {
    Spawn {
        player: PlayerInfo;
    },
    Despawn {
        uuid: Uuid;
    }
}
```

## Features

* Structures.
* Enums.
* Type aliases.
* Dynamic and fixed-size arrays.
* 128-bit integer types.

See the <a href="#Comparison">Comparison</a> section to see how NeatProto compares to other popular formats.

## Targets

* Rust
    - Works with Serde.
    - Compiler is available as a crate, so you can easily integrate it with your Rust project using a
      build script.

## Comparison

|                       |                                NeatProto                                | Protocol Buffers | Flatbuffers | Apache Thrift |
|-----------------------|:-----------------------------------------------------------------------:|:----------------:|:-----------:|:-------------:|
| Structures            |                                    ✅                                    |        ✅         |      ✅      |       ✅       |
| Enums                 |                                    ✅                                    |        ✅         |      ✅      |       ✅       |
| Type aliases          |                                    ✅                                    |        ❌         |      ❌      |       ✅       |
| Tagged unions         |                                    ✅                                    |        ✅         |      ✅      |       ✅       |
| Namespaces / packages | 🚧<br/>[Tracking issue](https://github.com/adamwych/neatproto/issues/2) |        ✅         |      ✅      |       ✅       |
| RPC / services        |                                    ❌                                    |        ✅         |      ✅      |       ✅       |
| Comments              | 🚧<br/>[Tracking issue](https://github.com/adamwych/neatproto/issues/3) |        🟡        |     🟡      |      🟡       |
| 128-bit integers      |                                    ✅                                    |        ❌         |      ❌      |       ❌       |
| Embedded protocol*    |                                    ❌                                    |        ✅         |      ❌      |       ❌       |

✅ = Implemented.
🟡 = Implemented, but less advanced.
🚧 = In Progress / Planned.
❌ = Not Implemented.

\* - Some other formats allow you to store the protocol as part of serialized data - NeatProto requires you to know
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
