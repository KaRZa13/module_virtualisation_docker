## Module virtualisation - Docker Compose

Ce projet fournit une stack complete avec :
- Backend Rust (port 8080)
- Frontend Nuxt (port 3000)
- Base de donnees PostgreSQL (port interne 5432)
- Motor Admin (port 9000)

## Installation

1) Cloner le depot et se placer a la racine du projet.
```bash
git clone https://github.com/KaRZa13/module_virtualisation_docker.git
cd module_virtualisation_docker
```

2) Lancer la stack en mode detache :
```bash
docker compose up -d --build
```

3) Verifier l'etat des services :
```bash
docker compose ps
```

## Services et ports

- Backend: http://localhost:8080
- Frontend: http://localhost:3000
- Motor Admin: http://localhost:9000
- PostgreSQL: acces interne via `db:5432`

## Commandes utiles

- Construire les images :
```bash
docker compose build
```

- Demarrer la stack :
```bash
docker compose up -d
```

- Voir les logs :
```bash
docker compose logs -f
```

- Redemarrer un service :
```bash
docker compose restart <service>
```

- Arreter la stack :
```bash
docker compose down
```

- Arreter et supprimer les volumes (reset DB) :
```bash
docker compose down -v
```

## Depannage rapide

- Rebuild complet :
```bash
docker compose up -d --build
```

- Voir les logs d'un service :
```bash
docker compose logs -f <service>
```
