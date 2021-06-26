# Sections
- [Sections](#sections)
- [About YAML Format](#about-yaml-format)
- [Validation Rules](#validation-rules)
- [Examples](#examples)
  - [Good Examples](#good-examples)
    - [Example 1](#example-1)
  - [Bad Examples](#bad-examples)
    - [Example 1](#example-1-1)
    - [Example 2](#example-2)
    - [Example 3](#example-3)
    - [Example 4](#example-4)
    - [Example 5](#example-5)

# About YAML Format

YAML (YAML Ain't Markup Language) is a human friendly data format and serialization standard. Read more about YAML format [here](https://yaml.org/).

# Validation Rules

  * YAML must contain a top-level element called `root` which is a *string*, which specifies the entry point node key.
  * YAML must contain a top-level element called `nodes` which is a *hash*, which specifies the map of nodes.
  * `nodes` must contain at least 1 node.
  * `nodes` is a *hash*.
  * Node keys are *strings*.
  * Node values are *hashes*.
    * Node data must contain a `dialogue` key which stores a *string* value.
    * If node data contains a `links` :
      * link values are *array elements*.
      * link names are *strings*.
      * link dialogues are *strings*.
  * **Future ([#3](https://github.com/simbleau/convo/issues/3))** : Orphan nodes must not exist.
  * **Future ([#10](https://github.com/simbleau/convo/issues/10))** : Link keys must reference existing nodes.

# Examples

You can find valid examples also in the [dialogue_files](../dialogue_files/) folder.

## Good Examples

### Example 1
```yaml
---
root: start
nodes:
  start:
    dialogue: "Hello, how are you?"
    links:
      - end: "I'm rudely in a hurry."
  end:
    dialogue: "Ok, let's talk some other time."
```

Why is this a **good** example?

The example adheres to [YAML format](#about-yaml-format) and follows [validation rules](#validation-rules).

Namely,
  * YAML contains a top-level element called `root` which specifies the entry point called `start`.
  * YAML contain a top-level element called `nodes`.
  * `nodes` contains at least 1 node (It contains 2).
  * All nodes contain a `dialogue` key.
  * Orphan nodes do not exist (all links can be visited).

## Bad Examples

### Example 1

```yaml
---
nodes:
  root:
    dialogue: "It's a bad day."
```
Why is this a **bad** example?

  * YAML does not contain a top-level element called `root`.

### Example 2

```yaml
---
root: root
nodes:
  start:
    dialogue: "It's a bad day."
```
Why is this a **bad** example?

  * YAML contains a top-level element called `root` which specifies the entry point to be a node called `root`. This will not work because there is only one node called `start`.

### Example 3

```yaml
---
root: start
nodes:
```
Why is this a **bad** example?

  * `nodes` does not contains at least 1 node.
  * `start` does not exist.

### Example 4

```yaml
---
root: start
nodes:
  start:
    links:
      - end: "I'm rudely in a hurry."
  end:
    dialogue: "Ok, let's talk some other time."
```

Why is this a **bad** example?

  * `start` does not contain dialogue.

### Example 5

```yaml
---
root: start
nodes:
  start:
    dialogue: "Hello, how are you?"
  end:
    dialogue: "Ok, let's talk some other time."
```
Why is this a **bad** example?

  * `end` is an orphan node. It is not traversible.