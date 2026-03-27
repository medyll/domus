# Plan : Tests avancés + corrections du moteur réactif

## Objectif
Implémenter 8 catégories de tests pour couvrir la profondeur du système réactif et corriger les bugs révélés.

---

## Phase 1 : Tests qui passent déjà (documentent le bon comportement)

### 1. Scope + signal cross-lifetime ✅
- **Fichier** : `domius-core/src/scope.rs` (ajouter tests)
- **Tests** :
  - Signal survit à la disposition du scope qui l'a créé
  - Effect disposé ne reçoit plus de notifications
  - Pas de subscribers "fantômes" après disposal
  - Signal dans parent scope, effect dans child scope
- **Status** : Ces tests **passent déjà**, ils documentent le nettoyage correct

### 2. Stabilité sous charge ✅
- **Fichier** : Nouveau `domius-core/src/benchmarks.rs` (tests de charge)
- **Tests** :
  - 1000 signaux, 1000 effets, mutations aléatoires
  - Compter exact du nombre d'exécutions d'effet
  - Vérifier pas d'exécutions redondantes
- **Status** : Passe si l'implémentation est correcte

---

## Phase 2 : Corrections + tests (bugs critiques)

### 3. Graphe de dépendances dynamique ❌→✅
- **Problem** : `effect.execute()` ne re-track pas les dépendances (pas de `RUNNING_EFFECT` en TLS)
- **Fix** :
  1. Renommer `effect.execute()` → `execute_without_tracking()`
  2. Créer `effect.run_with_tracking()` qui remet `RUNNING_EFFECT` en TLS
  3. Dans `runtime::flush_queue()`, appeler `run_with_tracking()` au lieu de `execute()`
  4. **Bonus** : Implémenter un mécanisme de "clear dependencies at entry" pour vraiment tracker dynamiquement
- **Tests** : `effect_dynamic_dependency_switching` — un effet qui lit A ou B selon une condition
- **File** : `domius-core/src/tests_dynamic_deps.rs`

### 4. Diamonds (convergence sans duplication) ❌→✅
- **Problem** : Pas de topological sort, effet D déclenché 2x quand A change et notifie B+C
- **Fix** :
  1. Numéroter les effets par "génération" ou "epoch"
  2. Dans `flush_queue()`, grouper par génération et exécuter par niveaux
  3. OU : utiliser un Set de "noeuds à exécuter cette génération"
- **Tests** :
  - `effect_diamond_convergence_once` — A→B,C→D, vérifier D exécuté 1x
  - `effect_multiple_sources_one_run` — plusieurs sources, 1 effet qui les lit
- **File** : `domius-core/src/tests_diamond.rs`

### 5. Batch imbriqués ❌→✅
- **Problem** : Nested `batch()` clear `IN_BATCH` à la sortie du batch interne
- **Fix** :
  1. Remplacer le booléen `IN_BATCH` par un compteur `BATCH_DEPTH` (usize)
  2. `batch(F)` : `BATCH_DEPTH += 1`, run F, `BATCH_DEPTH -= 1`, flush si depth==0
  3. `schedule_effect()` : defer si `BATCH_DEPTH > 0`
- **Tests** :
  - `nested_batch_single_flush` — deux batch imbriqués, flush une seule fois
  - `triple_nested_batch` — trois niveaux
- **File** : `domius-core/src/tests_batch.rs`

### 6. Ré-entrée (cycle detection) ❌→✅
- **Problem** : Un effect qui appelle `signal.set()` sur un signal qu'il lit → boucle infinie
- **Fix** (options) :
  - **Option A** (strict) : Détection de cycle — marquer l'effet comme "dirty" au lieu de re-scheduler immédiatement
  - **Option B** (génération) : Utiliser l'epoch/génération pour ignorer les re-schedules de la même génération
  - **Option C** (simple) : Compter les re-entrées, panic si > N
