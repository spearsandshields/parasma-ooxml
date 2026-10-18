# ooxml

> **This repository is a fork.** It is the [Parasma](https://github.com/spearsandshields/parasma)
> document viewer's fork of [pterror/ooxml](https://github.com/pterror/ooxml) and is not
> affiliated with or endorsed by the upstream project. The differences are listed in
> [CHANGES-FROM-UPSTREAM.md](CHANGES-FROM-UPSTREAM.md). The crates here are not published to
> crates.io; the `ooxml-*` crates on crates.io are upstream's.

Rust library for reading and writing Office Open XML formats: DOCX, XLSX, and PPTX.

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

## Crates

| Crate | Description |
|-------|-------------|
| [`ooxml-wml`](crates/ooxml-wml) | WordprocessingML — read/write `.docx` |
| [`ooxml-sml`](crates/ooxml-sml) | SpreadsheetML — read/write `.xlsx` |
| [`ooxml-pml`](crates/ooxml-pml) | PresentationML — read/write `.pptx` |
| [`ooxml-dml`](crates/ooxml-dml) | DrawingML — shared graphics layer |
| [`ooxml-omml`](crates/ooxml-omml) | Office Math Markup Language |
| [`ooxml-opc`](crates/ooxml) | OPC packaging core (ZIP, relationships, content types) |

## Quick Start

### Read a Word document

```rust
use ooxml_wml::Document;
use ooxml_wml::ext::{BodyExt, ParagraphExt};

let doc = Document::open("document.docx")?;

// All paragraphs
for para in doc.body().paragraphs() {
    println!("{}", para.text());
}

// Full plain text
println!("{}", doc.text());
```

### Write a Word document

```rust
use ooxml_wml::{DocumentBuilder, ListType};

let mut builder = DocumentBuilder::new();

// Heading
{
    let para = builder.body_mut().add_paragraph();
    let run = para.add_run();
    run.set_text("My Document");
    run.set_bold(true);
    run.set_font_size(48); // half-points, so 24pt
}

// Paragraph
builder.add_paragraph("A regular paragraph.");

// Bulleted list
let list_id = builder.add_list(ListType::Bullet);
for item in ["First", "Second", "Third"] {
    let para = builder.body_mut().add_paragraph();
    para.add_run().set_text(item);
    para.set_numbering(list_id, 0);
}

// Table
let table = builder.body_mut().add_table();
let row = table.add_row();
for header in ["Name", "Value"] {
    let cell = row.add_cell();
    cell.add_paragraph().add_run().set_text(header);
}

builder.save("output.docx")?;
```

### Read a spreadsheet

```rust
use ooxml_sml::{Workbook, WorksheetExt, RowExt, CellResolveExt};

let mut workbook = Workbook::open("data.xlsx")?;
println!("Sheets: {:?}", workbook.sheet_names());

let sheet = workbook.resolved_sheet(0)?;
for row in sheet.rows() {
    for cell in row.cells_iter() {
        print!("{}\t", cell.value_as_string(sheet.context()));
    }
    println!();
}
```

### Read a presentation

```rust
use ooxml_pml::{Presentation, ShapeExt};

let mut pres = Presentation::open("slides.pptx")?;

for slide in pres.slides()? {
    println!("--- Slide {} ---", slide.index() + 1);
    for shape in slide.shapes() {
        if let Some(text) = shape.text() {
            println!("{}", text);
        }
    }
    if let Some(notes) = slide.notes() {
        println!("Notes: {}", notes);
    }
}
```

## Features

Each crate uses fine-grained feature flags for smaller compile times. The `full` feature (enabled by default) includes everything.

**ooxml-wml features:** `wml-styling`, `wml-tables`, `wml-layout`, `wml-hyperlinks`, `wml-drawings`, `wml-numbering`, `wml-comments`, `wml-fields`, `wml-track-changes`, `wml-settings`, `wml-math`, `wml-charts`

**ooxml-sml features:** `sml-styling`, `sml-formulas`, `sml-layout`, `sml-filtering`, `sml-validation`, `sml-comments`, `sml-charts`, `sml-hyperlinks`, `sml-pivot`, `sml-tables`, and more

**ooxml-pml features:** `pml-transitions`, `pml-animations`, `pml-notes`, `pml-comments`, `pml-styling`, `pml-masters`, `pml-hyperlinks`, `pml-charts`

Example minimal dependency:

```toml
[dependencies]
ooxml-wml = { version = "0.1", default-features = false, features = ["wml-styling"] }
```

## Design

- **Typed** — every XML element maps to a Rust struct, generated from the ECMA-376 RELAX NG schemas
- **Roundtrip-faithful** — unknown elements and attributes are preserved, never silently dropped
- **Lazy** — large parts (worksheets, slide lists) are streamed rather than parsed upfront
- **Spec-driven** — types and names follow ECMA-376 / ISO 29500; deviations are documented

## Status

The library is in active development at `v0.1`. The core reading and writing APIs for all three formats are functional. See [TODO.md](TODO.md) for the backlog and [SPEC.md](SPEC.md) for the full roadmap.

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
