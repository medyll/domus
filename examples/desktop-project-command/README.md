# Desktop Project Command

## Mandat

**Agent responsable : OpenCode**

Construire une application desktop Domius de pilotage de projets avec plusieurs fenêtres : portefeuille, planification, détail d'une tâche et préférences. Elle doit vérifier la couche `domius-desktop`, les événements entre fenêtres, la destruction des scopes, le CLI et les composants avancés de planification.

Domius constitue ici le framework applicatif desktop. L'exemple ne doit dépendre ni de Tauri ni de ses conventions, commandes, plugins ou fichiers de configuration. Une éventuelle bibliothèque de bas niveau comme `wry` ou `tao` reste cachée derrière les types Domius.

Le résultat attendu est une application desktop lançable, pas une page web emballée sans intégration native. Ouvrir une fenêtre, transmettre un événement, fermer la fenêtre et libérer son scope doivent former un seul parcours testé.

## Place dans la couverture globale

```text
domius-cli ---> squelette du projet
                    |
domius-desktop ---> runtime + fenêtres + commandes + événements
                    |
                    v
         Desktop Project Command
                    |
             interface Domius web
```

`operations-control-center` possède le runtime, le routeur et les vues analytiques. `collaborative-media-studio` possède la macro, les formulaires, les hooks et les médias.

## Parcours attendu

Le gestionnaire ouvre le portefeuille, choisit un projet et passe de la vue Kanban au Gantt. Il peut ouvrir le détail d'une tâche dans une deuxième fenêtre, modifier son état, puis observer la mise à jour dans la fenêtre principale via un événement typé. Les préférences vivent dans le contexte desktop et restent disponibles dans chaque fenêtre.

## Fenêtres et écrans

### Fenêtre `portfolio`

- liste des projets et accès au dernier projet ouvert ;
- palette de commandes pour ouvrir un projet, une tâche ou les préférences ;
- modal de création de projet ;
- toast après création, archivage ou erreur.

### Fenêtre `planner-{project_id}`

- barre de panneaux redimensionnables : arborescence courte, planification, inspecteur ;
- bascule segmentée fournie par l'autre exemple ou contrôle simple entre Kanban et Gantt ;
- splitter horizontal pour le journal d'activité ;
- sélection d'une carte ouvrant le détail dans une fenêtre dédiée ;
- changements publiés par `EventBridge`.

### Fenêtre `task-{task_id}`

- titre et taille calculés à partir des props du `DomiusDesktopComponent` ;
- édition minimale de la tâche ;
- émission d'un événement `Custom` lors de la sauvegarde ;
- fermeture entraînant `cleanup_component_scope` et arrêt des effets associés.

### Fenêtre `settings`

- lecture et écriture du contexte desktop ;
- préférences de densité, fenêtre par défaut et raccourcis ;
- événement de focus et de blur visible dans un journal de diagnostic local.

## API Domius attribuée à cet exemple

### Correction d'architecture préalable

`domius-desktop/Cargo.toml` dépend actuellement de `tauri`, et plusieurs commentaires présentent Domius comme un backend Tauri. Cette direction contredit le rôle du projet. Avant de construire l'exemple, OpenCode doit :

- retirer la dépendance publique à Tauri et le vocabulaire associé ;
- définir des types Domius pour le runtime, les fenêtres, les commandes et les événements ;
- isoler le moteur de fenêtre ou de webview derrière un module privé ;
- faire du cycle de vie Domius la source de vérité pour les scopes ;
- exposer une boucle `run` et un builder d'application utilisables sans outil externe ;
- conserver les API publiques déjà utiles lorsqu'elles ne portent aucune hypothèse Tauri.

L'ancien dossier `examples/hello-world-tauri` constitue une application Tauri réelle et ne peut pas rester l'exemple desktop de Domius. Il doit être remplacé par un exemple utilisant le runner Domius, puis retiré dès que la nouvelle application couvre son petit compteur. Le README racine doit être corrigé dans la même passe : architecture, installation, commandes, tableau des crates et feuille de route.

Critère de sortie de cette étape zéro : `cargo tree -p domius-desktop` ne contient plus `tauri`, le workspace ne référence plus `tauri-build` ni `tauri-macros`, et aucun exemple actif ne demande d'installer le CLI Tauri.

La couche minimale visée peut reposer sur `wry` et `tao`, mais les exemples et les utilisateurs ne doivent jamais importer ces crates directement. Si une autre base plus légère répond mieux aux tests multi-plateformes, consigner le choix dans le journal de décisions avant l'implémentation.

### `domius-desktop`

- `DomiusDesktopComponent` avec `Props`, `State`, `title`, `label`, `window_size` et `url` ;
- `build_window_config`, `get_component_url`, `ComponentScope` et `cleanup_component_scope` ;
- `provide_context`, `use_context`, `has_context` et `remove_context` ;
- `DesktopEvent`, `EventHandler`, `EventBridge::new`, `on` et `emit` ;
- `use_event_signal` avec une vraie souscription ;
- `init`, `init_event_listeners` et `on_window_close`.

`use_event_signal` et l'écoute globale de fermeture contiennent encore des marqueurs temporaires. Les terminer dans `domius-desktop` fait partie de ce chantier. Les tests doivent prouver qu'un handler reçoit l'événement attendu et qu'une fenêtre fermée ne laisse aucun scope actif.

