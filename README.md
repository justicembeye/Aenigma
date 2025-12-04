# Aenigma

# 🌍 Projet : **Aenigma(1.0)**

## **1. Description générale du projet**
Ce projet consiste à développer un **jeu de devinettes de pays** en langage C. Chaque joueur choisit un nom de pays en secret, et les joueurs posent des questions pour deviner les noms des pays des autres joueurs. Le jeu se termine lorsque tous les noms sont devinés, sauf un. Le dernier joueur restant est déclaré **vainqueur traditionnel**. Un système de points récompense également les joueurs qui devinent les pays des autres, créant ainsi un **Champion des devinettes**. En cas d'égalité, une **confrontation finale** est organisée pour déterminer le **vainqueur absolu**. 

---

## **2. Règles du jeu**
### **2.1 Déroulement général**
1. Chaque joueur choisit un nom de pays de manière secrète.
2. Le joueur actif choisit une lettre et demande à tous les autres joueurs si cette lettre est présente dans leur nom de pays.
   - Si oui, les autres joueurs révèlent les positions exactes de la lettre.
   - Si non, le joueur suivant prend la main.
3. Si un joueur devine complètement le nom d’un autre joueur, ce dernier est éliminé.
4. Le dernier joueur avec un nom non deviné est déclaré **vainqueur traditionnel**.

### **2.2 Système de points et Champion des devinettes**
- Chaque fois qu'un joueur devine correctement le pays d'un autre joueur, il gagne **2 points**.
- Si un joueur utilise un **buzz** pour deviner rapidement, il gagne un **bonus de 1 point** (total de 3 points).
- Le joueur avec le plus de points à la fin de la partie est déclaré **Champion des devinettes**.

### **2.3 Confrontation finale en cas d'égalité**
- Si le **vainqueur traditionnel** et le **Champion des devinettes** ne sont pas la même personne, une **confrontation finale** est organisée pour déterminer le **vainqueur absolu**.
- La confrontation se déroule comme suit :
  1. Chaque joueur choisit un **continent** dans lequel l'autre joueur devra choisir un pays.
  2. Chaque joueur choisit **secrètement** un pays dans le continent désigné.
  3. Une **lettre aléatoire** est révélée dans chaque pays choisi.
  4. Les joueurs doivent **deviner le pays de l'autre** en posant des questions sur les lettres.
  5. La confrontation se déroule en un **temps défini** (par exemple, 2 minutes).
     - Si un joueur devine correctement le pays de l'autre avant la fin du temps, il est déclaré **vainqueur absolu**.
     - Si aucun des deux joueurs ne devine le pays de l'autre dans le temps imparti, le **vainqueur traditionnel** est déclaré gagnant par défaut.

### 2.4 Simulations de parties
1. Exemple de déroulement d'une partie normale

Initialisation :

3 joueurs (Joueur A, Joueur B, Joueur C) choisissent secrètement un pays.

Les pays choisis sont : Joueur A = "France", Joueur B = "Japon", Joueur C = "Brésil".

Tour 1 :

Joueur A choisit la lettre "E".

Joueur B révèle que "E" n'est pas dans "Japon".

Joueur C révèle que "E" est dans "Brésil" (position 4).

Affichage : Joueur C = _ _ _ E _.

Tour 2 :

Joueur B choisit la lettre "A".

Joueur A révèle que "A" est dans "France" (position 2).

Joueur C révèle que "A" n'est pas dans "Brésil".

Affichage : Joueur A = _ A _ _ _.

Tour 3 :

Joueur C choisit la lettre "S".

Joueur A révèle que "S" n'est pas dans "France".

Joueur B révèle que "S" n'est pas dans "Japon".

Aucune lettre révélée.

Tour 4 :

Joueur A devine que le pays de Joueur C est "Brésil".

Joueur C est éliminé.

Joueur A gagne 2 points.

Fin de partie :

Joueur B devine que le pays de Joueur A est "France".

Joueur A est éliminé.

Joueur B est déclaré vainqueur traditionnel.

Joueur A est déclaré Champion des devinettes avec 2 points.

6.2 Exemple de déroulement avec confrontation finale
Fin de partie :

Joueur A est le vainqueur traditionnel.

Joueur B est le Champion des devinettes avec 10 points.

Une confrontation finale est organisée.

Confrontation finale :

Joueur A choisit le continent Afrique pour Joueur B.

Joueur B choisit le continent Europe pour Joueur A.

Joueur A choisit secrètement "Allemagne".

Joueur B choisit secrètement "Égypte".

Une lettre aléatoire est révélée :

Joueur A = _ _ _ _ _ _ _ E (Allemagne).

Joueur B = _ _ _ _ T _ (Égypte).

Début de la confrontation :

Un chronomètre de 2 minutes est lancé.

Joueur A demande : "Est-ce que la lettre 'R' est dans ton pays ?"

Joueur B répond : "Non."

Joueur B demande : "Est-ce que la lettre 'M' est dans ton pays ?"

Joueur A répond : "Oui, en position 4."

Joueur A devine que le pays de Joueur B est "Égypte".

Joueur A est déclaré vainqueur absolu.


---

