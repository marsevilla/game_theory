# game_theory

## Règles

Deux stratégies s’affrontent.
À chaque tour, chaque stratégie choisit simultanément :
Coopérer (C)
Trahir (D)

Chaque créateur de stratégie est libre de décider de comment cette décision est prise par son programme.

Les gains pour un tour sont :
C / C → 3 points chacun
D / D → 1 point chacun
D / C → 5 points pour le traître, 0 point pour le coopérant

Chaque confrontation dure 200 tours

Toutes les stratégies s’affrontent elles-mêmes et toutes les autres stratégies.
Le gagnant est la stratégies ayant obtenu le score cumulé le plus élevé sur l'ensemble des confrontations.


# Création de stratégie

Les 5 stratégies de bases sont:

- Always defect : trahis à chaque tour

- Always cooperate : coopère à chaque tour

- Tit For Tat : coopère au premier tour pour copie le dernier coup de l'adverse

- Grim Trigger : coopère tant que l'adversaire n'a jamais trahis sinon, trahis systématiquement

- Alternator : alterne coopèrer et trahir à chaque tour

## Nouvelle stratégie 

Pour mettre en place votre stratégie, créer un nouveau fichier <stategy>_<initial>.rs dans le dossier "strategies", renommez-le en SNAKE CASE selon le nom de vore tratégie (ex: grim_trigger_ge.rs).

Merci de ne pas nommer deux stratégie avec le même nom


