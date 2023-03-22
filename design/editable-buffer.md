
# Design Document for Editable Buffer

Goal: Implement / find some data structure that will allow us to position a cursor into the buffer and make edits efficiently.

Data structures
- rope: seems to be better for large texts

Possible libraries:
- [jumprope](https://github.com/josephg/jumprope-rs): a rust rope implementation
- [ropey](https://github.com/cessen/ropey): another rope implementation

## Resources

- [Rope Science](https://xi-editor.io/xi-editor/docs/rope_science_00.html)
- [Data structures for text sequences](https://www.cs.unm.edu/~crowley/papers/sds.pdf)
- [A Brief Glance at How Various Text Editors Manage Their Textual Data](A Brief Glance at How Various Text Editors Manage Their Textual Data)
