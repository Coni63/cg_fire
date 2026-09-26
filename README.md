
# Ta mission

Un feu vient de démarrer dans une forêt. Il va se propager et détruire son environnement composé de trois types de cellules :

- des cellules de grande valeur, représentées par des maisons
- des cellules de petite valeur, représentées par des arbres
- des cellules sécurisées, représentées par un sol beige

Tu as la possibilité de sécuriser des cellules afin de bloquer la propagation du feu. Cette action sera appelée par la suite "raser" (une cellule). Cela te permettra d'empêcher certaines cellules de brûler. Cependant, cette action prend du temps, et le feu ne va pas t'attendre. Ainsi, ton but est de protéger le plus de ressources possible (somme des valeurs des cellules) avant que le feu ne les atteigne.

# Règles

## Terrain (grille)

Une cellule est soit sécurisée, soit un arbre, soit une maison. Chaque type de cellule non sécurisée (arbre / maison) a trois paramètres :

- fireDuration : nombre de tours durant lequel le feu reste dans cette cellule avant de se propager dans les cellules adjacentes non sécurisées et pas encore brûlées.
- cutDuration : nombre de tours nécessaires pour sécuriser cette cellule.
- value : score que représente cette cellule, donc perdu si cette dernière est brûlée ou rasée. 

Ces trois paramètres sont les mêmes pour tous les arbres, et les mêmes pour toutes les maisons. Ces paramètres (6 valeurs) te sont fournis, ainsi que les dimensions du terrain, la cellule de départ du feu, et le type de chaque cellule. L'objectif est de sauvegarder autant de valeur que possible en bloquant la propagation du feu dans certaines zones.

## Propagation du feu
Une cellule en feu le reste pendant fireDuration tours. Chaque cellule a un fireProgress incrémenté à chaque tour. Une fois que fireProgress atteind fireDuration, la cellule est considérée brûlée, et le feu se propage dans les cellules adjacentes valides. Ces dernières sont dans les 4 directions (Nord, Sud, Est, Ouest), et sont valides si elles ne sont ni brûlées ni sécurisées (pour lesquelles fireProgress vaut -1).

## Bloquer la propagation du feu
Tu peux raser des cellules afin de les sécuriser et bloquer la propagation du feu. Quand tu rases une cellule, un cooldown est initialisé avec la valeur cutDuration de la cellule. Il diminuera alors sur les prochains tours jusqu'à 0. Tant que le cooldown n'est pas zéro, tu ne peux pas indiquer une autre cellule à raser.
Tu ne peux pas raser les cellules déjà sécurisées (ce serait inutile) et celles en feu (parce que le feu, ça brûle). Autrement, la simulation prend fin. En revanche, une cellule est considérée sécurisée dès que tu commences à la raser. Autrement dit, le feu dans une cellule adjacente ne pas pas se propager dans la cellule où tu es.

## Calcul du score
Il s'agit d'un jeu d'optimisation. Ainsi ton but est de sauvegarder autant de valeur (des cellules) que possible. Le score pour un test est la somme des valeurs des cellules qui n'ont pas brulé ou été rasées. Le score pour ce jeu est la somme des scores de chaque test.

*Note*: Les tests ne peuvent pas échouer. Ne rien faire amènera simplement un score de 0. Si la simulation est interrompue à cause d'un timeout ou d'une mauvaise commande, la propagation du feu sera calculée jusqu'à extinction, avant de calculer le score final.

# Game input
## Initialization input
- 1ère ligne: trois entiers séparés par des espaces treeCuttingDuration, treeFireDuration et treeValue, les paramètres des cellules "arbres", comme expliqués dans les règles.
- 2ème ligne: trois entiers séparés par des espaces houseCuttingDuration, houseFireDuration et houseValue, les paramètres des cellules "maisons", comme expliqués dans les règles.
- 3ème ligne: deux entiers séparés par des espaces width, height, la largeur et la hauteur du terrain (grille).
- 4ème ligne: deux entiers séparés par des espaces fireStartX, fireStartY, la position où le feu démarre.
- height Prochaines lignes: séquence de caractères de longueur width représentant une ligne de la grille. Chaque caractère représente le type d'une cellule. '#' est une cellule sécurisée. '.' est une cellule "arbre". 'X' est une cellule "maison".

## Input for one game turn
- 1ère ligne: un entier cooldown, le nombre de tours avant que tu puisses donner l'instruction de raser une cellule (≥ 1 signifie que tu dois attendre (WAIT) / == 0 signifie que tu peux donner une cellule à raser).
- Prochaines height lignes: width entiers séparés par des espaces représentant la progression du feu (fireProgress) de chaque cellule d'une ligne de la grille.

  - fireProgress == -2 signifie que la cellule est sécurisée
  - fireProgress == -1 signifie qu'il n'y a pas de feu
  - 0 ≤ fireProgress < fireDuration signifie que la cellule est en feu
  - fireProgress == fireDuration signifie que la cellule est brûlée

## Output for one game turn
Une ligne contenant soit "WAIT" si ton cooldown is strictement positif, soit "x y" la position de la cellule à raser. Tu peux également attendre (WAIT) si ton cooldown est zéro, mais ce n'est pas vraiment productif :p
Si le format de l'output donné est invalide ou si tu essaies de raser une cellule alors que ton cooldown n'est pas zéro, alors tu perds. Si tu donnes une cellule à raser, alors au début de ton prochain tour, ton cooldown sera à cellCuttingDuration-1.

### Contraintes
- 0 ≤ width, height ≤ 50
- 0 ≤ cellFireDuration ≤ 10
- 0 ≤ cellCuttingDuration ≤ 10
- 0 ≤ cellValue
- 8000 ≤ [Somme de cellValue pour chaque cellule] ≤ 12000
- Toutes les cellules en bordure de terrain seront sécurisées (SAFE).

Temps de calcul maximal pour le 1er tour : 5000 ms.
Temps de calcul maximal pour les tours suivants : 100 ms.
