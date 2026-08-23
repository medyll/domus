# Collaborative Media Studio

## Mandat

**Agent responsable : Claude**

Construire un studio web où une équipe importe une vidéo ou un fichier audio, prépare ses métadonnées, rédige une fiche éditoriale, demande une revue et publie une version validée. L'application doit pousser le système de composants Domius, la macro RSX, les formulaires, les hooks et les portails.

Le studio ne doit pas masquer les trous de la bibliothèque. Si `Autocomplete`, `RichTextEditor`, `DiffViewer` ou un autre module attribué n'a pas d'implémentation, terminer d'abord ce module dans `domius-web` et couvrir son comportement par un test navigateur.

## Place dans la couverture globale

```text
domius-macro + DomiusComponent + hooks
                    |
                    v
        Collaborative Media Studio
                    |
        +-----------+------------+
        |                        |
   édition média            revue d'équipe
```

Le centre d'exploitation couvre le runtime, les données et les graphiques. L'application desktop couvre le runtime natif Domius, le CLI, Kanban et Gantt.

## Parcours attendu

Un utilisateur importe un média sur `/library`, ouvre `/editor/:id`, complète un formulaire riche, puis envoie la version sur `/review/:id`. Un relecteur compare deux versions, laisse un commentaire avec une mention et attribue une note. La publication se termine sur `/publish/:id`, avec récapitulatif et retour possible vers la bibliothèque.

Chaque contrôle doit modifier un vrai modèle en mémoire. Pas de panneau rempli de composants sans relation avec le média en cours.

## Écrans

### `/library`

- import par glisser-déposer et sélection de fichier ;
- liste de médias avec miniature, avatar du propriétaire et badges ;
- recherche par autocomplete, filtre multi-sélection et arbre de collections ;
- carousel pour les derniers médias ;
- panneau de transfert pour déplacer des éléments entre deux collections.

### `/editor/:id`

- lecteur audio ou vidéo selon le média ;
- formulaire avec date, masque de référence, select, cascader, tree select, slider et switch ;
- éditeur riche, choix de couleur et tags éditoriaux ;
- panneaux redimensionnables pour séparer aperçu, formulaire et notes ;
- sauvegarde au clavier et fermeture d'un panneau détectée par `use_click_outside`.

### `/review/:id`

- comparaison des versions avec `DiffViewer` ;
- commentaires, fil de messages, mentions et groupe d'avatars ;
- note de revue et choix segmenté du verdict ;
- popover pour les détails d'une annotation ;
- modal de confirmation fournie par l'intégration finale si le parcours la réclame.

### `/publish/:id`

- stepper montrant les contrôles avant publication ;
- check cards pour les canaux de diffusion ;
- résumé dans un drawer ;
- état final, puis retour à la bibliothèque ;
- tour guidé disponible au premier lancement.

## API Domius attribuée à cet exemple

### Composants et macro

- `DomiusComponent`, `mount_component` et `DomiusNode` ;
- props et état séparés pour chaque composant applicatif ;
- macro procédurale de `domius-macro` avec syntaxe Rust et syntaxe HTML ;
- expressions réactives, attributs, fragments, texte et gestionnaires d'événements dans les tests de macro ;
- `create_scope` et `dispose_scope` pour les éditeurs montés et démontés.

La macro publique renvoie aujourd'hui un commentaire de debug au lieu du DOM généré, alors que `codegen.rs` existe déjà. Ce dossier possède la correction : brancher le parseur sur le générateur, fixer le nom public retenu sans casser les usages existants, puis ajouter des tests de compilation et un test navigateur.

### Hooks et utilitaires

- `use_click_outside` et sa variante avec callback ;
- `use_focus`, `use_focus_auto`, `focus_element` et `blur_element` ;
- `use_keyboard` et `KeyboardConfig` pour sauvegarde, fermeture et déplacement entre champs ;
- `class_names`, la macro `cn!` et `Portal`, y compris montage, remplacement et démontage du contenu.

### Composants réservés à ce dossier

