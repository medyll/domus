# Operations Control Center

## Mandat

**Agent responsable : Codex**

Construire une application web de supervision qui permet de suivre des services, des incidents et des indicateurs de production. Ce dossier ne doit pas devenir une galerie de composants. Chaque écran doit répondre à une action réelle : détecter un problème, comprendre son origine, suivre sa résolution ou produire un rapport.

L'exemple sert aussi de chantier d'intégration pour le runtime réactif et les composants de données. Lorsqu'un composant attribué à cet exemple contient encore un `todo!`, il faut terminer le composant dans `domius-web`, écrire ses tests, puis l'utiliser ici. Une copie locale du composant dans l'exemple n'est pas acceptée.

## Place dans la couverture globale

```text
domius-core
    |
    v
runtime réactif + état partagé + listes indexées
    |
    v
Operations Control Center
    |
    +--> vue temps réel
    +--> enquête sur incident
    +--> rapports analytiques
```

Les deux autres applications couvrent l'édition riche et le desktop :

- `../collaborative-media-studio/README.md` : composants, macro RSX, formulaires, média et hooks ;
- `../desktop-project-command/README.md` : runtime desktop Domius, événements natifs, CLI et planification de projet.

## Parcours attendu

L'utilisateur arrive sur `/overview` et voit la santé des services, les alertes ouvertes et les délais avant dépassement de SLA. Il peut ensuite ouvrir `/services/:id`, filtrer les événements d'un service, réordonner une liste suivie et consulter les tendances. La route `/incidents` contient un flux paginé avec chargement progressif. Enfin, `/reports` permet de croiser les mêmes données dans une table filtrable, une grille, une heatmap, un nuage de points et un tableau croisé.

Les données restent locales et déterministes. Prévoir un générateur de jeu de données avec une graine fixe plutôt qu'un faux serveur réseau.

## Écrans

### `/overview`

- barre de navigation, résumé des incidents et grille de statistiques ;
- badges d'état, courbe de charge, heatmap d'activité et message défilant pour une alerte critique ;
- compte à rebours du SLA et cartes de services ;
- skeleton au chargement initial, puis contenu réel ;
- tour rapide accessible depuis l'aide contextuelle.

### `/services/:id`

- breadcrumbs et ancres vers les sections de la page ;
- chronologie des déploiements et incidents ;
- tableau triable des événements avec pagination ;
- tooltip sur les valeurs abrégées ;
- bouton de retour en haut et bloc fixé lors du défilement.

### `/incidents`

- filtres stockés dans le contexte de l'application ;
- flux utilisant `InfiniteScroll` ;
- progression de résolution et spinner pendant les transitions ;
- ajout, retrait et réordonnancement par clés afin d'exercer `diff_keys` et `ListPatch` ;
- toast après acquittement d'un incident, fourni par l'intégration finale si nécessaire.

### `/reports`

- `DataTable` pour comparer les agrégats par service et `DataGrid` pour explorer les données brutes ;
- `PivotTable`, `ScatterPlot` et `Heatmap` alimentés par la même source ;
- état vide et état d'échec rendus avec `Result` ;
- filigrane sur la zone exportable ;
- QR code pointant vers une URL locale de rapport.

## API Domius attribuée à cet exemple

### Runtime `domius-core`

- `signal` et `Signal` pour les filtres, la sélection et les données ;
- `computed` et `Computed` pour les totaux, tendances et agrégations ;
- `create_effect` et `Effect` pour synchroniser le titre, les filtres et les vues ;
- `batch` pour appliquer plusieurs filtres sans rendus intermédiaires ;
- `create_scope`, `dispose_scope` et `ScopeId` dans au moins un test explicite de cycle de vie.

### Infrastructure web

- `domius_web::init` et la destruction automatique des scopes ;
- `DomiusPage`, `Router` et `RoutePattern`, y compris un paramètre `:id` et une route inconnue ;
- `provide_context`, `use_context`, `has_context`, `remove_context` et `clear_all_contexts` ;
- `diff_keys`, `DiffOp` et `ListPatch` sur le flux d'incidents.

### Composants réservés à ce dossier

| Famille | Modules à intégrer |
|---|---|
| Primitives | `affix`, `backtop`, `card`, `countdown`, `divider`, `grid`, `icon`, `qrcode`, `scrolltext`, `tag`, `text`, `typography` |
| Navigation | `anchor`, `breadcrumbs`, `navbar`, `pagination`, `tabs` |
| Feedback | `infinite_scroll`, `progress`, `skeleton`, `spinner`, `tooltip` |
| Données | `badge`, `charts`, `statistic`, `table`, `timeline` |
| Pro | `data_grid`, `heatmap`, `pivot_table`, `result`, `scatter_plot`, `watermark` |

`qrcode` génère actuellement un motif de démonstration ; ce chantier doit produire un QR code lisible. Plusieurs composants de données et `pro` arrêtent encore l'exécution avec `todo!`. Leur correction fait partie du travail, dans les fichiers de la bibliothèque.

