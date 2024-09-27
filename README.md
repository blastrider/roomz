# ROOMZ

Ce programme est une application web de gestion de salles de réunion, développée en Rust avec le framework Actix-web et utilisant Diesel pour interagir avec une base de données PostgreSQL. Voici un aperçu des fonctionnalités principales :

### Fonctionnalités principales :

1. **Gestion des Salles de Réunion :**

   - Permet de créer, lister, rechercher, et vérifier la disponibilité des salles de réunion.
   - Les salles peuvent être filtrées en fonction de leur capacité, des équipements disponibles, et de leur emplacement.

2. **Réservation de Salles :**

   - Les utilisateurs peuvent réserver des salles pour des créneaux horaires spécifiques.
   - Le programme vérifie la disponibilité des salles avant de confirmer une réservation pour éviter les conflits.

3. **Vérification de Disponibilité :**

   - Une fonctionnalité clé permet de vérifier si une salle est disponible pour une période donnée, en tenant compte des réservations existantes.

4. **Gestion des Conflits :**

   - Le programme évite les doubles réservations en s'assurant que les périodes réservées ne se chevauchent pas.

5. **Journalisation des Événements :**
   - Toutes les opérations importantes, telles que la création de salles ou les réservations, sont enregistrées dans un fichier de log journalier pour assurer un suivi complet des activités.

### Technologies utilisées :

- **Rust et Actix-web** : Pour construire une API web performante et sécurisée.
- **Diesel ORM** : Pour interagir avec la base de données PostgreSQL.
- **Logging** : Les événements sont enregistrés dans des fichiers de log rotatifs pour le suivi et le débogage.

### Cas d'utilisation :

Ce programme est idéal pour les entreprises ou les organisations qui ont besoin de gérer plusieurs salles de réunion et de s'assurer qu'elles sont utilisées de manière efficace, tout en évitant les conflits de réservation. Les administrateurs peuvent également surveiller l'utilisation des salles via les journaux générés.

Si vous avez besoin d'ajouter des fonctionnalités supplémentaires ou d'intégrer ce programme dans un système plus large, le code est structuré de manière modulaire pour faciliter les modifications et les extensions.
