# 🎮 Aenigma

## 📋 Table des Matières
- [Concept et Philosophie](#concept-et-philosophie)
- [Architecture Globale](#architecture-globale)
- [Mécaniques de Jeu Principales](#mécaniques-de-jeu-principales)
- [Systèmes de Contenu](#systèmes-de-contenu)
- [Modes de Jeu](#modes-de-jeu)
- [Interface et Expérience Utilisateur](#interface-et-expérience-utilisateur)
- [Gestion des Données](#gestion-des-données)
- [Plan de Développement Évolutif](#plan-de-développement-évolutif)
- [Règles et Scénarios Complets](#règles-et-scénarios-complets)

## Concept et Philosophie

### 🎯 Vision Fondamentale
Aenigma est un jeu de déduction stratégique multijoueur où chaque participant choisit secrètement un élément (pays, concept, personnage) et tente de découvrir ceux des adversaires par déduction progressive. Le jeu fusionne culture générale, stratégie et psychologie dans une expérience immersive.

### 🧭 Principes de Conception
- **Universalité** : Architecture indépendante du langage de programmation
- **Modularité Extrême** : Séparation stricte moteur/interface
- **Équilibrage Dynamique** : Système de difficulté intelligent et progressif
- **Immersion Asymétrique** : Chaque joueur possède une vision unique du jeu
- **Extensibilité Radicale** : Thèmes, modes et règles facilement extensibles

### 🎓 Objectifs Pédagogiques
- Développer la culture générale à travers des thèmes variés
- Stimuler la déduction logique et la mémoire
- Encourager la pensée stratégique et la prise de décision
- Favoriser l'apprentissage implicite par le jeu

## Architecture Globale

### 🏗️ Structure Conceptuelle du Moteur

**Entité MotSecret**
- Identifiant unique
- Contenu textuel (le mot à deviner)
- Thème d'appartenance
- Métadonnées de difficulté (score calculé)
- Niveau de rareté culturelle (1-3)
- Longueur en caractères
- Code unique d'identification

**Entité Joueur**
- Identifiant unique
- Nom d'affichage
- Mot secret attribué
- Statut (actif/éliminé)
- Score actuel
- Historique des actions (lettres proposées, devinettes)
- Vue personnelle des progressions adverses
- Pénalités actives éventuelles
- Indicateur de contrôle (humain/IA)

**Entité Partie**
- Configuration globale (mode, difficulté, variantes)
- Liste des joueurs participants
- Ordre de tour actuel
- Historique des tours passés
- État de progression (en cours/terminée)
- Paramètres de sauvegarde
- Métadonnées temporelles

**Entité Thème**
- Catégorie principale
- Sous-thèmes disponibles
- Liste des mots associés
- Paramètres d'équilibrage
- Restrictions éventuelles

### 🔄 Flux Principal

**Phase d'Initialisation**
1. Sélection du mode de jeu et des paramètres
2. Chargement des thèmes disponibles
3. Création et configuration des joueurs
4. Attribution secrète des mots
5. Préparation des vues personnelles

**Boucle de Jeu Principale**
```
RÉPÉTER jusqu'à condition_victoire
   POUR chaque joueur actif DANS ordre_tour
      AFFICHER vue_personnelle(joueur)
      PROPOSER choix_actions(joueur)
      TRAITER action_sélectionnée
      METTRE À JOUR états_jeu
      VÉRIFIER conditions_élimination
   FIN POUR
FIN RÉPÉTER
```

**Phase Finale**
- Calcul des scores et classements
- Déclaration des vainqueurs
- Gestion des confrontations éventuelles
- Sauvegarde des résultats
- Retour au menu principal

## Mécaniques de Jeu Principales

### 🔍 Système de Découverte Progressive

**Révélation Asymétrique des Lettres**
- Chaque joueur découvre indépendamment les lettres des mots adverses
- Les révélations sont personnelles et non partagées
- Accumulation progressive d'indices par joueur
- Mémorisation nécessaire des découvertes

**Gestion des Tours**
- Tour par tour dans un ordre défini
- Choix libre entre action de lettre ou devinette
- Pas de limitation du nombre d'actions par tour
- Passage automatique au joueur suivant après action

### 🎯 Actions Disponibles

**Proposition de Lettre**
- Validation de la lettre (non proposée précédemment)
- Recherche dans tous les mots adverses actifs
- Révélation des positions exactes dans la vue personnelle
- Mise à jour immédiate de l'interface

**Tentative de Devinette**
- Sélection d'un adversaire cible
- Saisie du mot supposé
- Validation exacte (respect de la casse et accents)
- Conséquences selon le résultat

### ⚖️ Système de Conséquences

**Devinette Réussie**
- Élimination immédiate du joueur ciblé
- Attribution de points au devineur
- Bonus potentiels selon les conditions
- Mise à jour des statuts

**Devinette Échouée**
- Pénalité éventuelle selon le mode
- Continuité normale du jeu
- Apprentissage des mauvaises pistes

### 🏆 Conditions de Victoire

**Victoire Traditionnelle**
- Dernier joueur actif restant
- Élimination de tous les adversaires
- Reconnaissance de survivant

**Champion des Devinettes**
- Meilleur score cumulé
- Récompense de l'efficacité offensive
- Titre honorifique

**Confrontation Finale**
- Départage si différents vainqueurs
- Épreuve décisive chronométrée
- Couronnement du vainqueur absolu

## Systèmes de Contenu

### 🌍 Gestion des Thèmes

**Catégories de Thèmes Disponibles**

| Domaine       | Sous-Thèmes Exemples                | Complexité  |
|---------------|-------------------------------------|-------------|
| 🌍 Géographie | Pays, Capitales, Fleuves, Montagnes | Variable    |
| 📚 Culture    | Auteurs, Œuvres, Mythologies        | Élevée      |
| 🧪 Sciences   | Découvertes, Inventions, Théories   | Spécialisée |
| 🎨 Arts       | Peintres, Compositeurs, Courants    | Modérée     |
| 🏆 Sports     | Clubs, Athlètes, Événements         | Accessible  |
| 🦸 Fiction    | Héros, Univers, Créatures           | Ludique     |


**Base de Données des Mots**
- Structure hiérarchique par thème
- Métadonnées complètes pour chaque entrée
- Système de tags et de relations
- Historique des utilisations

### 🧮 Système de Difficulté Intelligent

**Calcul du Score de Difficulté**
```
Score = BaseLongueur + RaretéLettres + RaretéCulturelle

BaseLongueur = (longueur - 4) * coefficient
RaretéLettres = Σ(fréquence_lettre_rare * poids)
RaretéCulturelle = niveau_connu (1-3) * multiplicateur
```

**Plages de Difficulté par Niveau**
- 🎈 Facile : Scores 1-10 (mots courants et courts)
- ⚖️ Normal : Scores 11-20 (mots standards)
- 🔥 Difficile : Scores 21+ (mots complexes et rares)

**Validation Automatique**
- Vérification de l'appartenance au thème
- Contrôle du niveau de difficulté
- Validation orthographique et syntaxique
- Prévention des doublons indésirables

### 🎲 Équilibrage Dynamique

**Adaptation en Temps Réel**
- Ajustement des points selon la complexité
- Bonus/malus contextuels
- Équilibrage des chances
- Prévention des situations bloquantes

**Système de Rareté Culturelle**
- Niveau 1 : Très connu (ex: "FRANCE")
- Niveau 2 : Moyennement connu (ex: "COMORES")
- Niveau 3 : Rare (ex: "KIRIBATI")

## Modes de Jeu

### 🧭 Mode Classique (Tirage Secret)

**Concept Central**
Tous les joueurs explorent le même thème général, mais chacun reçoit une sélection personnelle de mots sans chevauchement, garantissant l'équité et le secret.

**Mécanismes de Sécurité**
- Distribution aléatoire équilibrée
- Listes personnelles uniques
- Aucune fuite d'information
- Expérience compétitive pure

**Avantages**
- Équilibre parfait entre les joueurs
- Focus sur la déduction pure
- Absence de biais thématiques

### 🧠 Mode Spécialiste

**Liberté Thématique**
Chaque joueur choisit individuellement son domaine de prédilection, créant une expérience personnalisée et valorisant l'expertise.

**Variantes Disponibles**
- Thèmes visibles (connaissance des domaines adverses)
- Thèmes secrets (mystère total)
- Thèmes mixtes (partiellement révélés)

**Stratégies Emergentes**
- Exploitation de ses propres connaissances
- Anticipation des choix adverses
- Adaptation à la diversité thématique

### 🌀 Mode Chaos

**Révolution des Règles**
Autorisation explicite des doublons de mots, transformant complètement la dynamique du jeu et introduisant bluff et psychologie.

**Conséquences des Doublons**
- Devinette réussie élimine tous les détenteurs
- Alliances involontaires entre joueurs similaires
- Stratégies de groupe émergentes

**Éléments Psychologiques**
- Détection des similitudes
- Utilisation du bluff
- Gestion du risque collectif

### ✍️ Mode Création Libre

**Expression Personnelle**
Saisie manuelle des mots avec système de validation intelligent préservant l'équilibre tout en permettant la créativité.

**Processus de Validation**
- Vérification de l'existence dans le thème
- Contrôle du niveau de difficulté
- Validation orthographique
- Prévention des incohérences

**Avantages Créatifs**
- Personnalisation avancée
- Adaptation aux connaissances du groupe
- Innovation thématique

## Interface et Expérience Utilisateur

### 🖥️ Concept d'Interface Console

**Affichage Principal du Tour**
```
═══════════════════════════════════════════════
🎮 AENIGMA - Tour de [Nom Joueur]
═══════════════════════════════════════════════
📜 VOTRE MOT SECRET : [Mot Complet]
🔠 LETTRES DÉCOUVERTES : [Lettres Trouvées]
🎯 VOTRE PROGRESSION :
[Joueur 1] : [Progression Personnelle]
[Joueur 2] : [Progression Personnelle]
...
═══════════════════════════════════════════════
💡 ACTIONS DISPONIBLES :
1. Proposer une lettre
2. Deviner un mot adverse
3. Consulter l'aide
4. Sauvegarder la partie
═══════════════════════════════════════════════
```

**Éléments d'Interface Avancés**
- Codes couleur pour différents types d'information
- Barres de progression visuelles
- Messages contextuels adaptatifs
- Aide intégrée accessible instantanément

### 📱 Principes d'Interface Graphique Future

**Design Philosophy**
- Clarté et lisibilité prioritaires
- Feedback visuel immédiat des actions
- Adaptation responsive aux différentes plateformes
- Cohérence visuelle across les écrans

**Composants Interface**
- Grilles de progression interactives
- Historiques d'actions consultables
- Statistiques en temps réel
- Personnalisation de l'affichage

## Gestion des Données

### 💾 Système de Sauvegarde

**Structure de Sauvegarde**
- Format universel (JSON/XML/binaire)
- État complet de la partie
- Métadonnées techniques
- Historique des actions
- Configuration des préférences

**Fonctionnalités de Persistance**
- Sauvegarde manuelle à tout moment
- Sauvegarde automatique périodique
- Gestion des versions de sauvegarde
- Reprise sécurisée après interruption

### 🗃️ Base de Données de Contenu

**Architecture des Données de Jeu**
- Structure hiérarchique des thèmes
- Métadonnées enrichies pour chaque mot
- Statistiques d'utilisation
- Système de tags et catégorisation

**Gestion des Extensions**
- Ajout facile de nouveaux thèmes
- Import/export de contenus personnalisés
- Validation automatique des nouvelles entrées
- Compatibilité ascendante des données

## Plan de Développement Évolutif

### Phase 1 : Noyau Fondamental
- [ ] Architecture de base indépendante du langage
- [ ] Structures de données fondamentales
- [ ] Moteur de règles de base
- [ ] Système de tour par tour
- [ ] Gestion des joueurs et des mots

### Phase 2 : Mécaniques Avancées
- [ ] Vue personnelle asymétrique complète
- [ ] Système de points et bonus
- [ ] Calcul dynamique de difficulté
- [ ] Gestion des pénalités et récompenses
- [ ] Validation des règles complexes

### Phase 3 : Modes de Jeu
- [ ] Implémentation du mode Classique
- [ ] Développement du mode Spécialiste
- [ ] Intégration du mode Chaos
- [ ] Finalisation du mode Création Libre
- [ ] Système de sélection de mode

### Phase 4 : Expérience Utilisateur
- [ ] Interface console avancée
- [ ] Système d'aide et tutoriel
- [ ] Gestion d'erreurs robuste
- [ ] Personnalisation des paramètres
- [ ] Documentation utilisateur complète

### Phase 5 : Contenu et Équilibrage
- [ ] Base de données de thèmes étendue
- [ ] Système d'équilibrage automatique
- [ ] Outils de création de contenu
- [ ] Tests d'équilibre complets
- [ ] Optimisation des performances

### Phase 6 : Fonctionnalités Étendues
- [ ] Système d'IA pour joueurs virtuels
- [ ] Statistiques et analytics
- [ ] Mode campagne avec progression
- [ ] Système d'accomplissements
- [ ] Export des données de partie

## Règles et Scénarios Complets

### 📜 Règles Détaillées du Jeu

**Début de Partie**
1. Configuration initiale (mode, difficulté, variantes)
2. Sélection/attribution des thèmes
3. Choix secret des mots par les joueurs
4. Initialisation des vues personnelles
5. Détermination de l'ordre de jeu

**Déroulement Standard**
- Tours séquentiels dans l'ordre établi
- Liberté d'action complète à chaque tour
- Accumulation progressive des informations
- Possibilité de devinette à tout moment

**Conditions d'Élimination**
- Devinette correcte de son mot par un adversaire
- Validation exacte du mot complet
- Passage immédiat en statut éliminé
- Conservation du score acquis

### 🏆 Système de Récompenses

**Points de Base**
- Devinette standard réussie : 2 points
- Utilisation de buzz : 3 points (bonus +1)
- Bonus de rareté : +1 à +3 points supplémentaires

**Reconnaissances**
- Vainqueur traditionnel : Dernier actif
- Champion des devinettes : Meilleur score
- Vainqueur absolu : Gagnant de la confrontation

### ⚔️ Confrontation Finale

**Déclenchement**
- Différence entre vainqueur traditionnel et champion
- Accord des joueurs concernés
- Configuration spécifique de l'épreuve

**Déroulement**
1. Choix croisé des thèmes
2. Sélection secrète des mots finaux
3. Révélation d'une lettre aléatoire par mot
4. Duel chronométré (2 minutes par défaut)
5. Victoire par devinette correcte ou timeout

**Résolution**
- Victoire immédiate en cas de devinette correcte
- Victoire du traditionnel en cas d'égalité ou timeout
- Couronnement du vainqueur absolu

### 🎭 Scénarios d'Exemple

**Partie Standard 4 Joueurs**
- Thème : Pays du monde
- Mots : FRANCE, JAPON, BRÉSIL, ÉGYPTE
- Durée : 15-20 tours
- Issues possibles variées

**Partie Expert Mode Chaos**
- Thème : Super-héros
- Doublons autorisés
- Stratégies de groupe émergentes
- Dynamique sociale accentuée

**Partie Création Libre**
- Thèmes personnalisés
- Mots inventés ou rares
- Adaptation aux connaissances du groupe
- Expérience unique et mémorable

---