# IDE-RS Improvement Plan (Visual UI Editor, Project/File Management, Source Code Editor, Terminal, RCL Library & Integration)

Date: 2025-09-08  
Scope Owner: (fill)  
Version: 1.0 (living document)

---
## 1. Executive Summary
The codebase provides a broad but uneven foundation for a modern Rust IDE with a visual UI builder (visual designer + palette + inspector), project/workspace management, integrated terminal(s), and an internal Rust Component Library (RCL). Many modules exist as structural placeholders (e.g. `code_editor::state`, limited code intelligence integration) while others (project manager, terminal, RCL advanced widgets) are comparatively more detailed. This plan prioritizes stabilizing core abstractions, reducing duplication, and enabling a sustainable path toward feature depth (LSP, designer–code round‑tripping, plugin model, performance & testing).

Immediate focus:  
1. Consolidate editor & RCL component lifecycle + property system (unify reflection-like access).  
2. Implement robust text model (rope + incremental syntax) and real code editing features.  
3. Harden project/workspace layer (async file ops, indexing, cache).  
4. Rationalize terminal duplication (merge baseline + advanced).  
5. Formalize architecture: events, services, and extension points.  
6. Introduce automated tests + validation pipeline.

---
## 2. Current State Assessment (Area by Area)

### 2.1 Visual UI Editor (Modules in `editor/` such as `canvas`, `palette`, `visual_designer`, `inspector`, `object_inspector`)
**Strengths**
- Clear modular separation; commentary-rich docs.  
- Component registry & palette concept exist.  
- Inspector & hierarchy foundations present.

**Gaps / Risks**
- Missing centralized immutable/mutable state store pattern (ad-hoc mutable references).  
- No undo/redo stack or command pattern for design actions.  
- No layout constraints engine (layout_manager only in RCL advanced).  
- Persistence of designs → code generation is only partially defined (guarded sections concept noted but not fully implemented).  
- No diff-aware regeneration to preserve user manual edits.  
- Missing selection / drag behaviors abstraction & snapping/alignment grid logic centralization.  
- Property editing logic duplicated per component; lacks typed metadata schema (validation, categories, types, hints).  
- No test coverage (UI logic remains unverified).  
- Lack of event bus for decoupled interactions (e.g. component added → inspector update → code regen queue).

### 2.2 Project & File Management (`project_manager`, `file_browser`, `file_manager`, `file_operations`)
**Strengths**
- Project templates concept and serializer abstraction.  
- Recent projects tracking and backup scaffolding.  
- Multi-tab file manager with dirty tracking + type classification.  
- Early association groundwork for languages.

**Gaps / Risks**
- Synchronous IO; no async or indexing pipeline (blocks UI for large repos).  
- No language/workspace symbol index or search index persistence (search_index placeholder).  
- No virtual file system abstraction (future remote / ephemeral / memory).  
- No watchers integration (file_watchers placeholder not implemented).  
- Lacks workspace multi-root semantics parity with VSCode (partial workspace manager).  
- No dependency graph view or crate metadata extraction.  
- No integrated task orchestration (build/test tasks surfaced only via terminal commands).  
- Missing reliability features: autosave policy enforcement, conflict resolution, external change notifications.

### 2.3 Source Code Editor (`editor/code_editor`, `syntax_highlighter`, `rust_analyzer`, `lsp_integration`, `advanced_code_editor`)
**Strengths**
- High-level modules structured for LSP, advanced features, syntax highlighting.  
- Intention to integrate Rust Analyzer & generic LSP abstraction.  
- Minimap and code folding modules present as placeholders.

**Gaps / Risks**
- Core `code_editor` is effectively a stub (no buffer abstraction beyond naive String).  
- No rope/line index; scaling risk for large files.  
- No incremental syntax tree, diagnostics overlay, inline hints, or semantic tokens pipeline.  
- Lack of edit transaction model (undo/redo, multi-cursor, selections).  
- No background worker / scheduler for LSP requests (potential UI thread blocking).  
- Missing test harness for lexical/tokenization correctness.  
- No plugin extension surface for language injections or custom formatters.  
- LSP integration layering may duplicate logic with `rust_analyzer` module -> unify into LanguageService trait.

### 2.4 Integrated Terminal (`terminal.rs`, `terminal_advanced.rs`, `terminal_ansi.rs`)
**Strengths**
- Extensive advanced design (environment, completions, session mgmt, themes).  
- Command palette alignment with IDE tasks.  
- ANSI parsing placeholder module.

**Gaps / Risks**
- Duplication between basic and advanced terminal models (risk of drift).  
- Direct `std::process` usage without asynchronous reading (potential UI stalls).  
- No PTY layer abstraction; limited cross-platform correctness (Windows specifics).  
- Security considerations (no sanitization / safe execution sandbox).  
- Absence of streaming incremental rendering & scrollback memory strategy (VecDeque only).  
- No test harness (hard to validate log parsing / history / completion).  
- No standardized command/job model for build & tasks (tight coupling to terminal).  
- No plugin injection for custom completions.