## Architecture cible

```text
src/
  lib.rs                 démarrage WASM et montage du shell
  app.rs                 contexte global et déclaration des routes
  data/
    fixtures.rs          données déterministes
    model.rs             Service, Incident, Metric, Deployment
  pages/
    overview.rs
    service_detail.rs
    incidents.rs
    reports.rs
  components/
    app_shell.rs
    service_health.rs
    incident_feed.rs
    report_workspace.rs
  state/
    filters.rs
    monitoring.rs
tests/
  browser.rs
```

Ne pas placer toute l'application dans `lib.rs`. Les composants applicatifs assemblent les composants Domius ; ils ne réécrivent pas leurs comportements.

## Ordre de travail

1. Faire l'inventaire des API attribuées et ouvrir un test rouge pour chaque implémentation manquante.
2. Monter quatre routes avec `DomiusPage`, puis partager les filtres par contexte.
3. Connecter le runtime réactif aux cartes, statistiques et listes indexées.
4. Terminer les composants de données nécessaires, avec tests natifs ou navigateur selon le cas.
5. Ajouter les vues analytiques `pro`, puis les états chargement, vide et erreur.
6. Écrire les parcours navigateur et documenter les commandes de lancement.

## Commandes de vérification

```powershell
cargo test --workspace --locked
cargo test --manifest-path examples/operations-control-center/Cargo.toml
cargo clippy --manifest-path examples/operations-control-center/Cargo.toml --all-targets -- -D warnings
wasm-pack build examples/operations-control-center --target web
wasm-pack test --headless --firefox examples/operations-control-center --test browser
```

La dernière commande exécute les tests d'acceptation ci-dessous dans Firefox.
Le premier test clique réellement sur `/services/svc-01`; un autre utilise le
bouton Retour pour vérifier que l'URL, le titre et le DOM reviennent ensemble.

L'interface charge `@medyll/css-base` 0.7.10 depuis jsDelivr, puis applique les
règles propres au centre d'opérations dans `styles.css`. Pour une vérification
sans accès réseau, il faut servir une copie locale de `dist/app.css` et modifier
le lien de `index.html`.

## Contrat de qualité

- aucun `todo!`, `unimplemented!` ou panic volontaire sur un parcours visible ;
- aucune manipulation directe de `web_sys` hors démarrage, adaptateur Domius ou test bas niveau ;
- chaque route reste utilisable au clavier et conserve un focus visible ;
- le jeu de données contient assez de lignes pour exercer tri, filtre, pagination et chargement progressif ;
- les calculs dérivés changent après une mise à jour groupée avec `batch` ;
- le retrait d'une page ou d'un bloc détruit ses effets, preuve automatisée à l'appui ;
- `cargo test --workspace --locked`, les tests WASM et le build de cet exemple passent.

## Tests d'acceptation

1. Passer de `/overview` à un service change la route, le titre et le contenu sans rechargement complet.
2. Appliquer deux filtres dans un `batch` déclenche une seule mise à jour observable des agrégats.
3. Réordonner puis supprimer des incidents conserve les nœuds portant la même clé.
4. Faire défiler le flux charge la page suivante une seule fois.
5. Les vues table, grille, pivot, heatmap et scatter utilisent le même état filtré.
6. Une route absente rend un état `Result` exploitable et permet de revenir à l'accueil.
7. Le QR code de rapport se décode vers l'URL affichée.
8. Retirer le conteneur d'une page arrête ses effets.

## Limites de propriété

Codex possède ce dossier, `domius-core`, le routeur, le contexte web, la réconciliation de listes et les modules de composants listés plus haut. Les fichiers centraux de réexport (`domius-web/src/lib.rs` et `components/mod.rs`) sont modifiés lors d'une passe d'intégration unique, après les travaux des trois agents, afin d'éviter des conflits inutiles.

## Journal de décisions

Ajouter ici toute décision qui change une API publique ou déplace une fonctionnalité vers un autre exemple. Une entrée tient sur une ligne : date, décision, fichiers concernés.

- 2026-08-24 : `qrcode` encode réellement sa valeur ; `QRCodeProps.svg` disparaît (la sortie est toujours du SVG) et `qrcode_matrix` expose la grille sans DOM. Ajout de `qrcode` en dépendance et de `rqrr` en dépendance de développement pour prouver le décodage — `domius-web/Cargo.toml`, `domius-web/src/components/primitives/qrcode.rs`, `domius-web/tests/qrcode_tests.rs`.
- 2026-08-24 : `ResultProps.extra_actions: Option<String>` devient `actions: Vec<ResultAction>` et `Result::not_found` prend ses actions en argument ; le HTML brut injecté par `set_inner_html` disparaît au profit de vrais liens atteignables au clavier — `domius-web/src/components/pro/result.rs`, `domius-web/tests/result_tests.rs`.
