# 🎮 Aenigma / Occultus Mundus - Cahier des Charges Complet

## 📋 Table des Matières
- [Concept et Philosophie](#concept-et-philosophie)
- [Architecture Globale](#architecture-globale)
- [Mécaniques de Jeu Détaillées](#mécaniques-de-jeu-détaillées)
- [Système de Buzz Avancé](#système-de-buzz-avancé)
- [Modes de Révélation et Investigation](#modes-de-révélation-et-investigation)
- [Système de Progression et Pouvoirs](#système-de-progression-et-pouvoirs)
- [Interface et Expérience Utilisateur](#interface-et-expérience-utilisateur)
- [Modes de Jeu](#modes-de-jeu)
- [Plan de Développement](#plan-de-développement)

## Concept et Philosophie

### 🎯 Vision Fondamentale
Aenigma est un jeu de déduction stratégique multijoueur où l'investigation personnelle et la prise de risque calculée sont au cœur de l'expérience. Chaque joueur mène sa propre enquête pour découvrir les secrets adverses tout en protégeant le sien.

### 🧭 Principes de Conception
- **Investigation Asymétrique** : Chaque joueur construit sa propre vision du jeu
- **Prise de Risque Stratégique** : Le système de buzz récompense l'intuition et la déduction
- **Progression Personnelle** : Acquisition de capacités spéciales en cours de partie
- **Équilibre Dynamique** : Mécaniques adaptatives selon le niveau de difficulté

## Architecture Globale

### 📦 Structures Conceptuelles

**Entité Joueur**
- Identifiant unique et nom
- Mot secret personnel
- Statut (actif/éliminé)
- Score et ressources
- Vue d'investigation personnelle
- Capacités spéciales débloquées
- Historique des actions

**Entité Investigation**
- Grille de découvertes par adversaire
- Lettres révélées et leurs positions
- Certitudes et hypothèses
- Marqueurs de progression

**Entité Capacité Spéciale**
- Type (Voyance, Précision, etc.)
- Niveau de déverrouillage
- Coût d'activation
- Durée et effets

## Mécaniques de Jeu Détaillées

### 🔍 Système d'Investigation Ciblée

**Principe Fondamental**
Chaque action d'investigation doit cibler un adversaire spécifique. Le joueur ne peut pas interroger tous les adversaires simultanément avec une même lettre.

**Processus d'Investigation**
```
1. Sélection de l'action "Investigation"
2. Choix d'un adversaire cible
3. Proposition d'une lettre à rechercher
4. Vérification exclusive chez l'adversaire ciblé
5. Révélation conditionnelle dans la vue personnelle
```

**Conséquences Stratégiques**
- Nécessité de prioriser les cibles
- Construction progressive de profils adverses
- Impossibilité de profiter passivement des découvertes des autres
- Valorisation de la mémoire et de la déduction

### 🎯 Affichage des Informations Essentielles

**Éléments Obligatoires par Joueur**
- Son mot secret complet
- Les lettres de son mot déjà découvertes par les adversaires
- Sa progression personnelle sur chaque adversaire
- Les positions exactes des lettres découvertes
- Les actions disponibles et leurs coûts

## Système de Buzz Avancé

### ⚡ Mécanisme de Chargement du Buzz

**Concept du Condensateur**
Le buzz n'est pas toujours disponible. Il se "charge" progressivement pendant la partie selon un système temporel ou basé sur les actions.

**Modalités de Chargement**
- **Temps** : Le buzz se recharge après un délai fixe
- **Actions** : Le buzz se recharge après un nombre défini de tours
- **Mixte** : Combinaison temps et actions

**Cycle d'Utilisation**
```
[CHARGE COMPLÈTE] → [BUZZ DISPONIBLE] → [UTILISATION] → [DÉCHARGE] → [RECHARGE]
```

### 🎮 Conditions d'Activation du Buzz

**Restrictions Initiales**
- Impossible d'utiliser le buzz pendant les premiers tours
- Nécessité d'avoir collecté un minimum d'informations
- Condition de charge complète du "condensateur"

**Limitations d'Usage**
- Nombre maximum de buzz par partie (ex: 1-3)
- Délai de recharge entre chaque utilisation
- Coût en ressources ou points

### 🏆 Récompenses du Buzz Réussi

**Bonus de Base**
- Points bonus supérieurs à une devinette normale
- Gain immédiat si la devinette est correcte

**Avantages Stratégiques**
- **Accélérateur de recharge** : Temps de recharge réduit pour le prochain buzz
- **Charge supplémentaire** : Obtention d'une charge bonus
- **Pouvoir spécial** : Déverrouillage temporaire d'une capacité

**Système de Cumul**
- Deux buzz réussis : Obtention du "Don de Voyance"
- Trois buzz réussis : Capacité spéciale avancée
- Bonus progressifs selon le nombre de buzz réussis

### ⚠️ Pénalités du Buzz Échoué

**Option C : Révélation Forcée (Recommandée)**
- Le joueur doit révéler une lettre de son propre mot à tous les adversaires
- Pénalité stratégique significative
- Impact direct sur sa vulnérabilité

**Option D : Combinaison Équilibrée**
- Perte de points significative
- Saut de tour obligatoire
- Révélation partielle de son mot

**Conséquences Additionnelles**
- Réinitialisation du compteur de recharge
- Délai de recharge augmenté
- Perte de charges accumulées

## Modes de Révélation et Investigation

### 🔎 Deux Systèmes de Révélation

**Mode Ciblé (Normal/Difficile)**
- Investigation obligatoirement dirigée vers un adversaire spécifique
- Révélation des positions exactes des lettres trouvées
- Expérience stratégique et méthodique

**Mode Global (Facile)**
- Investigation appliquée à tous les adversaires simultanément
- Révélation standard des positions
- Expérience plus accessible et rapide

### 🔮 Don de Voyance - Système Avancé

**Concept**
Capacité spéciale permettant de voir les positions exactes des lettres sans investigation ciblée.

**Conditions de Déverrouillage**
- Réussite de deux buzz dans la même partie
- Accumulation d'un certain nombre de découvertes
- Achievement spécifique à atteindre

**Effets**
- Vision des positions pendant un nombre limité de tours
- Investigation avancée avec informations positionnelles
- Avantage tactique temporaire

**Équilibrage**
- Durée limitée de l'effet
- Coût en ressources ou points
- Délai de recharge important

## Système de Progression et Pouvoirs

### 🌟 Arbre de Progression

**Ressources de Progression**
- Points d'expérience par action réussie
- Charges de buzz accumulées
- Découvertes stratégiques
- Achievements spécifiques

**Capacités Débloquables**
- **Voyance** : Vision des positions exactes
- **Précision** : Investigation plus efficace
- **Intuition** : Chance accrue de buzz réussi
- **Résilience** : Réduction des pénalités

### 🎯 Système d'Achievements

**Catégories d'Achievements**
- **Investigation** : Nombre de lettres découvertes
- **Déduction** : Buzz réussis avec peu d'indices
- **Stratégie** : Éliminations rapides
- **Maîtrise** : Utilisation efficace des capacités

**Récompenses des Achievements**
- Déverrouillage de capacités permanentes
- Bonus cosmétiques ou interface
- Avantages en parties futures

## Interface et Expérience Utilisateur

### 🖥️ Écran de Jeu Détaillé

**Section Personnelle**
```
🧿 VOTRE ENQUÊTE PERSONNELLE
Mot Secret: E S P A G N E
Lettres découvertes par les autres: E, A, G
```

**Section Investigation**
```
🔍 VOS DÉCOUVERTES
Joueur 1: _ _ _ _ _ _ 
Joueur 2: _ T A _ I E  [Positions: 2,3,5]
Joueur 3: _ _ _ I _ _ _ 
```

**Section Actions et Ressources**
```
⚡ ÉNERGIE DE BUZZ: [██████░░░░] 60%
🎯 BUZZ DISPONIBLE: ✅ OUI
🔮 POUVOIRS ACTIFS: Aucun

ACTIONS:
1. Investiguer un joueur
2. Tenter une devinette  
3. Utiliser le Buzz (1 disponible)
4. Activer un pouvoir
```

### 🎨 Éléments d'Interface Avancés

**Indicateurs Visuels**
- Barre de progression du buzz
- Icônes de pouvoirs actifs
- Marqueurs de statut des adversaires
- Historique des actions récentes

**Feedback Contextuel**
- Animations de recharge du buzz
- Effets visuels pour les pouvoirs activés
- Notifications des achievements débloqués
- Alertes stratégiques

## Modes de Jeu

### 🎮 Variantes de Révélation

**Mode Investigation Pure**
- Révélation ciblée obligatoire
- Système de buzz avancé
- Pouvoirs et progression
- Expérience stratégique complète

**Mode Classique Adapté**
- Révélation globale disponible
- Buzz simplifié
- Focus sur la déduction de base
- Accessibilité maximale

### 🏆 Modes Compétitifs

**Mode Championnat**
- Parties successives avec progression conservée
- Déverrouillage de capacités entre parties
- Classement et ligues

**Mode Duel**
- Confrontation 1 contre 1
- Système de buzz intensifié
- Parties rapides et intenses

## Plan de Développement

### Phase 1 : Noyau Stratégique
- [ ] Système d'investigation ciblée
- [ ] Mécanique de base du buzz
- [ ] Interface d'information essentielle

### Phase 2 : Système Avancé
- [ ] Mécanisme de recharge du buzz
- [ ] Pouvoirs et capacités spéciales
- [ ] Arbre de progression

### Phase 3 : Équilibrage Expert
- [ ] Don de Voyance et capacités avancées
- [ ] Système d'achievements
- [ ] Équilibrage fin des pénalités/récompenses

### Phase 4 : Expérience Complète
- [ ] Modes de jeu variés
- [ ] Interface avancée et feedback
- [ ] Tutoriel et onboarding

---