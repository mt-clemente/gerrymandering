#!/usr/bin/env python3

# INF8775 - Analyse et conception d'algorithmes
#   TP3 - Compétition de Gerrymandering
#
#   AUTEUR :
#     HAOUAS, Mohammed Najib - 25 novembre 2020
#
#   RÉSUMÉ DES CHANGEMENTS :
#     12/11/2020 - Ajout de l'argument -p dans les exemples de cette présente documentation
#     12/7/2020  - Correction du calcul de la norme L1
#     12/4/2020  - Introduction d'un argument facultatif pour l'affichage de la Machine Readable Zone, maintenant désactivée par défaut. 
#     12/4/2020  - Ajout de l'option -r/--do-transpose pour lire correctement les solutions des équipes qui ont considéré une convention inverse à celle d'algèbre linéaire.
#                  Cette option est pour débogage immédiat et ne sera pas utilisée lors de la correction. Assurez-vous de mettre à jour votre affichage avant la remise.
#     11/26/2020 - Inversion des dimensions indiquées dans les instances pour suivre celle de generator.py.
#     11/26/2020 - Disponibilité initiale.
#
#   USAGE :
#     Ce script vérifie le stdout passé en pipe pour conformité avec les exigences du TP3 tel que rédigé à la session A20.
#     Ce même script sera utilisé par les correcteurs pour juger la qualité des programmes développés avec l'appel suivant :
#       $ (timeout 180s ./tp.sh -e FICHIER_EXEMPLAIRE -c NB_CIRCONSCRIPTIONS -p; exit 0) | ./check_sol.py -e FICHIER_EXEMPLAIRE -c NB_CIRCONSCRIPTIONS [-s FICHIER_SORTIE]
#     où :
#       * "timeout 180s" interrompt, après 3 minutes (180 sec), l'exécution de...
#       * "./tp.sh -e FICHIER_EXEMPLAIRE -c NB_CIRCONSCRIPTIONS -p" tel que...
#       * "FICHIER_EXEMPLAIRE" est l'adresse de l'exemplaire à résoudre et...
#       * "NB_CIRCONSCRIPTIONS" est la variable m dans le problème, ie le nombre visé de circonscriptions.
#       * "; exit 0" remplace le code de sortie de timeout pour permettre à...
#       * "|" de pipe la sortie de "tp.sh" après interruption par timeout à...
#       * "./check_sol.py" qui prend des paramètres *obligatoirement identiques* à ceux passés à "tp.sh" mais aussi...
#       * admet, de façon facultative, un paramètre "-s" avec le chemin/nom du fichier "FICHIER_SORTIE" où on désire sauvergarder le contenu en pipe.
#
#   EXEMPLES D'USAGE :
#     $ (timeout 180s ./tp.sh -e 3_3_0.txt -c 3 -p; exit 0) | ./check_sol.py -e 3_3_0.txt -c 3 -s sortie.out
#       cette commande exécute tp.sh pour un max de 180s et passe son affichage à ce présent script, appelé avec les mêmes paramètres pour vérifier le résultat.
#       L'affichage est par ailleurs sauvegardé dans un fichier texte "sortie.out" dans le même dossier.
#       Sauvegarder cet affichage de tp.sh permet de le revérifier ultérieurement, sans avoir à réexécuter le programme, en utilisant par exemple la commande ci-dessous.
#
#     $ cat sortie.out | ./check_sol.py -e 3_3_0.txt -c 3
#       sortie.out contient l'affichage d'une exécution antérieure. L'utilisation de "cat" permet de vérifier la solution qu'il contient par ce présent script.
#       Prendre garde à utiliser les mêmes paramètres avec ce script que ceux qui ont été employés lors de la génération de "sortie.out".
#     
#     $ ./tp.sh -e 3_3_0.txt -c 3 -p > sortie.out
#     $ cat sortie.out | ./check_sol.py -e 3_3_0.txt -c 3
#       Alternativement, il est possible de lancer vos programmes dans un premier temps et d'enregistrer leur sortie avec ">"...
#       pour ensuite les vérifier plus tard avec la deuxième commande.
#
#   ATTENTION:
#     Pour que la commande :
#       $ (timeout 180s ./tp.sh -e FICHIER_EXEMPLAIRE -c NB_CIRCONSCRIPTIONS -p; exit 0) | ./check_sol.py -e FICHIER_EXEMPLAIRE -c NB_CIRCONSCRIPTIONS [-s FICHIER_SORTIE]
#     prenne en compte toutes vos solutions, il est INDISPENSABLE de flush votre stdout (ie, l'affichage standard de votre programme) À CHAQUE FOIS qu'une solution est trouvée.
#     Pour ce faire, après chaque affichage d'une solution améliorante:
#       * pour python 3: appelez sys.stdout.flush() ou spécifiez l'argument flush=True dans print().
#       * pour C : stdout est flushed automatiquement après un saut de ligne (ie impression de '\n' ou appel de println) ou appelez fflush(stdout).
#       * pour C++ : insérez (<<) à std::cout soit std::endl (qui flush automatiquement après un saut de ligne) soit std::flush.
#       * pour Java : System.out flush automatiquement à chaque saut de ligne ou appelez System.out.flush().
#
#     Il est nécessaire de rendre ce script exécutable en utilisant chmod +x
#     Python 3.5 ou ultérieur recommandé pour lancer ce script.