## **3. Objectifs pédagogiques**
Ce projet te permettra de pratiquer et de développer des compétences en :
- Manipulation des chaînes de caractères.
- Utilisation des structures en C.
- Pointeurs et allocation dynamique.
- Gestion des tableaux et des matrices.
- Boucles et conditions complexes.
- Gestion des fichiers pour la persistance des données (sauvegarde et chargement des parties).
- Gestion des entrées/sorties utilisateur.
- Implémentation de la logique de jeu et détection des gagnants.

---

## **4. Fonctionnalités à implémenter**

### **4.1 Gestion des joueurs**
- [ ] **4.1.1 Saisie du nom de pays de chaque joueur**  
- [ ] **4.1.2 Stocker les noms des pays des joueurs de manière cachée**  
- [ ] **4.1.3 Initialisation des lettres révélées pour chaque joueur**  

### **4.2 Tour de jeu**
- [ ] **4.2.1 Choisir une lettre par le joueur actif**  
- [ ] **4.2.2 Poser la question à chaque joueur sur la présence de la lettre**  
- [ ] **4.2.3 Révéler les positions de la lettre si elle est présente**  
- [ ] **4.2.4 Passer le tour au joueur suivant**  

### **4.3 Gestion des lettres révélées**
- [ ] **4.3.1 Stocker les lettres et positions révélées pour chaque joueur**  
- [ ] **4.3.2 Afficher l’état des lettres révélées pour chaque joueur**  

### **4.4 Détection de l'élimination**
- [ ] **4.4.1 Vérifier si le nom de pays d’un joueur est complètement deviné**  
- [ ] **4.4.2 Éliminer un joueur si son nom est deviné**  
- [ ] **4.4.3 Déclarer le dernier joueur restant comme vainqueur**  

### **3.5 Système de points et buzz**
- [ ] **4.5.1 Attribuer des points pour chaque pays deviné**  
- [ ] **4.5.2 Limiter le nombre de buzz par joueur (par exemple, 3 buzz par partie)**  
- [ ] **4.5.3 Autoriser le buzz uniquement si au moins 2 ou 3 lettres du pays ont été révélées** 

### **3.6 Confrontation finale**
- [ ] **4.6.1 Organiser une confrontation finale en cas d'égalité entre le vainqueur traditionnel et le Champion des devinettes**  
- [ ] **4.6.2 Implémenter le choix des continents et des pays pour la confrontation**  
- [ ] **4.6.3 Révéler une lettre aléatoire dans chaque pays choisi**  
- [ ] **4.6.4 Gérer la logique de devinette pendant la confrontation**  
- [ ] **4.6.5 Ajouter un chronomètre pour limiter la durée de la confrontation**  
- [ ] **4.6.6 Déclarer le vainqueur traditionnel en cas d'expiration du temps**  

### **4.5 Sauvegarde et chargement des parties**
- [ ] **4.7.1 Sauvegarder les noms des joueurs et leur état dans un fichier**  
- [ ] **4.7.2 Charger une partie sauvegardée à partir d’un fichier**  

---

## **5. Structure des données**

### **5.1 Structure Joueur**
```c
typedef struct {
    char nomPays[50];          // Le pays choisi par le joueur
    int lettresRevelees[50];   // Indique si une lettre a été révélée (1 = révélée, 0 = cachée)
    int estElimine;            // Indique si le joueur est éliminé
} Joueur;
```

**5.2 Tableaux de joueurs**

Un tableau de `Joueur` sera utilisé pour stocker les informations de tous les joueurs.

---

## **6. Menu principal**
Le menu principal proposera les options suivantes :
```plaintext
=== Menu Principal ===
1. Démarrer une nouvelle partie
2. Charger une partie sauvegardée
3. Sauvegarder la partie en cours
4. Afficher les règles du jeu
5. Quitter
```

Chaque option sera gérée dans une fonction distincte pour une meilleure organisation du code.

---

## **7. Plan de développement étape par étape**

### **Phase 1 : Gestion des joueurs**
- [ ] Implémenter la saisie des noms des joueurs.
- [ ] Initialiser les lettres révélées pour chaque joueur.

### **Phase 2 : Déroulement des tours**
- [ ] Implémenter la logique de choix d’une lettre.
- [ ] Révéler les positions des lettres pour chaque joueur.
- [ ] Passer le tour au joueur suivant.

### **Phase 3 : Gestion de l’élimination**
- [ ] Vérifier si un nom est entièrement deviné.
- [ ] Éliminer les joueurs devinés.
- [ ] Déclarer le vainqueur traditionnel.

### **Phase 4 : Système de points et buzz**
- [ ] Attribuer des points pour chaque pays deviné.
- [ ] Implémenter le système de buzz avec des conditions spécifiques.

### **Phase 5 : Confrontation finale**
- [ ] Organiser la confrontation finale en cas d'égalité.
- [ ] Implémenter le choix des continents et des pays pour la confrontation.
- [ ] Ajouter un chronomètre pour limiter la durée de la confrontation.

### **Phase 6 : Sauvegarde et chargement**
- [ ] Sauvegarder l’état des joueurs dans un fichier.
- [ ] Charger une partie à partir d’un fichier.

### **Phase 7 : Améliorations et finition**
- [ ] Ajouter un affichage plus clair des lettres révélées.
- [ ] Ajouter des statistiques (par exemple, nombre de tours joués).

---
