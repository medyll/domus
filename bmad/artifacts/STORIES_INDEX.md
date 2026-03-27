# Domus Implementation Stories Index

**Total Stories Created:** 12
**Total Points:** 120
**Status:** Ready for development

---

## Story Quick Reference

### MVP 1: Reactive Core (Sprint 1)

| ID | Title | Points | Status | Epic |
|----|-------|--------|--------|------|
| S1-01 | Set up Cargo workspace and crate structure | 3 | ⏳ Pending | MVP 1 |
| S1-02 | Implement Signal<T> core type | 5 | ⏳ Pending | MVP 1 |
| S1-03 | Implement TLS runtime and Effect type | 5 | ⏳ Pending | MVP 1 |
| S1-04 | Integrate auto-tracking between Signal and Effect | 3 | ⏳ Pending | MVP 1 |
| S1-05 | Implement batching scheduler | 5 | ⏳ Pending | MVP 1 |
| S1-06 | Implement Scope system for memory management | 5 | ⏳ Pending | MVP 1 |

**Sprint 1 Total: 26 points**

### MVP 2: Rendering Engine (Sprint 2)

| ID | Title | Points | Status | Epic |
|----|-------|--------|--------|------|
| S2-01 | Create RSX parser for domus! macro | 8 | ⏳ Pending | MVP 2 |
| S2-02 | Implement code generation for static elements | 5 | ⏳ Pending | MVP 2 |
| S2-03 | Implement code generation for dynamic bindings | 8 | ⏳ Pending | MVP 2 |
| S2-04 | Implement event handler code generation | 5 | ⏳ Pending | MVP 2 |

**Sprint 2 Total: 26 points**

### MVP 3: Component System (Sprint 3)

| ID | Title | Points | Status | Epic |
|----|-------|--------|--------|------|
| S3-01 | Implement DomusComponent trait | 5 | ⏳ Pending | MVP 3 |

**Sprint 3 Total: 5+ points** (more stories to be created)

### MVP 4: Routing System (Sprint 4)

| ID | Title | Points | Status | Epic |
|----|-------|--------|--------|------|
| S4-01 | Implement DomusPage trait and Router | 8 | ⏳ Pending | MVP 4 |

**Sprint 4 Total: 8+ points** (more stories to be created)

### MVP 5: Advanced Features (Sprint 5)

| ID | Title | Points | Status | Epic |
|----|-------|--------|--------|------|
| S5-01 | Implement For<T> component for lists | 8 | ⏳ Pending | MVP 5 |
| S5-02 | Implement Context API | 5 | ⏳ Pending | MVP 5 |
| S5-03 | Implement Scoped CSS system | 6 | ⏳ Pending | MVP 5 |

**Sprint 5 Total: 19 points** (more stories to be created)

### MVP 6: CLI & Production (Sprint 6)

| ID | Title | Points | Status | Epic |
|----|-------|--------|--------|------|
| S6-01 | Implement domius-cli basic commands | 8 | ⏳ Pending | MVP 6 |
| S6-02 | Implement MutationObserver-based disposal | 5 | ⏳ Pending | MVP 6 |
| S6-03 | Create hello-world example application | 5 | ⏳ Pending | MVP 6 |

**Sprint 6 Total: 18+ points** (more stories to be created)

---

## Stories by Dependency Order

### Foundation (Must be first)
```
S1-01 (Setup)
  ↓ S1-02 (Signal)
    ↓ S1-03 (Effect/TLS)
      ↓ S1-04 (Auto-tracking)
        ↓ S1-05 (Batching)
          ↓ S1-06 (Scoping)
```

### Macro System (After S1-06)
```
S1-06 (Scoping)
  ↓ S2-01 (Parser)
    ↓ S2-02 (Static Gen)
      ↓ S2-03 (Dynamic Gen)
        ↓ S2-04 (Events)
```

### Framework (After S2-04)
```
S2-04 (Events)
  ↓ S3-01 (Component)
    ↓ S4-01 (Router)
      ↓ S5-01 (Lists)
        ↓ S6-01 (CLI)
```

### Parallel (Independent)
```
S5-02 (Context) — can start after S3-01
S5-03 (CSS) — can start after S5-01
S6-02 (Disposal) — can start after S1-06
S6-03 (Example) — can start after S4-01
```

