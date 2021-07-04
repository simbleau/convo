# Sections
- [Sections](#sections)
- [About YAML Format](#about-yaml-format)
- [File extension](#file-extension)
- [Validation Rules](#validation-rules)
  - [Exporting](#exporting)
  - [Importing, Parsing](#importing-parsing)
- [Examples](#examples)
  - [Good Examples](#good-examples)
    - [Example 1](#example-1)
  - [Bad Examples](#bad-examples)
    - [Example 1](#example-1-1)
    - [Example 2](#example-2)
    - [Example 3](#example-3)
    - [Example 4](#example-4)
    - [Example 5](#example-5)
    - [Example 6](#example-6)
    - [Example 7](#example-7)

# About YAML Format

YAML (YAML Ain't Markup Language) is a human friendly data format and serialization standard. Read more about YAML format [here](https://yaml.org/).

# File extension

The conventional file extension for convo files is `*.convo.yml`.

# Validation Rules

You can expect an error to be thrown when trying to export or import `*.convo.yml` files if the following validation rules have not been satisfied. You will receive verbose error information on failure, but for comprehension, the rules are listed below in full.

## Exporting
  * The `Tree` must have a root key set
  * The `Tree#nodes` must contain at least 1 node.
  * **Future ([#10](https://github.com/simbleau/convo/issues/10))** : Links must all reference existing nodes.
  * **Future ([#3](https://github.com/simbleau/convo/issues/3))** : All nodes must be reachable; Nodes must be the root element or linked to by a parent.

## Importing, Parsing
  * YAML must contain a top-level element called `root` which is a *string*, which specifies the entry point node key.
  * YAML must contain a top-level element called `nodes` which is a *hash*, which specifies the map of nodes.
  * `nodes` must contain at least 1 node.
  * `nodes` is a *hash*.
  * Node keys are *strings*.
  * Node values are *hashes*.
    * Node must contain a `dialogue` key and/or a `links` key.
    * If node data contains a `dialogue` :
      * node dialogue value is a *string* value.
    * If node data contains a `links` :
      * node link values are *array elements*.
      * node link keys are *strings*.
      * node link values are *strings*.
      * **Future ([#10](https://github.com/simbleau/convo/issues/10))** : Link keys must all reference existing nodes.
  * **Future ([#3](https://github.com/simbleau/convo/issues/3))** : All nodes must be reachable; Nodes must be the root element or linked to by a parent.

# Examples

You can find valid examples in the [dialogue_files](../dialogue_files/) folder.

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

  * `end` is an orphan node. It is not linked to and therefore not part of the hierarchy.

### Example 6

```yaml
---
root: start
nodes:
  fork:
    dialogue: "I link to all nodes!"
    links:
      - start: "I link to start"
      - end: "I link to end"
      - fork: "I even link to myself!"
  start:
    dialogue: "Hello, how are you?"
  end:
    dialogue: "Ok, let's talk some other time."
```
Why is this a **bad** example?

  * `end` and `fork` are unreachable nodes because the root node is `start`. 

### Example 7

```yaml
---
root: start
nodes:
  start:
    dialogue: "I am the start node"
    links:
      - start: "I am valid and link to myself"
      - not_a_real_key: "I do not link to a valid key"
```
Why is this a **bad** example?

  * `not_a_real_key` is not a node that exists, e.g. invalid reference key. 