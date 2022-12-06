import	argparse
import	numpy as np
import	os
import	random

def generate_precincts(precincts_x,precincts_y):
	assert precincts_x >= 0 and precincts_y >= 0
	precincts_map = [[random.randint(0,100) for i in range(precincts_y)] for j in range(precincts_x)]
	is_file,copy_nb = True, 0
	while is_file:
		filename = f"{precincts_x}_{precincts_y}_{copy_nb}.txt"
		is_file = os.path.isfile(f"./exemplaires/{filename}")
		copy_nb += 1

	with open('./exemplaires/'+filename,'w+') as f:
		f.write(str(precincts_x) + " " + str(precincts_y) + "\n")
		np.savetxt(f,np.array(precincts_map),fmt="%-3.0i")

if __name__=="__main__":
	parser = argparse.ArgumentParser()
	parser.add_argument("-x","--x",help="Le nombre de municipalites le long de l’axe des x",required=True)
	parser.add_argument("-y","--y",help="Le nombre de municipalites le long de l’axe des y",required=True)
	args=parser.parse_args()

	if not os.path.exists('./exemplaires'):
		os.makedirs('exemplaires')

	generate_precincts(int(args.x),int(args.y))
