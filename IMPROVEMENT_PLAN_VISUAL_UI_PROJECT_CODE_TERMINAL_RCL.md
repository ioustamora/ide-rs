# IDE-RS Comprehensive Improvement Plan

Date: 2025-09-07

Scope Areas:

1. Visual UI Editor
2. Project & File Management
3. Source Code Editor
4. Integrated Terminal
5. RCL Library
6. RCL ⇄ IDE Integration (End‑to‑End Workflow)

---

## 1. Visual UI Editor (Designer + Property / Hierarchy)

### Current Gaps

- No canonical scene graph with stable IDs & diffing.
- Property editing not schema‑driven; no typed reflection.
- Limited undo/redo (coarse granularity) & no per‑property command log.
- Missing layout abstraction (flex/grid/anchor) and snapping/alignment guides.
- No event binding or code round‑trip (design ↔ generated code).
- No responsive preview / multi‑device frames.

### Roadmap (Visual UI Editor)

| Phase | Deliverables | Notes |
|-------|--------------|-------|
| P0 | `SceneStore` (components, properties, children, layout_meta), UUID IDs, Command pattern undo/redo, PropertySchema & auto inspector | Foundation |
| P1 | Layout engines (absolute, flex axis, simple grid), alignment & snap guides, hierarchy drag & reparent, multi-select | Productivity |
| P2 | Code round‑trip parse/generate, responsive frames, constraints (anchors, min/max) | Round‑trip |
| P3 | Data binding, design-time event wiring, diff view | Advanced |

### Success Metrics

- 200 component scene: traversal < 5ms/frame.
- Undo latency < 10ms.
- Code regen idempotent (hash stable across runs unless real change).

---

## 2. Project & File Management

### Gaps (Project Management)

- No buffer manager (dirty state / versioning) or external file watcher.
- No background task queue (indexing, search, symbol scan).
- Limited template post‑processing; no multi-root workspace features.
- No refactor-safe (LSP) renames or text search index.

### Roadmap (Project Management)

| Phase | Deliverables |
|-------|--------------|
| P0 | BufferManager (open buffers map), file dirty tracking, notify-based watcher, persisted config (recent projects, settings JSON), async WorkspaceTask queue |
| P1 | Full‑text search index (trigram/fst), template variable expansion + hooks, LSP rename integration, project-wide symbol cache |
| P2 | Multi-root workspace model, dependency graph extraction (Cargo metadata), batch operations (format all, test all) |

### Metrics (Project Management)

- Search across 5k files < 500ms.
- Opening 50 files memory growth predictable (< +15%).

---

## 3. Source Code Editor (AdvancedCodeEditor)

### Gaps (Source Editor)

- Plain String storage; no rope for large file efficiency.
- Full-document LSP change events (inefficient) & approximate char metrics.
- Syntax highlighting placeholder; no viewport tokenization.
- Lacking undo/redo stack, selection model, multi-cursor insertion logic.
- Diagnostics mapping naive; missing symbol outline, find/replace, semantic tokens.

### Roadmap (Source Editor)

| Phase | Deliverables |
|-------|--------------|
| P0 | `TextBuffer` (ropey or custom rope), diff-based LSP changes, structured Cursor/Selection model, operation undo stack, viewport tokenization only |
| P1 | Diagnostics gutter, inline squiggles with precise spans, symbol outline (DocumentSymbol), find/replace panel, completion insertion logic |
| P2 | Semantic tokens, inlay hints, code lens, multi-file search integration, persistent editor sessions |
| P3 | Collaborative editing (CRDT / OT), AI assisted refactor previews |

### Metrics (Source Editor)

- 20k line file edit operations < 8ms.
- Scroll latency < 16ms/frame.

---

## 4. Integrated Terminal

### Gaps (Terminal)

- No ANSI parsing; plain text only.
- Lacks PTY/conpty abstraction; limited interactive compatibility.
- No task linkage (build/test) or error navigation bridging to editor.
- No session persistence, splits, or style theme integration.

### Roadmap (Terminal)

| Phase | Deliverables |
|-------|--------------|
| P0 | ANSI parser (vt100 or minimal), StyledSpan buffer, scrollback ring, theme colors |
| P1 | Task API: structured build/test/run tasks + status line; error line clicks open file; command palette integration |
| P2 | Split panes, session persistence (JSON state), environment profiles |
| P3 | Attach external process, terminal multiplex (linking to remote) |

### Metrics (Terminal)

- 10k line scrollback render < 12ms.
- Build output error links open correct file/line reliably.

