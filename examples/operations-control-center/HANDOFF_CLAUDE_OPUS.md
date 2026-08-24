# Relais pour Claude Opus

Lis d'abord `examples/operations-control-center/README.md`. Ce fichier donne l'état du chantier au commit `f29acab`; s'il ne correspond plus à `HEAD`, vérifie l'historique avant de modifier quoi que ce soit.

## Mission

Terminer **Operations Control Center**, l'exemple complexe attribué à Codex. Il doit exercer les API Domius qui lui sont réservées, sans réintroduire Tauri ni Dioxus. Domius est leur alternative, pas une surcouche construite dessus.

Travaille par petits lots. Pour chaque composant encore vide : test rouge ciblé, implémentation dans `domius-web`, test navigateur, commit; intègre ensuite le composant à l'exemple dans un second commit. Ne copie pas une implémentation dans `examples/`.

## État Git au passage de relais

- Branche : `codex/operations-control-center`
- `HEAD` : `f29acab`
- Suivi distant observé : `origin/codex/operations-control-center` à `b5950a6`
- La branche locale possède 9 commits non poussés.
- Le dossier non suivi `.acp-team/` existait déjà au moment du relais. Ne l'ajoute pas et ne le supprime pas sans en connaître le propriétaire.
- Remote : `https://github.com/medyll/domus.git`

Ne pousse pas ces commits sans accord explicite du propriétaire pour exporter ce code vers ce remote. Une fois l'accord obtenu : pousse la branche, ouvre la PR, attends les contrôles, puis fusionne seulement si tout passe.

## Ce qui fonctionne déjà

Les routes `/overview`, `/services/:id`, `/incidents` et `/reports` montent dans l'application WASM. Les données locales à graine fixe produisent 6 services, 48 incidents et 360 métriques.

Le travail récent a ajouté et testé :

- `Charts`, puis les rapports de fiabilité;
- `InfiniteScroll`, utilisé par un flux de 48 incidents avec clés stables;
- `DataGrid`, utilisé en lecture seule sur les 360 mesures;
- `PivotTable`, avec les sept agrégateurs, les totaux et le repli accessible;
- `Heatmap`, avec axes, cellules absentes, sélection et trois familles d'échelles.

La page `/reports` affiche maintenant la courbe de débit, les erreurs moyennes, la distribution des incidents, la grille brute, le tableau croisé par fenêtres de 20 minutes et une heatmap 6 × 6 par fenêtres de 10 minutes. Les trois dernières vues lisent la même collection de métriques.

Commits locaux à préserver :

```text
f29acab feat(example): add error activity heatmap
8b6ffc9 feat(web): implement accessible Heatmap
d4dd2b8 feat(example): add throughput pivot report
75ba0f7 feat(web): implement PivotTable
c6e6540 feat(example): add raw metrics grid
8258bf4 feat(web): implement editable DataGrid
dcf0c61 feat(example): add progressive incident feed
d91f1bf fix(web): make scroll completion reactive
dc659de feat(web): implement InfiniteScroll
```

## Reprise immédiate

- [ ] Implémenter `domius-web/src/components/pro/scatter_plot.rs` avec un SVG accessible, domaines explicites ou calculés, quadrillage optionnel, labels, tailles et couleurs par point.
- [ ] Ajouter des tests WASM Firefox dédiés à `ScatterPlot`; couvrir les domaines constants et les données vides, pas seulement le cas nominal.
- [ ] Committer la bibliothèque seule, par exemple `feat(web): implement ScatterPlot`.
- [ ] Ajouter sur `/reports` une corrélation latence/taux d'erreur ou débit/taux d'erreur issue des métriques existantes.
- [ ] Vérifier l'intégration dans Chromium, puis créer un second commit.
- [ ] Traiter ensuite `watermark.rs`, l'intégrer à la zone exportable du rapport, et garder le même découpage bibliothèque/exemple.

Après ces deux composants, ferme les exigences visibles de `/reports` : état vide, état d'échec via `Result`, puis QR code réellement décodable vers l'URL affichée. Le QR code actuel ne produit qu'un motif de démonstration.

## Travail restant dans le mandat Codex

La table « API Domius attribuée à cet exemple » du README fait foi. Les points les plus faciles à oublier sont :

- [ ] `computed`, `create_effect`, `batch` et un test de destruction de scope;
- [ ] `diff_keys`, `DiffOp` et `ListPatch` sur un vrai retrait/réordonnancement d'incidents;
- [ ] skeleton initial, spinner de transition, progression de résolution et toast d'acquittement;
- [ ] heatmap sur `/overview`, tour d'aide, tooltip, affix et retour en haut;
- [ ] état de route inconnue avec retour utilisable vers l'accueil;
- [ ] parcours navigateur couvrant les huit tests d'acceptation du README.

Ne tente pas de réparer tous les autres `todo!` du dépôt. `diff_viewer`, `kanban`, `gantt` et les autres composants non cités dans le mandat appartiennent aux deux autres exemples, sauf preuve contraire dans leur README.

## Commandes de preuve

Le dernier état connu passe avec :

```powershell
cargo clippy --manifest-path examples/operations-control-center/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path examples/operations-control-center/Cargo.toml
wasm-pack build examples/operations-control-center --target web
wasm-pack test --headless --firefox domius-web --test pivot_table_tests
wasm-pack test --headless --firefox domius-web --test heatmap_tests
```

Les tests navigateur observés avant le relais :

- grille : 360 lignes, 1 440 cellules, zéro erreur de page;
- pivot : 6 lignes, 3 fenêtres, totaux présents, repli fonctionnel;
- heatmap : 36 cellules renseignées, 4 niveaux de couleur, zéro erreur de page;
- flux d'incidents : 48 clés uniques, arrêt propre sur « All items loaded ».

Avant une PR, lance aussi `cargo test --workspace --locked` et tous les tests WASM concernés. Si Windows refuse la création du dossier temporaire de `wasm-bindgen` ou le lancement du navigateur (`EPERM`), relance la même commande avec l'autorisation adaptée; ne change pas le code pour contourner cette restriction locale.

## Règles de conception

- Utilise des éléments HTML natifs quand ils portent déjà la bonne sémantique : `table`, titres, boutons et liens.
- N'ajoute pas de styles inline. Expose les états et jetons avec des classes ou attributs `data-*` compatibles avec `@medyll/css-base`.
- Garde chaque vue utilisable au clavier et annonce les changements asynchrones avec ARIA.
- N'ajoute aucune dépendance Tauri ou Dioxus, même pour gagner du temps sur le desktop.
- Ne touche pas aux changements non suivis dont tu n'es pas l'auteur.

## Prompt court à donner à Claude Opus

```text
Lis entièrement examples/operations-control-center/README.md puis
examples/operations-control-center/HANDOFF_CLAUDE_OPUS.md. Reprends au HEAD actuel,
vérifie l'état Git, puis continue à partir de ScatterPlot. Travaille de façon autonome,
teste dans un vrai navigateur et fais un commit atomique après chaque lot. Ne réintroduis
ni Tauri ni Dioxus et ne touche pas à .acp-team/.
```
