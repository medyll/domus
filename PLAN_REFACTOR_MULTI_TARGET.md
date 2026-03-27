# Plan de Refactorisation Multi-Cibles

**Objectif** : Rendre Domus compatible avec le Web (WASM), Tauri (desktop), et futures plateformes.

---

## 📋 État Actuel

### Problèmes Identifiés

1. **`domius-core` dépend de WASM**
   - `wasm-bindgen` dans `Cargo.toml`
   - `web_sys::console` pour les logs
   - Bloque l'utilisation sur desktop/native

2. **Architecture monolithique**
   - `domius-web` contient toute la logique reactive + DOM
   - Impossible de réutiliser le core sans WASM

3. **Documentation Web-only**
   - README ne mentionne que WASM
   - QWEN.md décrit une architecture Web-centrique

---

## 🏗️ Architecture Cible

```
domus/
├── domius-core/        # ← Platform-agnostic (100% Rust std)
│   ├── signal.rs       # Signal<T>, ReadSignal, WriteSignal
│   ├── effect.rs       # Effect, create_effect, TLS tracking
│   ├── scope.rs        # ScopeId, create_scope, dispose_scope
│   ├── runtime.rs      # Batch system, scheduler
│   ├── computed.rs     # Computed<T> (derived values)
│   └── lib.rs          # Public API
│
├── domius-web/         # ← Backend WASM/Web
│   ├── component.rs    # DomusComponent trait (web-sys)
│   ├── disposal.rs     # MutationObserver-based cleanup
│   ├── context.rs      # TypeId-based context (web-specific)
│   ├── router.rs       # URL-based routing
│   ├── list.rs         # Keyed list reconciliation
│   └── lib.rs          # web-sys integration
│
├── domius-desktop/     # ← Backend Tauri (NOUVEAU)
│   ├── component.rs    # DomusComponent trait (Tauri windows)
│   ├── disposal.rs     # Window event-based cleanup
│   ├── context.rs      # TypeId-based context (Tauri-specific)
│   ├── router.rs       # Route handling (Tauri-specific)
│   └── lib.rs          # Tauri integration
│
├── domius-macro/       # ← RSX parser (unchanged)
│   └── lib.rs          # Proc-macro, platform-agnostic
│
├── domius-cli/         # ← CLI scaffolding (unchanged)
│   └── main.rs         # Commands: new, add, build
│
└── examples/
    ├── hello-world-web/    # Web example
    └── hello-world-tauri/  # Tauri example (NOUVEAU)
```

---

## 📝 Étapes de Refactorisation

### Phase 1 : `domius-core` Platform-Agnostic

**Fichier : `domius-core/Cargo.toml`**

```toml
[package]
name = "domius-core"
version = "0.1.0"
edition = "2021"

[dependencies]
# AUCUNE dépendance WASM !
# 100% Rust std

[dev-dependencies]
# Tests natifs uniquement
```

**Changements requis :**

| Fichier | Action | Détails |
|---------|--------|---------|
| `Cargo.toml` | ✂️ Supprimer | `wasm-bindgen`, `web-sys`, `js-sys` |
| `src/lib.rs` | ✏️ Modifier | Retirer les imports WASM |
| `src/signal.rs` | ✅ OK | Déjà 100% Rust std |
| `src/effect.rs` | ✅ OK | Déjà 100% Rust std (TLS) |
| `src/scope.rs` | ✅ OK | Déjà 100% Rust std |
| `src/runtime.rs` | ✅ OK | Déjà 100% Rust std |
| `src/computed.rs` | ✅ OK | Déjà 100% Rust std |

**Code à modifier :**

Aucun code de `domius-core` n'utilise directement `web-sys`. Les tests sont déjà natifs.

**Validation :**
```bash
cd domius-core
cargo build          # Doit compiler sans WASM
cargo test           # Tests natifs doivent passer
```

---

### Phase 2 : `domius-web` Backend WASM

**Fichier : `domius-web/Cargo.toml`**

```toml
[package]
name = "domius-web"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
domius-core = { path = "../domius-core" }
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = [...] }
js-sys = "0.3"

[dev-dependencies]
wasm-bindgen-test = "0.3"
```

**Changements requis :**

| Fichier | Action | Détails |
|---------|--------|---------|
| `Cargo.toml` | ✏️ Modifier | Ajouter dépendance à `domius-core` |
| `src/lib.rs` | ✏️ Modifier | Importer depuis `domius-core` |
| `src/component.rs` | ⚠️ Adapter | Utiliser `domius_core::signal` au lieu de `crate::` |
| `src/disposal.rs` | ✅ OK | Déjà WASM-gated |
| `src/context.rs` | ⚠️ Adapter | Importer `Signal` depuis `domius-core` |
| `src/router.rs` | ✅ OK | Indépendant |
| `src/list.rs` | ✅ OK | Indépendant |

**Code à modifier :**

```rust
// Avant
use crate::signal::{signal, Signal};
use crate::effect::create_effect;

// Après
use domius_core::{signal, Signal, create_effect};
```

**Validation :**
```bash
cd domius-web
cargo check --target wasm32-unknown-unknown
wasm-pack build --target web
```

---

### Phase 3 : `domius-desktop` Backend Tauri

**Fichier : `domius-desktop/Cargo.toml`**

