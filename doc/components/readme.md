# Components

## ECS - components

blabla ici on va parler des components de 3Me

## Player

TODO images

## Items

TODO image

### DebugName(String)

Permets de donner un nom aux items.

### GhostOnly

Certains items ne peuvent interagir qu'avec les fantômes.

### PlayerOnly

Certains items ne peuvent interagir qu'avec le joueur.

### IsOn

Est-ce que l'item répond aux conditions qu'il faut pour être "allumé".

La condition commune est qu'il doit être "IsUsable" pour émettre un "IsOn".

### Dependencies(Vec<Entity>)

Permets de définir de quelles entités un item dépend pour être utilisable.

TODO: on se rend compte tout de suite que ça ne peut pas check que l'item ne soit pas "IsOn".
(donc est-ce qu'on veut un système de dépendance négative)

### IsUsable

En fonction de certaines conditions, un item peut être activé, donc utilisable.

Il ne prend pas de boolean. Le fait d'avoir ce tag en soi rend l'item utilisable. Cela permet de ne query les items
utilisables.

### IsActivated(bool)

C'est quoi la diff avec `IsOn` et `IsUsable` ?

### EnterAble

Pour détecter quand un joueur / un fantôme entre dans la case d'un item.

### PeopleOn(usize)

Le nombre de personnes sûr un item.

Besoin pour quoi ?

### Teleporter(Vec2i)

Le téléporter et l'endroit où il téléporte.

### LevelTeleporter(String)

Il t'amène dans un autre niveau, via le nom.

### ToggleInteract

TODO

### ToggleOnEnter

TODO