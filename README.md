# text-please

Extract plain text from PDF, DOCX, XLSX, and plain text files.

## Install

```bash
npm install text-please
```

## Usage

```js
import { extractText, extractPdfToText, extractDocxToText, extractXlsxToText, extractPlaintextToText } from 'text-please';

const pdfData = await fetch('document.pdf').then(r => r.arrayBuffer());
const text = extractPdfToText(new Uint8Array(pdfData));
```

## API

All functions accept a `Uint8Array` of document data and return a `string`. They throw a `JsError` on failure.

### Auto-detection

```js
extractText(data: Uint8Array): string
```

Detects document type automatically and extracts text. Supports PDF, DOCX, XLSX, and plain text.

### Individual extractors

```js
extractPdfToText(pdf: Uint8Array): string
extractDocxToText(docx: Uint8Array): string
extractXlsxToText(xlsx: Uint8Array): string
extractPlaintextToText(data: Uint8Array): string
```

Extract text from a specific format. For XLSX, cells are joined with tabs and rows with newlines.

## Error handling

Functions throw a `JsError` with a message on failure:

- Unknown format: `"Unknown document format"`
- Parsing errors: specific error from the parser
- UTF-8 errors: `"Invalid UTF-8: ..."`
