# Plan multi-cibles indépendant

## Contrat produit

Domius construit deux couches qui restent séparées :

1. un framework UI Rust à réactivité fine, sans Virtual DOM ;
2. un runtime d'application web et desktop appartenant à Domius.

Le projet ne dépend d'aucun framework applicatif concurrent. Un moteur de
fenêtre, de webview ou de rendu peut exister sous forme d'adaptateur privé ; ses
types ne traversent jamais l'API publique.

```text
Application
    |
    +-- domius-ui -------- composants, RSX, état, cycle de vie
    |
    +-- domius-runtime --- fenêtres, commandes, événements, assets
                                |
                                +-- adaptateurs de plateforme privés
```

## État constaté au 23 août 2026

| Zone | État |
|---|---|
| `domius-core` | Signals, computed, effects, batch et scopes testés nativement |
| `domius-macro` | Parseur et générateur présents ; l'entrée publique ne les relie pas encore |
| `domius-web` | Runtime web utilisable, bibliothèque de composants très inégale |
| `domius-desktop` | Types de composant, contexte, événements et destruction ; aucun moteur de fenêtre |
| `domius-cli` | Génération et CSS scoping ; templates encore incomplets |

Les fichiers de composants ne constituent pas une preuve de fonctionnalité. Un
module compte comme terminé seulement lorsqu'un exemple l'utilise et qu'un test
navigateur ou natif vérifie son comportement.

## Architecture visée

```text
domius-core
    Réactivité indépendante de la plateforme

domius-macro
    RSX vers instructions de rendu Domius

domius-ui
    Composants, props, contexte, listes, routeur, erreurs

domius-render-web
    Instructions Domius vers DOM web_sys

domius-desktop
    Application, fenêtres, commandes, événements et arrêt propre

domius-platform-webview
    Adaptateur privé vers les webviews système

domius-cli
    new, add, dev, check, build, bundle et doctor

domius
    Façade publique et prelude
```

Cette structure décrit la destination. Une nouvelle crate n'est créée que
lorsqu'un contrat testé exige son extraction.

## Séquence de migration

### M0 : zéro dépendance concurrente

- retirer toute dépendance au framework desktop précédemment introduit ;
- retirer l'ancien exemple qui l'utilisait ;
- corriger la documentation et les commandes ;
- ajouter un contrôle CI sur le graphe Cargo ;
- conserver les 181 tests natifs au vert.

Retour arrière : chaque changement reste dans un commit isolé. L'ancien exemple
reste consultable dans l'historique Git sans rester actif dans l'arborescence.

### M1 : noyau UI honnête

- connecter la macro publique au générateur RSX ;
- définir montage, mise à jour et démontage ;
- relier les expressions réactives aux nœuds concernés ;
- terminer attributs, événements, fragments et listes indexées ;
- remplacer les panics de parcours par des erreurs Domius ;
- réécrire `examples/hello-world` avec l'API Domius uniquement.

Condition de sortie : le compteur et la todo list ne construisent plus leur DOM
manuellement, et leur retrait détruit tous les effets associés.

### M2 : tranche desktop verticale

- créer un builder d'application Domius ;
- ouvrir une fenêtre et charger les assets locaux ;
- enregistrer une commande Rust typée ;
- transmettre événements et réponses corrélées ;
- ouvrir une deuxième fenêtre ;
- fermer chaque fenêtre et détruire son scope ;
- arrêter le processus sans thread orphelin.

Le premier moteur peut utiliser une bibliothèque de webview et de fenêtre sous
un adaptateur privé. L'API publique appartient entièrement à Domius.

### M3 : tooling

- rendre les projets générés compilables ;
- ajouter `domius dev`, puis `domius check` et `domius build` ;
- recharger CSS et assets sans recompilation complète ;
- fournir un diagnostic des dépendances système ;
- ajouter le packaging une plateforme après l'autre.

### M4 : applications de preuve

- `examples/operations-control-center` couvre runtime, routes et données ;
- `examples/collaborative-media-studio` couvre RSX, formulaires et média ;
- `examples/desktop-project-command` couvre fenêtres, commandes et événements.

Un composant manquant se corrige dans la bibliothèque avec son test. Aucun
exemple ne garde une copie locale destinée à masquer une API inachevée.

## Frontières de sécurité desktop

Le runtime refuse par défaut tout appel non enregistré. Chaque message possède
un identifiant, une commande connue, une charge sérialisée et une réponse typée.
Le host contrôle l'origine, la navigation externe, la taille des messages et la
politique de contenu. Les permissions se définissent par fenêtre.

Ces règles précèdent les plugins et le système de mise à jour.

## Ordre des plateformes

1. Windows pour établir le runtime et les tests de bout en bout ;
2. Linux pour traiter WebKitGTK et les différences de protocole ;
3. macOS pour WKWebView, signature et notarisation ;
4. mobile après stabilisation du desktop.

## Règles de contribution

- un commit porte une seule intention ;
- tout changement de contrat public inclut un test ;
- les adaptateurs ne remontent pas leurs types dans les crates publiques ;
- une fonctionnalité annoncée dans le README doit avoir un parcours exécutable ;
- les marqueurs `todo!` restent interdits sur les chemins publics ;
- le graphe Cargo ne peut pas réintroduire un framework concurrent.

## Définition de la première version desktop

Une application générée par le CLI compile, ouvre une fenêtre, sert une UI
Domius, appelle une commande native, reçoit un événement, ouvre puis ferme une
seconde fenêtre et produit un paquet installable sur Windows. Tout le parcours
dispose correctement ses scopes et passe en CI.