### 2.5 RCL Library (`rcl/ui/...`, advanced components)
**Strengths**
- Broad component surface (tabs, tree, data grid, toast, virtual list, etc.).  
- Trait-based `Component` interface clean & object safe.  
- Separation between basic vs advanced directories.

**Gaps / Risks**
- Inconsistent property reflection (manual string switch).  
- Lack of metadata schema (types, ranges, categories, events).  
- No composition/storybook sandbox for isolated component test-driving.  
- Missing theming system central registry (theme struct present but lacks dynamic layering).  
- No animation/layout systems integration (transitions, constraints).  
- Limited accessibility semantics (keyboard navigation, focus order).  
- Lack of performance profiling or virtualization strategy unify (virtual_list separate).  
- No codegen mapping from visual designer to actual instantiation DSL.

### 2.6 Integration (IDE ↔ RCL)
**Strengths**
- Components can be embedded directly; palette/registry conceptual alignment.  
- Visual designer presumably builds tree of `Component` implementors.

**Gaps / Risks**
- No canonical runtime model tying component instances to persistent IDs & serialization format.  
- No diff engine (design state → code updates).  
- Event signaling (component added/removed/properties changed) not standardized.  
- No bridging layer for property inspection (manual per component).  
- Code generation not round-trip safe (guard markers incomplete).  
- Missing design-time vs runtime mode separation (property handles, overlays, selection bounding boxes).

---
## 3. Cross-Cutting Architectural Issues
- Lack of unified async/task scheduler (blocking risk).  
- Need an internal event bus / message dispatcher (editor state, LSP, terminal, project).  
- Testing void (no unit, integration, snapshot tests).  
- Observability missing (structured logging, metrics).  
- Plugin/extension system conceptual only — no capability wiring or sandboxing strategy.  
- Configuration layering (user/workspace/project) absent.  
- Potential monolithic crate growth – consider workspace splitting.

---
## 4. Target Architecture (High-Level)
```
+--------------------+       +------------------+       +------------------+
|   Presentation     |<----->|  View Models     |<----->|  Core Services    |
| (egui UI Panels)   |  EVT  | (State + Commands)|  API | (LSP, FS, Tasks)  |
+--------------------+       +------------------+       +------------------+
           |                              |                      |
           v                              v                      v
   RCL Components                Event Bus / Dispatcher     System Adapters
           |                              |                      |
           +---------- Codegen / Serialization Layer -------------+
```

Key additions:  
1. Event Bus: typed events (enum) with subscription (observer pattern).  
2. Core Services: FileService, ProjectService, LanguageService (trait), TerminalService, ComponentService.  
3. State Management: distinct immutable snapshots (for UI) + mutation commands (command pattern for undo/redo).  
4. Code Generation Pipeline: AST/IR representing design; generator produces Rust with guard markers & stable anchors.  
5. Reflection/Metadata: derive macro `#[derive(ComponentMeta)]` generating property descriptors (type, category, setter/getter).  
6. Rope-based TextBuffer + incremental diff engine feeding LSP sync.

---
## 5. Detailed Recommendations & Phased Roadmap

### Phase 0 (Preparation, Week 0)
- Create `docs/` folder & architecture diagrams (PlantUML or Mermaid).  
- Introduce CI (GitHub Actions) running: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`.  
- Add baseline unit tests for existing simple modules (serialization, project template creation).  
- Add logging crate (e.g., `tracing`) + feature flag.

### Phase 1 (Weeks 1–2) Foundational Refactors
1. Text Buffer: Implement `editor/text_buffer.rs` as rope (use `ropey`) behind feature gate; integrate into FileTab & CodeEditor.  
2. Event System: Add `core/event_bus.rs` (simple subscribers + broadcast). Replace ad-hoc cross-calls (start with file opened, tab switched, project loaded).  
3. Component Metadata: Add macro crate `rcl-macros` deriving `ComponentDescriptor` (properties: name, type, default, doc).  
4. Merge Terminal Layers: Extract shared structs → `terminal/core.rs`; advanced features behind feature flags; unify creation API.  
5. Project File Watching: Implement cross-platform watcher via `notify` crate; update FileManager on external changes.

### Phase 2 (Weeks 3–5) Core Feature Maturation
1. Undo/Redo: Command trait + stack for visual designer operations (add/remove/move component, property change).  
2. Code Generation v1: Implement IR (DesignNode tree). Add guarded sections markers: `// <rad:begin user>` / `// <rad:end user>`.  
3. LanguageService Trait: Unify `rust_analyzer` + `lsp_integration`; add async request queue & throttling.  
4. Inline Diagnostics: Overlay diagnostics per line; quick-fix action placeholder.  
5. Search Index: Implement background indexing (identifiers, file paths) – incremental updates on save.  
6. Terminal PTY Abstraction: Evaluate `portable-pty` crate for cross-platform interactive shells.  
7. Property Inspector Revamp: Use generated metadata to auto-build UI with type-specific editors (bool, enum, color, number, text, list).

