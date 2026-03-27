# Domus Sprint Roadmap

**Total Stories:** 24
**Estimated Sprints:** 6
**Story Points:** 120
**Velocity Target:** 20 points/sprint

---

## Sprint 1: Reactive Core Foundation (MVP 1)

**Duration:** 2-3 weeks
**Goal:** Establish the Signal/Effect/TLS system
**Points:** 26

### Stories

| ID | Title | Points | Status |
|-----|-------------------------------------------------|--------|--------|
| **S1-01** | Set up Cargo workspace | 3 | ⏳ Pending |
| **S1-02** | Implement Signal<T> core type | 5 | ⏳ Pending |
| **S1-03** | Implement TLS runtime and Effect type | 5 | ⏳ Pending |
| **S1-04** | Integrate auto-tracking | 3 | ⏳ Pending |
| **S1-05** | Implement batching scheduler | 5 | ⏳ Pending |
| **S1-06** | Implement Scope system | 5 | ⏳ Pending |

**MVP 1 Success Criteria:**
- ✅ Signal stores and retrieves values
- ✅ Effects auto-run when dependencies change
- ✅ TLS tracks current effect
- ✅ No manual dependency registration needed
- ✅ Batching prevents multiple runs
- ✅ Scopes cleanup effects

**Deliverables:**
- `domius-core` crate with complete reactive system
- Comprehensive unit tests
- Zero unsafe code

---

## Sprint 2: Rendering Engine (MVP 2)

**Duration:** 2-3 weeks
**Goal:** Create the domus! macro and DOM binding
**Points:** 26

### Stories

| ID | Title | Points | Status |
|----|-------|--------|--------|
| **S2-01** | Create RSX parser | 8 | ⏳ Pending |
| **S2-02** | Implement code generation for static elements | 5 | ⏳ Pending |
| **S2-03** | Implement code generation for dynamic bindings | 8 | ⏳ Pending |
| **S2-04** | Implement event handler code generation | 5 | ⏳ Pending |

**MVP 2 Success Criteria:**
- ✅ domus! macro accepts RSX syntax
- ✅ Static HTML renders correctly
- ✅ Dynamic bindings update on signal change
- ✅ Event listeners work
- ✅ Compiled WASM works in browser

**Deliverables:**
- `domius-macro` crate with complete code generation
- Hello world example compiles to WASM
- Integration tests with headless browser

---

## Sprint 3: Component System (MVP 3)

**Duration:** 2 weeks
**Goal:** Establish component trait and composition
**Points:** 10+

### Stories

| ID | Title | Points | Status |
|----|-------|--------|--------|
| **S3-01** | Implement DomusComponent trait | 5 | ⏳ Pending |
| **S3-02** | Component composition (Props/State) | 5 | ⏳ Pending (estimated) |

**MVP 3 Success Criteria:**
- ✅ Components implement DomusComponent trait
- ✅ Props pass through to children
- ✅ State remains local to component
- ✅ Multiple components compose

**Deliverables:**
- `domius-web` crate with component system
- Button, Input components as examples
- Component lifecycle working

---

## Sprint 4: Routing System (MVP 4)

**Duration:** 2 weeks
**Goal:** Implement page routing and navigation
**Points:** 8+

### Stories

| ID | Title | Points | Status |
|----|-------|--------|--------|
| **S4-01** | Implement DomusPage trait and Router | 8 | ⏳ Pending |

**MVP 4 Success Criteria:**
- ✅ URL changes trigger page transitions
- ✅ Multiple pages work
- ✅ Page lifecycle hooks (on_load) fire
- ✅ Type-safe navigation

**Deliverables:**
- Router implementation
- Example app with multiple pages
- Browser history working

---

## Sprint 5: Advanced Features (MVP 5)

**Duration:** 2-3 weeks
**Goal:** Lists, Context, Scoped CSS
**Points:** 24

### Stories

| ID | Title | Points | Status |
|----|-------|--------|--------|
| **S5-01** | Implement For<T> list component | 8 | ⏳ Pending |
| **S5-02** | Implement Context API | 5 | ⏳ Pending |
| **S5-03** | Implement Scoped CSS system | 6 | ⏳ Pending |

**MVP 5 Success Criteria:**
- ✅ Lists render 1000+ items efficiently
- ✅ List updates are surgical (O(1))
- ✅ Context shares state without props
- ✅ CSS is automatically scoped per component

