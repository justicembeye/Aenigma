
---

# Cahier des Charges du Projet Aenigma

## 1. Vision et Philosophie du Jeu

### 1.1. Concept Fondamental
**Aenigma** est un jeu de **déduction stratégique multijoueur** où chaque participant choisit secrètement un élément (mot, concept, personnage) appartenant à un thème commun et tente de découvrir ceux des adversaires par une investigation progressive et personnelle. Le jeu fusionne culture générale, stratégie, mémoire et psychologie.

### 1.2. Piliers de Conception
* **Investigation Asymétrique** : Le cœur du jeu. Chaque joueur possède une **vision unique et personnelle** de la partie. Les informations qu'il découvre ne sont pas partagées, transformant le jeu en une véritable enquête individuelle.
* **Stratégie et Prise de Risque** : Les joueurs doivent faire des choix stratégiques constants, notamment à travers le système de "Buzz" qui récompense la déduction audacieuse.
* **Extensibilité Radicale** : L'architecture du jeu est pensée pour être indépendante du contenu, permettant une extension quasi infinie via de nouveaux thèmes, mots et règles.
* **Équilibrage Dynamique** : Le jeu intègre des mécanismes intelligents (rareté, longueur des mots) pour assurer des parties justes et intéressantes, quel que soit le contenu choisi.

---

## 2. Architecture Conceptuelle (Indépendante du Langage)

### 2.1. Entités Principales
* **Entité `Joueur`**
    * **Identifiant** : Nom ou pseudo.
    * **MotSecret** : L'instance du mot qu'il doit protéger.
    * **Statut** : Actif / Éliminé.
    * **Score** : Points accumulés.
    * **VuePersonnelle** : Une collection de ses propres découvertes sur les mots des adversaires.
    * **Ressources** : Charges de Buzz, capacités spéciales, etc.
