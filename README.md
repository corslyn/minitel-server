# Minitel Server

Un serveur Vidéotex en Rust ! Enfin possible d'utiliser le réseau X.25 Transpac ! Internet n'a qu'à bien se tenir...

Maintenant avec plus de ~~services~~ factures !

## Demo

https://www.youtube.com/watch?v=-qG73u3dQIU

## Fonctionnalités

- [x] Envoi de pages Vidéotex générées par MiEdit (https://minitel.cquest.org/) ou enregistrées avec un micro
- [x] Gestion du modem si on utilise pas de Minitel retourné
- [x] Fonctionne avec une ligne téléphonique classique, à tester avec une ligne VoIP
- [ ] Websocket pour utiliser l'émulateur MiniPavi (https://www.minipavi.fr/emulminitel/indexws.php)
- [x] Création simple™ de serveur via une arborescence JSON
- [ ] Modification du port du modem ou autodétection
- [ ] Modification de la chaîne d'initialisation
- [ ] Gestion des arguments

## Mode d'emploi

### Installation
```bash
git clone https://github.com/corslyn/minitel-server.git
cd minitel-server
cargo run
```

En cas de problème de permission, vérifier les droits du port série (/dev/ttyUSB0):

```bash
ls -l /dev/ttyUSB0
crw-rw---- 1 root dialout 188, 0 Aug  4 22:06 /dev/ttyUSB0
```

Et ajoutez votre utilisateur au groupe `dialout` :

```bash
sudo usermod -aG dialout $USER
``` 

Après la réouverture de session, cela devrait fonctionner.

### Structure de navigation

Les différentes pages sont définies dans le fichier `pages.json` :

```json
[
  {
    "name": "teletel",
    "path": "ecrans/teletel.vdt",
    "routes": {
      "METEO": "meteo"
    },
    "guide": "teletel.guide"
  },
  {
    "name": "meteo",
    "path": "ecrans/meteo.vdt",
    "routes": {
      "cx": "teletel"
    }
  },
  {
    "name" : "teletel.guide",
    "path" : "ecrans/teletel.guide.vdt",
    "routes": {
      "retour": "teletel"
    }
  }
]
```

Une route en MAJUSCULE définit un code de service (télétel), et en minuscule une touche spéciale (Connexion/Fin, Retour).

## Bugs

Certainement


## Remerciements

- https://minitel.cquest.org/ : Génération de pages
- https://jbellue.github.io/stum1b/ : Spécification du Minitel
- https://x0r.fr/blog/43 : Pour les premiers essais avec mgetty et un début de serveur bash
- https://noelmrtn.fr/posts/v23_server/ : Pour la chaîne d'initialisation V.23 pour mon modem (U.S. Robotics 56k Message Modem)

