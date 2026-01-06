# YOLO

Ce TP nécessite une version récente de Python 3, avec l'interpréteur Python et PIP dans le PATH.

## 0. Mise en place de l'environnement de travail

- Clonez ce dépôt:

    ```sh
    git clone https://github.com/GuillaumeMZ/sffs-yolo-tp.git
    ```

- Ouvrez un terminal dans le dépôt cloné
- **macOS/Linux**: créez l'environnement virtuel et installez les dépendances:

    ```sh
    python3 -m venv venv
    source venv/bin/activate
    python3 -m pip install opencv-python ultralytics roboflow
    ```

- **Windows**: créez l'environnement virtuel et installez les dépendances:

    ```psh
    py -m venv venv
    .\venv\Scripts\activate
    py -m pip install opencv-python ultralytics roboflow
    ```

## 1. Création et annotation d'un dataset pour le jeu pierre-feuille-ciseaux

### 1.1 Création d'un *workspace*

Rendez-vous sur [Roboflow](https://roboflow.com) et créez-vous un compte. Vous serez redirigé sur la page de création d'un *workspace*. Entrez le nom que vous souhaitez, choisissez "Public Plan", cliquez sur "Continue", puis sur "Create Workspace" sur la page suivante.

### 1.2 Création d'un projet

Sur la page principale de votre *workspace*, cliquez sur "Use my own data" (au centre, en bas). Choisissez "Build a custom model" sur la *pop-up*. Sur la page "Let's create your project", entrez un nom de projet au choix (si vous le voulez) et laissez les options par défaut. Cliquez sur "Create Public Project".

### 1.3 Ajout des classes

Cliquez sur l'onglet "Classes & Tags" dans la barre de gauche. Ajoutez les classes `rock`, `paper` et `scissors` en écrivant "rock, paper, scissors" dans la barre de texte centrale puis cliquez sur le bouton "Add Classes".

### 1.4 Import et Annotation des données

#### 1.4.1 Import

Le dossier `images` contient 4 images (2 pierre, 1 feuille, 1 ciseaux) que vous devez téléverser dans votre projet Roboflow.

Dans votre projet Roboflow, cliquez sur "Select Files" au centre (onglet "Upload Data" dans la barre à gauche), et sélectionnez les 4 images. Cliquez sur "Save and Continue".

#### 1.4.2 Annotation

Sur la page "Annotate", cliquez sur "Label Myself" à droite de la page. L'interface d'annotation va alors s'ouvrir. Sélectionnez l'outil "Bouding Box Tool" (2e icone en partant du haut sur la barre de droite).

Pour chaque image, dessinez un rectangle le plus serré possible autour de la main. Dans le menu qui s'ouvre à gauche de l'image, sélectionnez la classe correspondante (`rock`, `paper` ou `scissors`) que vous avez créée précédemment.

Une fois les 4 images annotées, cliquez sur le bouton retour (flèche en haut à gauche) pour revenir à la vue d'ensemble, puis cliquez sur "Add 4 images to Dataset" (en haut à droite). Sur la *popup* qui apparaît, choisissez la méthode "Split Images Between Train/Valid/Test". La répartition proposée par défaut devrait être "Train: 3 images, Valid: 1 images, Test: 0 images". Si ce n'est pas le cas, déplacez les curseurs pour y arriver. Cliquez sur "Add 4 Images".

### 1.5 Génération du Dataset

Nous utiliserons le SDK Roboflow Python pour télécharger le dataset prêt à l'entraînement. Pour que le dataset soit utilisable par le code, il faut d'abord le "figer" dans une version.

Dans l'onglet "Dataset", cliquez sur le bouton "New Dataset Version" (en haut à droite). Cliquez sur "Continue" à chaque étape (laissez les valeurs par défaut) puis sur "Create".

Une fois le *dataset* créé, cliquez sur "Download Dataset" (en haut à droite). Choisissez le format d'annotation "YOLOv12", sélectionnez "Show download code", puis cliquez sur "Continue". Copiez-collez le code dans le script `download_dataset.py` puis exécutez le:

**MacOS/Linux:**

```sh
python3 download_dataset.py
```

**Windows:**

```sh
py download_dataset.py
```

Le dataset a été téléchargé dans un sous-dossier. Utilisez l'explorateur de fichiers pour voir sa structure.

**Question: comment YOLO fait-il le lien entre une image et le fichier texte qui contient les informations la concernant ?**

À partir de l'explorateur de fichiers, ouvrez l'un des fichiers .txt (soit dans train/labels ou dans valid/labels).

**Questions:**

- **À quoi correspond une ligne ?**
- **Que signifie chaque élément d'une ligne ?**
- **Pourquoi les nombres décimaux sont-ils normalisés entre 0 et 1 ?**

### 1.6 Entraînement

Nous allons maintenant entraîner notre (très) petit dataset avec YOLO. Lancez la commande suivante dans le terminal pour démarrer l'entraînement (n'oubliez pas d'adapter le chemin de `data=`). Nous l'avons configuré de la manière suivante:

- Utilisation de YOLOv12-nano (téléchargé automatiquement) comme base pour notre modèle
- 1 seule *epoch*
- Les images sont redimensionnées en 640x640 pixels.

```sh
yolo detect train data=chemin/vers/data.yaml epochs=1 imgsz=640
```

Le résultat de l'entraînement est contenu dans le dossier `runs/detect/train`.

- Les métriques résultant de l'entraînement (*precision*, *recall*, *F1*, etc.) se trouvent dans les fichiers .png contenus dans ce dossier.
- Notre modèle entraîné se trouve dans le sous-dossier `weights`. C'est lui que l'on chargerait avec YOLO pour effectuer une tâche de détection pierre/feuille/ciseaux sur de nouvelles images.

Remarquez que le sous-dossier `weights` contient à la fois `best.pt` et `last.pt`. **Question: pourquoi la dernière version du modèle n'est-elle pas nécessairement la meilleure ?**

Note: nous n'utiliserons pas ce modèle dans la suite de ce TP, pour plusieurs raisons:

- 4 images (dont 1 pour la validation) sont bien loin d'être suffisantes pour obtenir un modèle performant
- 1 epoch est bien loin d'être suffisante pour obtenir un modèle performant.

Ultralytics propose (entre autres) les recommendations suivantes:

- Au moins 1500 images par classe
- Au moins 10000 instances (objets labelisés) par classe
- Avoir entre 1 et 10% d'images ne contenant aucun objet
- Démarrer avec 300 *epochs* et ajuster en fonction du résultat (diminuer en cas d'*overfitting*, augmenter dans le cas contraire).

(Source: [Documentation Ultralytics](https://docs.ultralytics.com/yolov5/tutorials/tips_for_best_training_results/))

## 2. Analyse des graphes de l'entraînement d'un modèle

### 2.1 *Loss*

Ouvrez le fichier `dataset_part_2/results.png`. Nous nous concentrerons pour l'instant sur les graphes de perte/*loss* (les 6 à gauche).

Rappel: la perte est une note de "nullité": plus elle est élevée, moins le modèle est bon. Plus elle est basse (le minimum étant 0), plus le modèle est bon. Elle est mesurée en comparant le résultat attendu pour une tache par rapport au résultat obtenu à cette tâche.

Nous remarquons que les courbes concernant les données d'entraînement (`train`) continuent de descendre au bout de 300 *epochs* alors que celles concernant les données de validation (`val`) ont stagné dès ~150 *epochs*.

**Question: à quelle "maladie" cette constatation correspond-elle ?**

### 2.2 *Precision* et *recall*

Nous nous concentrons maintenant sur les graphes de *precision* et de *recall* (en haut à droite).

**Questions:**

- **À quoi correspond la *precision* ? À quoi correspond le *recall* ?**
- **Dans nos résultats, la *precision* et le *recall* sont proches de 1. Est-ce une bonne ou une mauvaise chose ? Pourquoi ?**
- **Pendant l'entraînement, quel jeu de données est utilisé pour calculer la *precision* et le *recall* ? `train`, `test`, ou `val` ?**

### 2.3 Matrice de confusion

Ouvrez maintenant le fichier `dataset_part_2/confusion_matrix_normalized.png`.

**Questions:**

- **Que représentent les cases foncées de la diagonale haut-gauche vers bas-droite ? Pourquoi veut-on qu'elles soient le plus proche possible de 1.0 ?**
- **À quoi correspondent la ligne et la colonne "background" ?**
- **Quelle est la classe que notre modèle reconnaît le mieux ? Le moins bien ?**
- **Quelles sont les deux classes que le modèle confond le plus ? Avec quelle autre classe ?**
- **Globalement, cette matrice est-elle satisfaisante ?**

### 2.4 Bonus: vitesse, précision et taille du modèle

Nous avons utilisé le modèle YOLOv12-small comme base pour l'entraînement. **Question: Si nous avions utilisé YOLOv12x (extra large), quels auraient été les impacts en termes de vitesse d'exécution, de précision et de taille du modèle sur le disque ? Quelle est la différence d'architecture entre un petit modèle et un grand modèle qui explique tout cela ?**

## 3. Utilisation d'un modèle pré-entraîné pour jouer à pierre/feuille/ciseaux en live 🔥

Le dossier `dataset_part_3` contient `model.pt`, un modèle basé sur YOLOv12-small, entraîné sur + de 3000 images pendant 300 *epochs* pour de la détection de pierre/feuille/ciseaux. Nous l'utiliserons pour créer un jeu pierre/feuille/ciseaux utilisant la webcam en live !

### Règles du jeu

L'écran est divisé en deux zones:

- la zone de gauche correspond au joueur 1;
- la zone de droite correspond au joueur 2.

L'objectif est d'afficher, au centre de la fenêtre, qui est le gagnant du pierre/feuille/ciseaux: le joueur 1 ou le joueur 2 (ou égalité) ?

**Complétez le TODO (ligne 36) du script `rock_paper_scissors_live.py` pour le faire fonctionner.**

Aide:

- `box` (ligne 35) est de type `Boxes` ([documentation](https://docs.ultralytics.com/reference/engine/results/#ultralytics.engine.results.Boxes)). Cette classe dispose notamment des propriétés `xyxy`, `conf` et `cls` qui sont des tenseurs PyTorch (et se manipulent comme des tableaux). **`Boxes` est une "structure de tableaux" et non pas un "tableau de structures", c'est-à-dire qu'au lieu d'écrire `box[0].abcd`, on écrit `box.abcd[0]`.**
- `model` (ligne 4) est de type `Model` ([documentation](https://docs.ultralytics.com/reference/engine/model/#ultralytics.engine.model.Model)). Cette classe dispose d'une propriété `names` de type `dict[int, str]` qui permet d'accéder au nom d'une classe à partir de son identifiant.