---

## 5. RCL Library

### Gaps (RCL Library)

- Components lack unified metadata registry & property typing.
- Theming not layered (no overrides per component variant).
- No standardized event set or serialization schema.
- CodeEditor component is minimal (no integration with advanced editor core).

### Roadmap (RCL Library)

| Phase | Deliverables |
|-------|--------------|
| P0 | ComponentRegistry, PropertyValue enum, PropertySchema, derive macro `ComponentMeta`, basic event list & inspector generation |
| P1 | Serialization (ComponentSnapshot JSON), theming layers (base + overrides), improved CodeEditor bridging to advanced backend |
| P2 | Data binding expressions, accessibility metadata, runtime dynamic loading plugin pattern |
| P3 | Live theme hot-swap + variant introspection, design tokens export |

### Metrics (RCL Library)

- Register 100 components start-up < 50ms.
- Inspector build per component < 2ms.

---

## 6. RCL ⇄ IDE Integration (Round‑Trip Flow)

### Target Flow (Integration)

Design visually → generate declarative UI spec → code scaffolding (guarded regions) → user edits logic outside guarded blocks → save triggers live preview update → edits in generated region reparse back into scene.

### Model (Integration)

- DSL / snapshot: `ComponentSnapshot { type, props, children[] }` → codegen → `generated/ui/*.rs` with markers:
  
  ```rust
  // <IDE-GENERATED-START>
  // auto-generated layout
  // <IDE-GENERATED-END>
  ```
  
- Parser maps code spans back to snapshot nodes (store span offsets & ID mapping).

### Roadmap (Integration)

| Phase | Deliverables |
|-------|--------------|
| P0 | Code generator + guarded sections, snapshot serializer, basic live preview panel |
| P1 | Round-trip parser (lossless for supported constructs), hover mapping (component -> AST span), event handler insertion |
| P2 | Hot reload diff application (structural patch not full regen), partial regen of changed subtrees |
| P3 | Intelligent merge for manual edits inside generated block (heuristic or structured comments) |

### Metrics (Integration)

- Snapshot → code gen < 80ms (500 components).
- Code edit → preview refresh < 150ms.

---

## Cross-Cutting Infrastructure

### Event Bus

`IdeEvent` variants: BufferChanged, ProjectLoaded, ComponentUpdated, BuildStarted, BuildFinished, DiagnosticsUpdated, TaskStatus.

### Background Task Runner

- Priority queue; tasks emit progress events; cancellable.

### Telemetry (optional)

- Collect LSP response latency, build durations, render times; toggle in settings.

---

## Immediate Implementation Order (Milestone 1)

1. Core `TextBuffer` abstraction and integrate into `AdvancedCodeEditor`.
2. `ComponentRegistry` + schemas + auto property inspector.
3. `SceneStore` + command-based undo/redo.
4. ANSI StyledSpan pipeline for terminal.
5. BufferManager & file watcher integration in ProjectManager.
6. Basic code generation (guarded section) scaffolding for visual designer output.

---

## Risk & Mitigation Summary

| Risk | Impact | Mitigation |
|------|--------|------------|
| Rope + diff complexity | Editor stability | Start with ropey crate; wrap in safe API; tests for insert/delete ranges |
| Code regen overwriting user logic | Data loss | Guarded section markers + snapshot hashing |
| Performance regressions (large scenes) | UX degradation | Profiling hooks + per-frame budget monitors |
| LSP latency blocking UI | Sluggish editing | Offload to async thread + channel results back |
| ANSI parsing overhead | Terminal lag | Incremental parse + span reuse + ring buffer |

---

## Validation & Testing Strategy

- Unit tests: text buffer ops, snapshot serialization, codegen idempotency, property schema reflection.
- Integration tests: open project, modify component property → confirm regenerated code diff minimal.
- Performance benchmarks: large file editing, scene with 500 components, terminal spam (colored). Run in CI.
- Golden files: generated UI code snapshots.

---

## KPIs (Quarterly Targets)

- 95% operations under 16ms frame budget.
- Code round-trip accuracy > 99% (unchanged structural semantics after regen cycle).
- Build/test task detection latency < 2s after save.
- Search index build < 30s on 10k files (once; incremental < 2s).

---

## Summary

This plan establishes foundational architecture (buffers, scene graph, registry), iterative feature layering (layout, code round‑trip, semantic tooling), and performance guardrails to evolve ide-rs into a robust, extensible IDE + UI design environment tightly integrated with the RCL component system.