### Phase 3 (Weeks 6–9) Integration & UX
1. Designer ↔ Code Round Trip: Diff engine to update only generated regions; detect manual conflicts.  
2. Multi-Root Workspaces: Extend `WorkspaceManager` to load multiple projects; add aggregated symbol search.  
3. AI Integration Pipeline: Provide structured context API (active file slice, diagnostics, symbol summary).  
4. Terminal Task Runner: Formal task manifest (JSON/TOML) -> named tasks UI.  
5. Theming & Appearance: Central theme registry with dynamic switching; propagate to RCL components.  
6. Accessibility & Keyboard: Focus traversal + command palette expansions; shortcuts mapping persistence.

### Phase 4 (Weeks 10–14) Advanced & Performance
1. Incremental Parsing / Syntax Trees: integrate `tree-sitter` (multi-language) with token -> theme mapping.  
2. Performance Telemetry: Frame timings, LSP latency, indexing stats; visualize in performance panel.  
3. Plugin System (Prototype): Dynamic loading (feature-gated) + sandbox capability manifest (declares hooks & resources).  
4. Visual Testing Harness: Headless rendering snapshot tests (compare hash).  
5. Collaborative Editing (Strategy Draft): CRDT or operational transform plan (foundation for realtime_sync module).

### Phase 5 (Longer-Term) Strategic
1. Remote Workspace Support (SSH / container).  
2. Cloud AI Agents with structured tool protocol.  
3. Cross-platform packaging & updater service.  
4. Marketplace for components/plugins.

---
## 6. Component Metadata & Reflection Proposal
Trait: `Component` stays minimal. New generated struct: `ComponentDescriptor { name: &'static str, properties: &'static [PropertyDescriptor] }`.  
`PropertyDescriptor { name, kind: PropertyKind, default_value, category, doc, flags }` where `PropertyKind` enumerates primitives & composite (Color, Size, Enum, List<T>, Callback).  
Generate helper: `fn apply_property(&mut self, name: &str, value: PropertyValue)` eliminating string parsing boilerplate.  
Property editing UI becomes generic.

---
## 7. Text Buffer & Editor Architecture
Core: `TextBuffer` (rope) + `Edit { range: (LineCol, LineCol), inserted: RopeSlice }` -> undo stack.  
Selections: primary + secondary (Vec<Range>).  
Services: SyntaxHighlighter (async tokenization slices), DiagnosticsOverlay, FoldingRegions provider.  
Data Flow: UI event → Command → Buffer mutation → EventBus (BufferChanged) → LanguageService sync (debounced) → Diagnostics → EventBus → UI overlays.

---
## 8. Code Generation & Round Trip
Markers:  
```rust
// <rad:begin generated>
// auto-emitted code
// <rad:end generated>
// <rad:begin user>
// developer customizations
// <rad:end user>
```
Algorithm: Parse existing file; slice into segments; rewrite only generated block; preserve user block order. Add stable IDs (e.g., component UUID) in comments for mapping.  
Conflict Strategy: If user edits inside generated block, flag during regeneration and prompt resolution (log + UI overlay).

---
## 9. Terminal Consolidation Plan
Refactor:  
- `terminal/core.rs`: Core structs (TerminalSession, OutputLine, ShellType, Settings).  
- `terminal/features/{autocomplete,env,git,package}.rs` gating advanced features.  
- Stream reading via async runtime (tokio) with channel to UI.  
- Scrollback ring buffer with memory ceiling (MB-based).  
- Task integration: `TaskDescriptor` (name, command, cwd, env, problem_matchers optional).  
- Provide events: TerminalOpened, TerminalClosed, LineAppended, TaskCompleted.

---
## 10. Project & Workspace Enhancements
File Index Service: (path, size, modified, language, symbol hashes).  
Search Index: token → postings list (in-memory + serialized).  
Watcher: reconcile changed files; prompt reload if dirty.  
Template DSL: metadata file describing dependencies, example components.  
Backups: compress into timestamped archives; prune policy.  
Workspace Config Layers: `.iderc` (user) + `project.ide.toml` (project) + overrides.

---
## 11. Testing & Quality Strategy
Test Pyramid:  
- Unit: serialization, codegen diff engine, text buffer edits.  
- Integration: open project → modify file → regenerate code → confirm markers.  
- Snapshot: component rendering (hash).  
- Performance Baselines: open 5k line file edit latency < threshold.  