```toml
[package]
name = "domius-desktop"
version = "0.1.0"
edition = "2021"

[dependencies]
domius-core = { path = "../domius-core" }
domius-macro = { path = "../domius-macro" }
tauri = { version = "2.0", features = [] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[dev-dependencies]
# Tests natifs
```

**Structure à créer :**

```
domius-desktop/
├── Cargo.toml
├── src/
│   ├── lib.rs          # Entry point
│   ├── component.rs    # DomusComponent trait (Tauri)
│   ├── disposal.rs     # Window event-based cleanup
│   ├── context.rs      # Context API (Tauri)
│   ├── router.rs       # Route handling
│   └── event.rs        # Event bridge (Rust → Tauri)
```

**Concepts clés :**

1. **Composants** : Fenêtres Tauri ou composants web dans une WebView
2. **Disposal** : Écouter `window.close()` et événements de destruction
3. **Events** : Bridge Rust → Tauri commands
4. **DOM** : Utiliser les APIs Tauri ou WebView selon l'approche

**Exemple d'implémentation :**

```rust
// domius-desktop/src/component.rs
use domius_core::{Signal, ScopeId, create_scope};
use tauri::{AppHandle, Manager, WindowBuilder};

pub trait DomusComponent {
    type Props: Clone + Send + 'static;
    type State: 'static;

    fn setup(props: Self::Props) -> Self::State;
    fn render(state: &Self::State, app: &AppHandle) -> WindowBuilder;
}

pub fn mount_component<C: DomusComponent>(
    app: &AppHandle,
    props: C::Props,
) {
    let state = C::setup(props);
    let window = C::render(&state, app);
    // Attacher le cleanup au window.close()
}
```

**Validation :**
```bash
cd domius-desktop
cargo check
cargo build
```

---

### Phase 4 : Mise à Jour Documentation

**Fichiers à modifier :**

| Fichier | Action | Détails |
|---------|--------|---------|
| `README.md` | ✏️ Modifier | Expliquer architecture multi-cibles |
| `QWEN.md` | ✏️ Modifier | Mettre à jour structure workspace |
| `PROJECT.md` | ✏️ Modifier | Ajouter roadmap Tauri |
| `CHANGELOG.md` | ✏️ Ajouter | Noter breaking changes |

**Nouveau README — Section Architecture :**

```markdown
## Architecture Multi-Plateformes

Domus utilise une architecture en couches :

```
┌─────────────────────────────────────┐
│         Votre Application           │
├─────────────────────────────────────┤
│  domius-web  │  domius-desktop      │  ← Backends
├─────────────────────────────────────┤
│           domius-core               │  ← Core (100% Rust)
├─────────────────────────────────────┤
│           domius-macro              │  ← RSX parser
└─────────────────────────────────────┘
```

### Choisir son Backend

| Backend | Cible | Use Case |
|---------|-------|----------|
| `domius-web` | WASM/Web | Applications web, PWA |
| `domius-desktop` | Tauri | Applications desktop (Windows, macOS, Linux) |
```

---

### Phase 5 : Exemples et Tests

**À créer :**

```
examples/
├── hello-world-web/        # Web/WASM example
│   ├── Cargo.toml
│   ├── index.html
│   └── src/
│       └── lib.rs
│
└── hello-world-tauri/      # Tauri example
    ├── Cargo.toml
    ├── tauri.conf.json
    └── src/
        └── main.rs
```

**Tests :**

```bash
# Tests natifs (core)
cargo test -p domius-core

# Tests WASM
cargo test -p domius-web --target wasm32-unknown-unknown

# Tests desktop
cargo test -p domius-desktop
```

---

## 📅 Roadmap

| Phase | Tâches | Statut |
|-------|--------|--------|
| 1. Core agnostic | Retirer deps WASM, valider tests | ⏳ Pending |
| 2. Web backend | Adapter imports, valider build | ⏳ Pending |
| 3. Desktop skeleton | Créer crate Tauri, implémenter traits | ⏳ Pending |
| 4. Documentation | Mettre à jour README, QWEN.md | ⏳ Pending |
| 5. Exemples | hello-world-web, hello-world-tauri | ⏳ Pending |

---

## 🔍 Breaking Changes

### Pour les Utilisateurs Actuels

**Avant :**
```toml
[dependencies]
domius-web = "0.1"
```

**Après :**
```toml
[dependencies]
domius-core = "0.2"  # Core séparé
domius-web = "0.2"   # Web backend
```

### Pour le Code

**Avant :**
```rust
use domius_web::signal;
```

**Après :**
```rust
use domius_core::signal;
```

---

## ✅ Critères de Succès

- [ ] `domius-core` compile sans dépendances WASM
- [ ] Tous les tests natifs passent (`cargo test -p domius-core`)
- [ ] `domius-web` build en WASM (`wasm-pack build`)
- [ ] `domius-desktop` compile et lance une window Tauri
- [ ] Documentation à jour (README, QWEN.md)
- [ ] Exemples fonctionnels (web + desktop)

---

## 📚 Références

- [Tauri v2 Documentation](https://v2.tauri.app/)
- [Rust TLS (Thread Local Storage)](https://doc.rust-lang.org/std/macro.thread_local.html)
- [WASM Bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