Ajouter au runtime desktop les capacités que l'exemple exige et que la bibliothèque ne possède pas encore :

- `DomiusDesktopApp` ou un builder équivalent pour enregistrer fenêtres, commandes et état partagé ;
- `WindowConfig` et `WindowHandle` appartenant à Domius ;
- registre de commandes typées entre l'interface et le processus natif ;
- abonnement avec garde de désinscription pour le bus d'événements ;
- démarrage, arrêt propre et propagation des erreurs.

### `domius-cli`

- `domius new project` pour produire la base de l'exemple dans un répertoire temporaire de test ;
- `domius add component` et `domius add page` ;
- conversion snake case et PascalCase ;
- génération d'un scope CSS stable, attribut de scope et transformation CSS ;
- sortie générée compilable sans `todo!` dans les méthodes `render`.

Le CLI génère aujourd'hui des méthodes `render` inachevées. Corriger les templates dans `domius-cli/src/scaffold.rs`, puis ajouter un test qui écrit le projet dans un dossier temporaire et lance une vérification de compilation.

### Composants réservés à ce dossier

| Famille | Modules à intégrer |
|---|---|
| Feedback | `modal`, `toast` |
| Pro | `command_palette`, `gantt`, `kanban`, `resizable_panels`, `splitter` |

Ces composants couvrent les derniers modules `pro` non attribués aux deux exemples web. La fenêtre de planification doit les utiliser ensemble dans un flux cohérent, avec sauvegarde et propagation d'événements.

## Architecture cible

```text
src-desktop/
  src/
    main.rs
    runtime.rs
    commands.rs
    events.rs
    windows.rs
  domius.desktop.toml
src/
  lib.rs
  app.rs
  model/
    project.rs
    task.rs
    preferences.rs
  windows/
    portfolio.rs
    planner.rs
    task_detail.rs
    settings.rs
  components/
    project_kanban.rs
    project_gantt.rs
    command_palette.rs
tests/
  desktop_events.rs
  window_lifecycle.rs
```

Le dossier peut adapter cette structure au moteur natif retenu, mais il doit conserver une séparation nette entre runtime Domius, commandes natives, modèle, fenêtres et composants web.

## Ordre de travail

1. Extraire Tauri de `domius-desktop` et poser les types publics du runtime Domius.
2. Corriger le CLI afin qu'un projet et ses ajouts compilent dès leur génération.
3. Créer une première fenêtre avec le runner Domius, puis valider démarrage et arrêt.
4. Implémenter la destruction sur fermeture, les commandes et la souscription d'événements.
5. Ajouter les quatre fenêtres, puis partager les préférences par contexte.
6. Terminer Kanban, Gantt, palette de commandes, panneaux et splitter dans la bibliothèque.
7. Relier les événements entre fenêtres et écrire les tests de cycle de vie.
8. Valider le lancement en développement et le build packagé sur Windows.

## Contrat de qualité

- toutes les fenêtres reçoivent un label unique et une taille vérifiable ;
- le graphe de dépendances de l'exemple et de `domius-desktop` ne contient pas `tauri` ;
- aucune donnée mutable globale non protégée ;
- un handler desktop peut être retiré ou cesse d'agir avec son scope ;
- la fermeture d'une fenêtre détruit son scope une seule fois ;
- les événements personnalisés utilisent une charge sérialisable et un nom documenté ;
- le CLI n'écrase jamais un fichier existant sans erreur explicite ;
- le CSS généré ne fuit pas entre deux composants portant la même classe ;
- l'application ne dépend pas d'un serveur distant pour son parcours de démonstration ;
- tests natifs, tests web concernés et build desktop Domius passent.

## Tests d'acceptation

1. Générer un projet, ajouter un composant et une page, puis compiler le résultat.
2. Deux contenus CSS identiques placés dans des chemins différents reçoivent des scopes distincts et stables.
3. Ouvrir deux tâches crée deux labels de fenêtre distincts.
4. Sauvegarder une tâche dans sa fenêtre met à jour le Kanban et le Gantt de la fenêtre principale.
5. Focus et blur produisent les événements attendus sans doublon.
6. Fermer une fenêtre retire son scope et ses effets ; répéter l'opération ne panique pas.
7. La palette exécute une commande au clavier et restaure le focus à sa fermeture.
8. Déplacer une carte Kanban modifie les dates visibles dans le Gantt.
9. Le build packagé par Domius démarre et affiche le portefeuille sans erreur de console.
10. `cargo tree` confirme l'absence de Tauri dans `domius-desktop` et dans l'exemple.

## Limites de propriété

OpenCode possède ce dossier, `domius-desktop`, `domius-cli` et les composants attribués dans le tableau. Les fichiers de workspace, les réexports globaux et la CI restent sous la responsabilité de Codex pendant l'intégration. Toute modification nécessaire dans `domius-core` doit être consignée ici avant d'être appliquée, car le centre d'exploitation dépend du même runtime.

## Journal de décisions

Consigner ici les noms d'événements, la forme de leurs charges, les choix de persistance et tout changement imposé aux templates du CLI.