import sys
import re
import math
import argparse


def load_instance(instance_path):
    with open(instance_path,'r') as instance_stream:
        # Process first line which defines problem characteristics
        line_one = next(instance_stream)
        if not re.match("^\\s*\\d+\\s*\\d+\\s*$", line_one):
            return 1

        dim_j, dim_i = [int(x) for x in line_one.split()]

        instance_data = []
        for line in instance_stream:
            instance_data.append([int(x) for x in line.split()])
            if len(instance_data[-1]) != dim_j:
                return 1

    if len(instance_data) != dim_i:
        return 1

    return dim_i, dim_j, instance_data


def is_solution_format_valid(raw_solution):
    # ^\s*(?:\d+\s+\d+\s+)*\d+\s+\d+\s*$
    target_pattern = "^\\s*(?:\\d+\\s+\\d+\\s+)*\\d+\\s+\\d+\\s*$"
    return bool(re.match(target_pattern, raw_solution))


def parse_solution(raw_solution, do_transpose):
    solution_data = []
    single_solution = []

    for line in raw_solution.splitlines():
        line_contents = line.split()

        if not line_contents and not single_solution:
            continue
        elif not line_contents and single_solution:
            solution_data.append(single_solution)
            single_solution = []
        else:
            raw_district = [int(x) for x in line_contents]
            single_solution.append([raw_district[x:x+2] for x in range(0, len(raw_district), 2)])
            if do_transpose:
                single_solution[-1] = [x[::-1] for x in single_solution[-1]]

    if single_solution:
        solution_data.append(single_solution)

    return solution_data


def check_inner_district_distances(solutions, n, m):
    for (solution, solution_index) in zip(solutions, range(len(solutions))):
        for (district, district_index) in zip(solution, range(len(solution))):
            for precinct1_index in range(len(district) - 1):
                for precinct2_index in range(precinct1_index + 1, len(district)):
                    if (abs(district[precinct1_index][0] - district[precinct2_index][0]) + abs(district[precinct1_index][1] - district[precinct2_index][1])) > math.ceil(n/2/m):
                        return (solution_index, district_index, precinct1_index, precinct2_index)

    return 0


# Error codes encapsulated into first element of tuple returned: 1 incomplete solution, 2 balance, 3 out of bounds, 4 reused
def check_consistency(solutions, dim_i, dim_j, m):
    n = dim_i*dim_j

    for (solution, solution_index) in zip(solutions, range(len(solutions))):
        total_count = 0
        usage_flags = [[False]*dim_j for _ in range(dim_i)]

        # Check whether number of districts is correct
        if len(solution) != m:
            return (1, solution_index)

        for (district, district_index) in zip(solution, range(len(solution))):
            # Check balance of districts
            if len(district) > math.ceil(n/m) or len(district) < math.floor(n/m):
                return (2, solution_index, district_index)
            
            total_count += len(district)

            for (precinct, precinct_index) in zip(district, range(len(district))):
                # Check whether bounds are correct
                if precinct[0] < 0 or precinct[0] >= dim_i or precinct[1] < 0 or precinct[1] >= dim_j:
                    return (3, solution_index, district_index, precinct_index)

                # Check whether precinct has been used before
                if usage_flags[precinct[0]][precinct[1]]:
                    return (4, solution_index, district_index, precinct_index)
                
                usage_flags[precinct[0]][precinct[1]] = True

        # Check whether all precincts have been used
        if total_count != n:
            return (1, solution_index)

    return 0


def compute_objective(solutions, instance, dim_i, dim_j, m):
    solution = solutions[-1]
    votes = [0]*len(solution)
    objective = 0

    for (district, district_index) in zip(solution, range(len(solution))):
        for precinct in district:
            votes[district_index] += instance[precinct[0]][precinct[1]]

        votes[district_index] /= len(solution[district_index])*100
        if votes[district_index] > 0.5:
            objective += 1

    return objective, votes


