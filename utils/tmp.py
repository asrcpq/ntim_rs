from word2zrm import word2zrm
import sys
p = sys.argv[1]
a = open(f"{p}/tmp.txt").read()
a = a.strip().split('\n')
b = [word2zrm(ll.split()[0]) for ll in a]
with open(f"{p}/out.txt", "w") as f:
	for i in range(len(a)):
		if b[i] is None:
			continue
		print(f"{b[i]} {a[i]}", file = f)