- **Recommandé** : Option B (s'intègre avec le fix du diamond)
- **Tests** :
  - `effect_reentrancy_no_infinite_loop` — effet qui écrit un signal qu'il lit
  - `effect_reentrancy_uses_latest_value` — la valeur finale est celle attendue
- **File** : `domius-core/src/tests_reentrancy.rs`

### 7. Glitch freedom (cohérence intermédiaire) ❌→✅
- **Problem** : Sans topo sort, observer D peut voir C dans un état intermédiaire
  - Exemple : A=2, B=A*2 (doit être 4), C=A+B (doit être 6)
  - Sans topo sort : A change→B exécute→notify C; C voit B=4, C=6. Mais A update B avant d'update C en cascade
- **Fix** : Topo sort dans `flush_queue()` — exécuter par niveaux de dépendance
- **Tests** :
  - `glitch_free_convergence` — A=signal, B=derived(A), C=derived(A+B), observer de C ne voit que états valides
  - `derived_signal_propagation` — chaîne de A→B→C, pas d'état intermédiaire glitchy
- **File** : `domius-core/src/tests_glitch_freedom.rs`

### 8. Effets imbriqués / Computed (dérivés) ❌→✅
- **Problem** : Pas de primitive `computed()` native
- **Fix** :
  1. Créer `Computed<T>` (ou `Derived<T>`) qui encapsule un signal + un effect
  2. `computed(fn)` : créer un signal, créer un effect qui appelle `fn()` et `set()` le signal
  3. Lazy recompute : l'effect ne déclenche que si une dépendance a changé
- **Tests** :
  - `computed_basic_lazy_evaluation` — computed se recalcule seulement si input change
  - `computed_chain_a_b_c` — chaîne A→B(A*2)→C(B+1), vérifier propagation correcte
  - `computed_sharing` — deux effects lisent le même computed, vérifier pas de double recompute
- **File** : `domius-core/src/computed.rs` (NEW) + `domius-core/src/tests_computed.rs`

---

## Ordre d'implémentation

1. **Batch imbriqués** (Fix le compteur `BATCH_DEPTH`) — plus simple, dépendance pour autres fixes
2. **Graphe dépendances** (Fix `execute()` avec re-tracking) — fondation pour diamonds
3. **Diamonds + Glitch** (Topo sort dans flush) — fixes critiques
4. **Ré-entrée** (Intégrer epoch dans les fixes précédentes)
5. **Computed** (Nouvelle primitive, utilise les fixes)
6. **Tests scope + charge** (Valider la stabilité)

---

## Fichiers à créer/modifier

| Fichier | Action | Description |
|---------|--------|-------------|
| `domius-core/src/runtime.rs` | Modify | Ajouter `BATCH_DEPTH`, topo sort dans `flush_queue()`, epoch tracking |
| `domius-core/src/effect.rs` | Modify | Ajouter `run_with_tracking()`, supporter re-tracking des dépendances |
| `domius-core/src/computed.rs` | Create | Nouvelle primitive `Computed<T>` |
| `domius-core/src/lib.rs` | Modify | Re-export `Computed`, `batch` |
| `domius-core/src/tests_dynamic_deps.rs` | Create | Tests dépendances dynamiques |
| `domius-core/src/tests_diamond.rs` | Create | Tests convergence sans duplication |
| `domius-core/src/tests_batch.rs` | Create | Tests batch imbriqués |
| `domius-core/src/tests_reentrancy.rs` | Create | Tests ré-entrée + cycle |
| `domius-core/src/tests_glitch_freedom.rs` | Create | Tests cohérence |
| `domius-core/src/tests_computed.rs` | Create | Tests computed |
| `domius-core/src/scope.rs` | Modify | Ajouter tests scope + lifetime |
| `domius-core/src/benchmarks.rs` | Create | Tests de charge |

---

## Résumé des fixes

| Bug | Fix | Fichier | Complexité |
|-----|-----|---------|-----------|
| Batch imbriqués | `BATCH_DEPTH` (int au lieu de bool) | runtime.rs | ⭐ |
| Graphe dépendances | Re-tracking en re-exécution | effect.rs + runtime.rs | ⭐⭐ |
| Diamonds | Topo sort / epoch | runtime.rs | ⭐⭐⭐ |
| Ré-entrée | Epoch blocking | runtime.rs | ⭐ (inclus dans diamonds) |
| Glitch freedom | Topo sort | runtime.rs | ⭐⭐⭐ (inclus dans diamonds) |
| Computed | Nouvelle structure | computed.rs | ⭐⭐ |

---

## Validation

- CI : Tous les tests passent ✅
- Tests de régression : Signal, Effect, Scope existants toujours ✅
- Couverture : 8 catégories + cas limites ✅