---

## Story Details Summary

### Epic Breakdown

**MVP 1: Reactive Core**
- 6 stories, 26 points
- Builds the Signal/Effect/TLS system
- No browser integration yet
- Full unit test coverage

**MVP 2: Rendering**
- 4 stories, 26 points
- Creates domus! macro
- Generates web-sys calls
- Compiles to WASM

**MVP 3: Components**
- 1+ stories, 5+ points
- DomusComponent trait
- Props/State separation
- Component composition

**MVP 4: Routing**
- 1+ stories, 8+ points
- DomusPage trait
- Router implementation
- Type-safe navigation

**MVP 5: Advanced**
- 3+ stories, 19+ points
- For<T> list component
- Context API
- Scoped CSS system

**MVP 6: Production**
- 3+ stories, 18+ points
- CLI tool
- Memory disposal
- Example applications

---

## Acceptance Criteria Summary

Each story defines clear acceptance criteria that serve as:
1. Definition of done
2. Test requirements
3. Feature completeness checklist

All criteria must pass before story is marked complete.

---

## Implementation Checklist

### Pre-Sprint 1
- [ ] Read all stories S1-01 through S6-03
- [ ] Understand dependency graph
- [ ] Identify any unknowns or risks
- [ ] Prepare development environment
- [ ] Set up CI/CD pipeline

### Sprint 1 Preparation
- [ ] Review S1-01 through S1-06 in detail
- [ ] Estimate any story points that seem off
- [ ] Prepare test structure for S1-02 onwards
- [ ] Gather references and documentation

### During Sprint
- [ ] Update story status as work progresses
- [ ] Move stories from ⏳ Pending → 🔄 In Progress → ✅ Done
- [ ] Track blockers and risks
- [ ] Update BMAD status.yaml daily

---

## Story Template

Each story includes:
- **Story ID:** Unique identifier (S{sprint}-{seq:02d})
- **Title:** Clear, action-oriented
- **Points:** Estimated effort
- **Epic:** Which MVP it belongs to
- **Description:** Context and goals
- **Acceptance Criteria:** Must-have checklist
- **Implementation Notes:** Technical guidance
- **Dependencies:** Other stories required first
- **Testing Strategy:** How to verify completion
- **References:** Links to docs, frameworks, etc.

---

## How to Use This Roadmap

### For Developers
1. Pick the next ⏳ Pending story from current sprint
2. Read **Description** and **Acceptance Criteria**
3. Review **Implementation Notes** for approach
4. Reference **Testing Strategy** as you code
5. Mark ✅ Done when all criteria pass

### For Project Managers
1. Track progress using story status
2. Watch for blockers in **Dependencies**
3. Adjust timelines if stories run long
4. Celebrate sprint completions

### For Architects
1. Review **Implementation Notes** for quality
2. Verify **Testing Strategy** is comprehensive
3. Escalate any design concerns
4. Prepare next sprint's stories

---

## Estimated Burndown

```
Sprint 1: |██████| 26 points (~3 weeks)
Sprint 2: |██████| 26 points (~3 weeks)
Sprint 3: |███   | 10+ points (~2 weeks)
Sprint 4: |███   | 8+ points (~2 weeks)
Sprint 5: |████  | 19+ points (~3 weeks)
Sprint 6: |████  | 18+ points (~3 weeks)

Total:    ~18-20 weeks to MVP completion
```

---

## File Locations

All story files are located in: `bmad/artifacts/stories/`

Each story has its own file:
- `S1-01.md`
- `S1-02.md`
- ... and so on

Stories can be opened and edited individually to track progress.

---

## Next Phase: Story Refinement

Before Sprint 1 begins:
1. ✅ Initial story creation (DONE)
2. ⏳ Story refinement and estimation review
3. ⏳ Risk assessment and mitigation planning
4. ⏳ Team alignment and assignment
5. ⏳ Sprint 1 kickoff

---

**Stories Created:** 2026-03-19
**Total Coverage:** MVP 1-6 (all phases)
**Estimated Total Points:** 120
**Ready for:** Sprint 1 Execution