* **Entité `MotSecret`**
    * **Contenu** : Le mot ou l'expression à deviner (ex: "FRANCE").
    * **Thème** : Sa catégorie (ex: "Pays d'Europe").
    * **Métadonnées** :
        * **Longueur**.
        * **Score de Rareté Culturelle** (ex: 1 pour très connu, 3 pour rare).
        * **Difficulté Calculée** (un score global pour l'équilibrage).
* **Entité `Partie`**
    * **Configuration** : Mode de jeu, thème(s), difficulté, nombre de joueurs.
    * **Liste des Joueurs**.
    * **État Actuel** : Tour du joueur en cours, historique des actions.

### 2.2. La Boucle de Jeu Principale
1.  **Phase 1 : Initialisation**
    * Les joueurs configurent la partie (mode, thème).
    * Chaque joueur choisit secrètement son mot.
    * Le système initialise les `VuePersonnelle` de chaque joueur (tous les mots adverses sont masqués, ex: `_ _ _ _ _ _`).
2.  **Phase 2 : Déroulement des Tours**
    * À son tour, le joueur actif choisit une action principale : **Investiguer** ou **Tenter une Devinette**.
    * Le système traite l'action et met à jour l'état de la partie (principalement la `VuePersonnelle` du joueur actif).
    * Le tour passe au joueur suivant.
3.  **Phase 3 : Fin de Partie**
    * La partie se termine lorsqu'il ne reste plus qu'un joueur **Actif** (Victoire Traditionnelle).
    * Le système calcule les scores finaux pour déterminer le **Champion des Devinettes**.
    * Si nécessaire, une **Confrontation Finale** est déclenchée.

---

## 3. Mécaniques de Jeu (Du Simple au Complexe)

### 3.1. Niveau 1 : Le Cœur du Gameplay

#### 3.1.1. L'Action "Investiguer"
C'est l'action de base pour récolter de l'information.
* Le joueur actif propose **une lettre**.
* Le système recherche cette lettre **uniquement dans les mots secrets des adversaires**.
* Si la lettre est trouvée, les positions sont révélées **uniquement dans la `VuePersonnelle` du joueur actif**.
    * *Exemple : Joueur 1 propose 'A'. Le mot de Joueur 2 est "ITALIE". La vue de Joueur 1 pour Joueur 2 devient `_ T A _ I E`. Les autres joueurs ne voient pas cette mise à jour.*

#### 3.1.2. L'Action "Tenter une Devinette"
* Le joueur actif cible un adversaire et propose un mot complet.
* **Si la devinette est correcte** :
    * Le joueur ciblé passe au statut **Éliminé**.
    * Le joueur actif gagne des points.
* **Si la devinette est incorrecte** :
    * Le jeu continue. Une pénalité peut s'appliquer selon le mode de jeu.

### 3.2. Niveau 2 : La Couche Stratégique

#### 3.2.1. Système de Difficulté et Équilibrage
Le choix d'un mot est un acte stratégique. L'équilibre est géré par deux facteurs principaux :
* **Longueur du Mot** : Un mot court est plus facile à deviner.
* **Rareté Culturelle** : Un mot peut être court mais très difficile car peu connu (ex: "NIUE" vs "FRANCE").
* **Mécanisme** :
    * En mode **Facile**, le jeu propose des mots courts et très connus.
    * En mode **Normal**, un équilibre de mots moyens est proposé.
    * En mode **Difficile**, des mots longs et/ou rares sont disponibles, mais ils rapportent plus de points en cas de devinette réussie, créant un système de risque/récompense.

#### 3.2.2. Système de "Buzz" Avancé
Le Buzz est une action spéciale de "devinette à haut risque".
* **Disponibilité** : Le Buzz n'est pas toujours disponible. Il se "charge" (via un "condensateur") après un certain nombre de tours ou de temps. Il est impossible à utiliser dans les premiers tours.
* **Utilisation** : Le joueur peut utiliser une charge de Buzz pour tenter une devinette.
* **Récompense (si réussi)** :
    * Gain de points **supérieur** à une devinette normale (ex: 3 points au lieu de 2).
    * Avantages stratégiques (ex: recharger le Buzz plus vite).
* **Pénalité (si échoué)** : La pénalité est sévère pour décourager les abus. L'option la plus stratégique est la **révélation forcée** : le joueur doit révéler une lettre de son propre mot à **tous** les adversaires.

### 3.3. Niveau 3 : L'Expansion du Contenu

#### 3.3.1. Le Système de Thèmes
Le jeu n'est pas limité aux pays. Il peut s'adapter à n'importe quel domaine de connaissance.
* **Exemples de Thèmes** : Capitales du monde, Auteurs célèbres, Inventions scientifiques, Personnages de fiction, etc.
* **Fonctionnement** : Au début de la partie, un thème (ou une sélection de thèmes) est choisi, et tous les mots secrets doivent lui appartenir.

---

## 4. Modes de Jeu

* **Mode Classique** : Le mode de base décrit ci-dessus, avec le système d'investigation asymétrique.
* **Mode Spécialiste** : Chaque joueur choisit son propre thème de prédilection pour défier les autres dans son domaine d'expertise.
* **Mode Chaos** : Les joueurs peuvent choisir le même mot secret, créant des dynamiques de bluff et des alliances involontaires.
* **Mode Débutant (Révélation Globale)** : Pour faciliter l'apprentissage, l'action "Investiguer" peut révéler les lettres à tous les joueurs simultanément, supprimant l'aspect asymétrique.

---

## 5. Vision à Long Terme (Fonctionnalités Futures)

Cette section sert à ne pas oublier les idées d'expansion, à implémenter une fois le cœur du jeu solide.
* **5.1. Mode Campagne Narratif** : Une histoire solo où le joueur progresse en résolvant des énigmes, avec un système de compétences à débloquer (ex: "Intuition", "Mémoire Photographique").
* **5.2. Mode Multijoueur en Ligne** : Salles de jeu publiques/privées, système de classement par ligues (Bronze à Maître), tournois, profils personnalisables.
* **5.3. Déclinaison en Jeu de Société** : Adaptation des mécaniques pour une version physique avec des cartes, des pions et des tableaux effaçables.
* **5.4. Écosystème Transmédia** : Le concept peut s'étendre à une émission télévisée interactive, créant un lien entre le jeu digital et un format média plus large.

---

## 6. Plan de Développement Progressif

Ce plan est conçu pour se concentrer d'abord sur la logique et valider le concept.

* **Phase 1 : Le Noyau Jouable (MVP - Produit Minimum Viable)**
    1.  **Objectif** : Créer une version console fonctionnelle du **Mode Classique**.
    2.  **Fonctionnalités** :
        * Gestion des joueurs (création, élimination).
        * Implémentation du **cœur du gameplay** : actions "Investiguer" (asymétrique) et "Tenter une Devinette".
        * Système de points de base.
        * Conditions de victoire simples (dernier joueur en vie).
        * Thème unique pour commencer : "Pays du Monde".
        * Interface console textuelle claire affichant la `VuePersonnelle`.
* **Phase 2 : L'Ajout de la Profondeur Stratégique**
    1.  **Objectif** : Enrichir l'expérience de jeu.
    2.  **Fonctionnalités** :
        * Intégration du système de **Difficulté et Équilibrage** (longueur/rareté).
        * Implémentation complète du **Système de Buzz**.
        * Ajout du **Mode Débutant** (révélation globale).
* **Phase 3 : L'Expansion du Contenu et de la Variété**
    1.  **Objectif** : Assurer la rejouabilité.
    2.  **Fonctionnalités** :
        * Implémentation du **Système de Thèmes multiples**.
        * Développement des autres **Modes de Jeu** (Spécialiste, Chaos).
        * Mise en place de la **Confrontation Finale**.
* **Phase 4 et au-delà : La Vision Globale**
    1.  **Objectif** : Construire l'écosystème Aenigma.
    2.  **Fonctionnalités** :
        * Développement de l'interface graphique (GUI).
        * Travail sur le **Mode Multijoueur en Ligne**.
        * Prototypage du **Mode Campagne** et du **Jeu de Société**.