# État de reprise

Lis d'abord `README.md`, puis contrôle `git status --short`. Le dossier non suivi `.acp-team/` appartient à un autre outil : ne l'ajoute pas, ne le supprime pas.

## Position du projet

Operations Control Center couvre maintenant les quatre routes prévues et les huit scénarios d'acceptation. Le scénario de navigation part de `/overview`, attend les cartes créées après le skeleton, clique sur `/services/svc-01`, puis vérifie l'URL, le titre et la nouvelle page sans rechargement. Un test séparé couvre le bouton Retour.

Les rapports montrent cinq lectures de la même fenêtre de métriques : table par service, grille des 360 mesures, pivot, heatmap 6 × 6 et scatter de 36 points. Le QR code se décode vers l'adresse imprimée à côté de lui.

## Corrections après la première livraison

- `Effect::execute` remet sa closure en place même si elle panique.
- La réconciliation associe chaque occurrence d'une clé répétée; réduire `a, a` vers `a` retire bien le second nœud.
- `computed_in_scope` lie les valeurs calculées au cycle de vie d'une vue. La page Overview l'utilise pour ses agrégats.
- Les écouteurs de scroll quittent la fenêtre ou le conteneur quand le `ViewScope` disparaît.
- Le délai d'une tooltip s'annule sur `mouseleave`, `focusout` ou Échap.
- Le tour modal déplace le focus, le garde dans ses actions, décrit son contenu avec ARIA et rend le focus à l'élément d'ouverture.
- La navigation intercepte aussi les liens ajoutés par un effet asynchrone et écoute `popstate`.
- `index.html` charge la version publiée `@medyll/css-base@0.7.10`; `styles.css` donne une apparence lisible aux graphiques, cellules, panneaux, listes, infobulles et au tour.

## Vérifications à refaire avant une publication

```powershell
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
wasm-pack build examples/operations-control-center --target web
wasm-pack test --headless --firefox domius-web
wasm-pack test --headless --firefox examples/operations-control-center --test browser
```

Le contrôle visuel final doit aussi ouvrir l'application construite depuis `/`, déclencher le tour, suivre le premier lien service, revenir en arrière et ouvrir Reports. Les valeurs attendues sont six lignes dans `#metric-table`, 36 cellules de heatmap colorées et 36 points de scatter colorés.

## Contraintes

Domius reste une alternative à Tauri et Dioxus. N'ajoute aucune dépendance vers ces projets. Garde les changements en petits commits, teste les comportements dans un vrai navigateur et ne pousse rien sans demande explicite du propriétaire.