| Famille | Modules à intégrer |
|---|---|
| Primitives | `button`, `checkcard`, `input`, `space` |
| Navigation | `accordion`, `drawer`, `stepper` |
| Formulaires | `autocomplete`, `date_picker`, `file_uploader`, `input_mask`, `multi_select`, `rich_text`, `select`, `slider`, `switch`, `treeselect`, `upload` |
| Feedback | `popover` |
| Données | `avatar`, `carousel`, `comment`, `message`, `tree_view` |
| Média | `audio_player`, `video_player` |
| Pro | `cascader`, `color_picker`, `diff_viewer`, `mention`, `rating`, `segmented`, `tour`, `transfer` |

Les deux API d'import, `file_uploader` et `upload`, doivent avoir des rôles distincts et documentés. Par exemple : `upload` gère la sélection simple, tandis que `file_uploader` gère la file, la progression, l'annulation et les erreurs. Ne conserver deux composants identiques que pour satisfaire une liste de couverture serait une mauvaise issue.

## Architecture cible

```text
src/
  lib.rs
  app.rs
  model/
    media.rs
    review.rs
    taxonomy.rs
  pages/
    library.rs
    editor.rs
    review.rs
    publish.rs
  components/
    media_form.rs
    media_preview.rs
    review_thread.rs
    version_diff.rs
  state/
    editor_session.rs
    library_filters.rs
tests/
  editor_flow.rs
  keyboard_flow.rs
  disposal_flow.rs
```

## Ordre de travail

1. Rendre la macro RSX fonctionnelle et verrouiller ses deux syntaxes par des tests.
2. Construire le shell avec `DomiusComponent`, les scopes et des composants applicatifs courts.
3. Terminer les contrôles de formulaire, puis connecter un modèle de média cohérent.
4. Ajouter l'import, les lecteurs et le flux de revue.
5. Terminer les composants `pro` attribués, les hooks clavier/focus et les portails.
6. Couvrir le parcours complet dans un navigateur, puis documenter le lancement.

## Contrat de qualité

- aucun formulaire ne garde un état parallèle dans le DOM ; le modèle Domius reste la source de vérité ;
- aucune fuite de listener après fermeture de l'éditeur, du drawer ou du popover ;
- les raccourcis ignorent les combinaisons non prévues et n'écrasent pas la saisie native ;
- tabulation, focus initial, restauration du focus et touche Échap fonctionnent sur les couches flottantes ;
- l'import refuse un type interdit et expose une erreur lisible ;
- audio et vidéo gèrent lecture, pause, volume et changement de source ;
- le diff affiche les lignes ajoutées, retirées et inchangées ;
- le build WASM et les tests navigateur passent dans Firefox et Chrome.

## Tests d'acceptation

1. Importer un fichier crée un média, met à jour la bibliothèque et réinitialise la zone d'import.
2. Les onze modules de formulaire changent une propriété utile du média ou de sa publication.
3. `Ctrl+S` sauvegarde une seule fois et garde le focus dans l'éditeur.
4. Un clic extérieur ferme le popover ; son listener disparaît ensuite.
5. Le passage d'une version à l'autre met à jour le lecteur, le diff et les commentaires associés.
6. Une mention sélectionnée insère l'identifiant attendu dans le commentaire.
7. Le transfert déplace les éléments sans doublon et conserve leur ordre.
8. Le montage puis démontage répété de l'éditeur ne multiplie pas les effets.
9. Les deux syntaxes RSX produisent un arbre DOM équivalent pour le même composant témoin.

## Limites de propriété

Claude possède ce dossier, `domius-macro`, le système de composants web, les hooks, les utilitaires et les modules de composants listés ici. Les modifications des fichiers de réexport partagés attendent la passe d'intégration tenue par Codex. Importer temporairement un module par son chemin complet évite de bloquer le développement.

## Journal de décisions

Consigner ici les changements d'API publique, les divergences entre `upload` et `file_uploader`, ainsi que toute dépendance ajoutée pour le diff, la couleur ou les lecteurs.