**Deliverables:**
- For<T> component with key-based reconciliation
- Context API working
- CSS scoping in macro and CLI

---

## Sprint 6: CLI & Production (MVP 6)

**Duration:** 2-3 weeks
**Goal:** Automation and production readiness
**Points:** 24

### Stories

| ID | Title | Points | Status |
|----|-------|--------|--------|
| **S6-01** | Implement domius-cli basic commands | 8 | ⏳ Pending |
| **S6-02** | Implement MutationObserver disposal | 5 | ⏳ Pending |
| **S6-03** | Create hello-world example | 5 | ⏳ Pending |
| **S6-04** | Error boundaries (estimated) | 3 | ⏳ Pending |
| **S6-05** | Testing utilities (estimated) | 3 | ⏳ Pending |

**MVP 6 Success Criteria:**
- ✅ CLI generates projects
- ✅ No memory leaks with dynamic lists
- ✅ Error boundaries prevent crash propagation
- ✅ Production bundle < 200KB gzipped
- ✅ Complete documentation

**Deliverables:**
- `domius-cli` tool working
- Zero memory leaks
- v0.1.0-alpha release ready

---

## Timeline Summary

```
Week 1-3:   MVP 1 (Reactive Core)          ████████░░░░░░░░░░░░░░░░░
Week 4-6:   MVP 2 (Rendering)              ░░░░░░░░████████░░░░░░░░░░
Week 7-8:   MVP 3 (Components)             ░░░░░░░░░░░░░░░░████░░░░░░
Week 9-10:  MVP 4 (Routing)                ░░░░░░░░░░░░░░░░░░░░████░░
Week 11-13: MVP 5 (Advanced)               ░░░░░░░░░░░░░░░░░░░░░░░████
Week 14-16: MVP 6 (Production)             ░░░░░░░░░░░░░░░░░░░░░░░░███

Total: 18-20 weeks for full framework
```

---

## Critical Path

The dependency chain determines the critical path:

```
S1-01 (Setup)
  ↓
S1-02 (Signal)
  ↓
S1-03 (Effect/TLS)
  ↓
S1-04 (Auto-tracking)
  ↓
S1-05 (Batching)
  ↓
S2-01 (RSX Parser)
  ↓
S2-02 (Static Gen)
  ↓
S2-03 (Dynamic Gen)
  ↓
S2-04 (Events)
  ↓
S3-01 (Component Trait)
  ↓
S4-01 (Router)
  ↓
S5-01 (For<T>)
```

**Any delay in S1-05 blocks all subsequent sprints.**

---

## Parallelization Opportunities

Some stories can run in parallel:
- **Sprint 1:** All stories are sequential (foundation)
- **Sprint 2:** S2-01 and S2-02 can start after S1-04
- **Sprint 3+:** More flexibility as dependencies clear

---

## Resource Requirements

| Role | Sprint 1 | Sprint 2 | Sprint 3+ |
|------|----------|----------|----------|
| **Architect** | Design | Review | Consulting |
| **Senior Dev** | S1-01-05 | S2-01-04 | Lead S3+ |
| **Junior Dev** | Support | S2-04 review | S3 support |
| **QA/Tester** | Unit tests | Integration tests | System tests |

---

## Risk Mitigation

| Risk | Impact | Mitigation |
|------|--------|-----------|
| Macro complexity (S2-01) | High | Early prototyping, simpler syntax first |
| List reconciliation (S5-01) | Medium | Algorithm proven in other frameworks |
| Browser compatibility | Medium | Test across Chrome/Firefox/Safari |
| Memory management | High | MutationObserver testing critical |

---

## Definition of Done (for each sprint)

- [ ] All stories pass acceptance criteria
- [ ] Code reviewed and merged
- [ ] Unit test coverage > 80%
- [ ] Integration tests passing
- [ ] No regressions from previous sprints
- [ ] Documentation updated
- [ ] Example code works

---

## Next Steps

1. **Before Sprint 1 starts:**
   - Review stories with team
   - Clarify acceptance criteria
   - Identify blockers
   - Assign owners

2. **Prepare Sprint 1:**
   - Set up project repository
   - Create development environment
   - Establish CI/CD pipeline

3. **Begin S1-01:**
   - Create Cargo workspace
   - Set up WASM toolchain
   - Verify build process

---

**Roadmap Created:** 2026-03-19
**Last Updated:** 2026-03-19
**Status:** Ready for Sprint 1 startup
