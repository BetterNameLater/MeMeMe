# TODO

- essayer de refactor les `Query` -> vu que l'ancien `Bundle` est maintenant un component (donc avec un tag), on peut get le truc en question directement -> est-ce qu'on a aussi toujours besoin de `Item` ??
- tester les `primitives` et les `items`
- le single use ne fonctionne pas
- `elapsed_time_from_start_rewind_system` devrait passer en premier dans le schedule
- `LevelInformations` : `player_start_position` why mutable ? should be set by the level asset config ?
- (pour player_plugin notament) : n'enregistrer les systeme que quand on est dans le bon state
- sub-state pour in-level (genre : `Idle`, `Playing`, `Winning`, `Reseting`, ...)
- renomer `IsActivated` en `IsEmitting`, ou quelque chose de plus clair (quand la plaque de pression est activée)
  - aussi, peut etre evité le `bool` dedans, pour juste faire avec les queries