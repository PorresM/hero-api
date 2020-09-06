Hero API
---

## Installation

```
cargo install
```

## Usage


```
docker-compose up
```

```
cargo run
```

### Diesel

Generate new migration file :
```
diesel migration generate <name>
```

Apply migrations :
```
diesel migration run
```

Redo migrations :
```
diesel migration redo
```

Cancel migrations :
```
diesel migration revert
```