Introduce: `cargo nextest` (optional), mutation testing later.  
CI Gates: fmt, clippy (deny warnings), tests, minimal doc build.  
Nightly Bench (feature gated) for buffer ops + LSP throughput.

---
## 12. Metrics & KPIs
Primary:  
- Edit latency p95 (ms).  
- LSP round-trip p95.  
- Codegen time vs components count.  
- Memory footprint baseline vs threshold.  
- Crash/panic count per session.  
- Test coverage % (line + mutation).  
- Startup time (cold).  
- Index build time.

---
## 13. Risks & Mitigations
| Risk | Impact | Mitigation |
|------|--------|------------|
| Scope creep across many feature fronts | Dilution of progress | Strict phase gates + definition of done |
| UI thread blocking due to sync ops | Perceived sluggishness | Async services + job queue |
| Codegen overwrites user logic | Data loss | Guarded sections + diff check + warning dialog |
| Terminal platform inconsistencies | Broken user workflows | Abstract PTY layer + integration tests |
| Missing tests enabling regressions | Stability issues | Enforce CI gates early |
| Macro-based reflection complexity | Compile errors/hard debugging | Start minimal; thorough docs & tests |

---
## 14. Immediate Action Items (Sprint 1 Backlog Candidates)
ID | Item | Est | Owner | Notes
---|------|-----|-------|------
S1-1 | Add CI workflow + fmt/clippy/test | 0.5d |  | GitHub Actions
S1-2 | Introduce `tracing` logging | 0.5d |  | Feature gated
S1-3 | Implement basic EventBus | 1d |  | Start with enum + Vec<Sender>
S1-4 | Rope-based `TextBuffer` w/ tests | 2d |  | Use `ropey`
S1-5 | Merge terminal core abstraction | 2d |  | Reduce duplication
S1-6 | File watcher integration | 1d |  | `notify` crate
S1-7 | Component metadata derive prototype | 2d |  | New macro crate
S1-8 | Codegen markers & rewrite proto | 2d |  | For single form file

---
## 15. Suggested Crate / Module Reorganization (Later Phase)
Workspace Layout:  
- `crates/ide-core` (event bus, services, text buffer, codegen IR).  
- `crates/ide-ui` (egui panels, visual designer).  
- `crates/rcl` (component library).  
- `crates/rcl-macros` (derive).  
- `crates/ide-terminal` (terminal + tasks).  
- `app/` binary crate.  
Enables parallel compilation & clearer boundaries.

---
## 16. Tooling & Developer Experience
- Add `cargo xtask` pattern for codegen validation, component docs generation.  
- Generate component docs site (static markdown) from metadata derive.  
- Add `--profile instrumentation` to gather perf baselines.  
- Optional dev panel with real-time event log & memory usage.

---
## 17. Documentation Enhancements
- Architecture Overview (high-level diagram).  
- Component Authoring Guide (derive macro usage).  
- Code Generation & Guard Policy doc.  
- Text Buffer design notes.  
- Terminal architecture doc.  
- Contribution & style guide (naming, module layout, error handling).  
- Testing strategy doc.

---
## 18. Conclusion
This plan emphasizes establishing reliable foundations (buffer, events, metadata, codegen safety) before layering advanced UX (AI, multi-root mastery, plugin system). By following phased execution with measurable success criteria, the project can evolve from a broad scaffold toward a robust, extensible Rust-centric IDE.

---
## 19. Appendix A: Event Enum (Draft)
```rust
pub enum IdeEvent {
    ProjectLoaded { path: PathBuf },
    FileOpened { path: PathBuf },
    FileSaved { path: PathBuf },
    BufferChanged { path: PathBuf, version: u64 },
    ComponentAdded { id: Uuid, kind: String },
    ComponentPropertyChanged { id: Uuid, name: String },
    CodeGenerationRequested { reason: String },
    DiagnosticsUpdated { path: PathBuf, count: usize },
    TerminalOutput { terminal_id: usize, line: String },
    TaskCompleted { name: String, status: TaskStatus },
}
```

## 20. Appendix B: Guarded Section Example
```rust
// <rad:begin generated>
fn build_ui(ctx: &egui::Context) {
    // auto generated component tree
}
// <rad:end generated>

// <rad:begin user>
// Custom logic persists across regenerations
fn custom_logic() {
    println!("user code");
}
// <rad:end user>
```

## 21. Appendix C: Property Metadata Derive (Sketch)
```rust
#[derive(ComponentMeta)]
pub struct Button {
    #[component(name="Text", category="Appearance")] 
    label: String,
    #[component(default=100, name="Width", category="Layout")] 
    width: u32,
    #[component(default=true, name="Enabled", category="Behavior")] 
    enabled: bool,
}
```

---
End of document.