if __name__ == '__main__':
    # Parse arguments
    parser = argparse.ArgumentParser()
    parser.add_argument("-e", "--exemplaire", \
                        help="Représente l'exemplaire correspondant à la solution étudiée", \
                        action='store', required=True, metavar='FICHIER_EXEMPLAIRE')
    parser.add_argument("-c", "--nb-circonscriptions", \
                        help="Représente le nombre de circonscriptions (m) visé par la solution fournie", \
                        action='store', required=True, metavar='NB_CIRCONSCRIPTIONS', type=int)
    parser.add_argument("-s", "--sortie", \
                        help="Si indiqué, écrire stdin (ie la sortie de votre programme tel que piped) dans un fichier du nom indiqué", \
                        action='store', required=False, metavar='FICHIER_SORTIE')
    parser.add_argument("-r", "--do-transpose", \
                        help="Si utilisé, considérer la transposée de la solution fournie", \
                        action='store_true', required=False)
    parser.add_argument("-z", "--disp-mrz", \
                        help="Si utilisé, affiche une Machine-Readable Zone pour permettre l'automatisation de la collecte de données", \
                        action='store_true', required=False)
    args = parser.parse_args()

    # Store pipe
    piped_content = sys.stdin.read()

    # Save pipe if requested
    try:
        if args.sortie:
            with open(args.sortie,'w') as f_output:
                f_output.write(piped_content)
            print("Info : affichage en pipe sauvegardé dans " + args.sortie + ".")
    except:
        print("Attention : impossible de sauvegarder l'affichage en pipe. Vérifiez les permissions d'écriture. Sauvegarde ignorée.")
    
    # Check whether format is as expected, ie an even number of space-separated integers on every line 
    if not is_solution_format_valid(piped_content):
        print("Erreur : les solutions fournies en pipe à stdin ont un format non valide. Revoyez la convention discutée dans l'énoncé.", file=sys.stderr)
        print("A reçu :", file=sys.stderr)
        print(piped_content, file=sys.stderr)
        sys.exit(1)

    # Load instance corresponding to solution
    instance_data = None
    try:
        instance_data = load_instance(args.exemplaire)
    except:
        print("Erreur : impossible d'ouvrir le fichier de l'exemplaire.", file=sys.stderr)
        sys.exit(1)
    
    if instance_data == 1 and instance_data is not None:
        print("Erreur : l'exemplaire fourni en argument à ce script de vérification a un format non valide. "\
            "Vérifiez le chemin et/ou le contenu de l'exemplaire.", file=sys.stderr)
        sys.exit(1)

    # Structure piped solution in memory
    resolution_data = parse_solution(piped_content, args.do_transpose)
    
    # Check solutions' consistency
    consistency_result = check_consistency(resolution_data, instance_data[0], instance_data[1], args.nb_circonscriptions)
    if consistency_result != 0:
        print("Erreur : une ou plusieurs des solutions fournies en pipe à stdin présentent un problème de consistance.", file=sys.stderr)

        if consistency_result[0] == 1:
            print("Raison : la solution " + str(consistency_result[1]) \
                  + " (0-indexé) contient un nombre inadéquat de circonscriptions et/ou de municipalités.", file=sys.stderr)
        elif consistency_result[0] == 2:
            print("Raison : la circonscription " + str(consistency_result[2]) + " de la solution " + str(consistency_result[1]) \
                  + " (0-indexés) ne respecte pas le nombre prescrit de municipalités.", file=sys.stderr)
        elif consistency_result[0] == 3:
            print("Raison : la municipalité " + str(consistency_result[3]) + " de la circonscription "  \
                  + str(consistency_result[2]) + " de la solution " + str(consistency_result[1]) \
                  + " (0-indexés) est représentée par des coordonnées hors-limites.", file=sys.stderr)
        elif consistency_result[0] == 4:
            print("Raison : la municipalité " + str(consistency_result[3]) + " de la circonscription " + str(consistency_result[2]) \
                  + " de la solution " + str(consistency_result[1]) + " (0-indexés) est réutilisée.", file=sys.stderr)
        
        sys.exit(1)

    # Check precinct distance constraint consistency
    distance_consistency_result = check_inner_district_distances(resolution_data, instance_data[0]*instance_data[1], args.nb_circonscriptions)
    if distance_consistency_result != 0:
        print("Erreur : une ou plusieurs des solutions fournies en pipe à stdin présentent un problème " \
              "de proximité des municipalités dans l'une ou plusieurs de leurs circonscriptions.", file=sys.stderr)
        print("Raison : la distance entre les municipalités " + str(distance_consistency_result[2]) \
              + " et " + str(distance_consistency_result[3]) \
              + " de la circonscription " + str(distance_consistency_result[1]) \
              + " de la solution " + str(distance_consistency_result[0]) \
              + " (0-indexés) dépasse la valeur limite.", file=sys.stderr)
        sys.exit(1)

    # Satisfied by the solutions' presentation, compute best objective
    objective, vote_proportions = compute_objective(resolution_data, instance_data[2], instance_data[0], instance_data[1], args.nb_circonscriptions)
    print("OK : la valeur de l'objectif de la dernière (ie, meilleure) solution fournie est de " + str(objective) + ".\n")
    print("n = " + str(instance_data[0]) + "*" + str(instance_data[1]) + " = " + str(instance_data[0]*instance_data[1]))
    print("m =", args.nb_circonscriptions)
    print("Nombre de solutions reçues : " + str(len(resolution_data)))
    if len(vote_proportions) < 15:
        print("Proportions des voix pour les verts :", vote_proportions)

    # Machine readable zone to facilitate automatic logging
    if args.disp_mrz:
        print("\nCSV Machine Readable Format:")
        print(objective, instance_data[0]*instance_data[1], args.nb_circonscriptions, args.exemplaire, ','.join(["{:.3f}".format(x) for x in vote_proportions]), sep=',')
