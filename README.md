# PDFCraft Studio

An open-source, sovereign PDF document viewer, editor, and form processor built in pure Rust, powered by the **Martensite** GPU-accelerated retained-mode GUI engine.

![PDFCraft Studio on Martensite](brag/demo.gif)

## Architecture

- **`crates/ui-martensite`**: Sovereign retained-mode PDF viewer UI with page navigation strip, zoom fit modes, and geometric annotation anchors.
- **`crates/engine`**: ISO 32000-compliant PDF parser, vector glyph renderer, and digital signature verifier.

## Legal & Compliance Notice

PDFCraft is an independent open-source document processor. It is not affiliated with Adobe Inc. Adobe, Acrobat, and PDF are trademarks of Adobe Inc. The Portable Document Format is an open international standard (ISO 32000-1 / ISO 32000-2).

## License

Dual-licensed under MIT OR Apache-2.0.
