from PIL import Image

# create the Sierpiński triangle (using rule 18)

game = []

def rule18(i,j):
	global game
	try:
		left = game[i-1][j-1]
	except IndexError:
		left = 0
	try:
		middle = game[i-1][j]
	except IndexError:
		middle = 0
	try:
		right = game[i-1][j+1]
	except:
		right = 0
	if left == 0:
		if middle == 0:
			if right == 0:
				game[i][j] = 0
			else:
				game[i][j] = 1
		elif middle == 1:
			game[i][j] = 0
	else:
		if middle == 0:
			if right == 0:
				game[i][j] = 1
			else:
				game[i][j] = 0
		else:
			game[i][j] = 0

a = 10000

subgame = [0] * (2*a-1)

for i in range(a):
	game.append(list(subgame))

game[0][int((2*(a-1))/2)] = 1

for i in range(1,len(game)):
	for j in range(len(game[0])):
		rule18(i,j)

"""
for thing in game:
	print(" ".join(map(str, thing)))
"""

imgx = len(game[0])
imgy = len(game)
image = Image.new("RGB",(imgx,imgy))

for i in range(len(game)):
	for j in range(len(game[0])):
		if game[i][j] == 1:
			image.putpixel((j,i),(0,0,0))
		else:
			image.putpixel((j,i),(255,255,255))
            
image.save("cellular_automata.png","PNG")
