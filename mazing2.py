"""
Sanctum Mazing Script - Pathing Approach.

Usage: Run through console (doesn't work interactively).
With the script's current folder as working directory:
python mazing2.py MAP_NAME TIME
The map you want to search for paths on, and time in minutes
(decimals allowed) for which to run. Will use all but one of
your CPUs. Will create a bunch of .json files in its current
folder, or use the ones already found there to continue
working on them.
The pathing approach is faster than the blocking approach,
but so far only works on maps (or rather: contiguous fields)
interpretable as coming from a single spawn and moving into
a single core, i.e. on fields that have a clear entry and exit
zone, though these can be comprised of multiple cells.
Other than those zones, the field must not be adjacent to or
contain unbuildable passable cells.
Example maps that support this by default are Park, Outpost,
The End, but notably also Bog. Example maps that can be made
to support this by splitting are Bio Lab, Cliff Lodge.
Maps that do not work include for example
* The Gate (separate spawns attached to the same field)
* Com Tower (passable nonbuildable cells in the field)
* Labyrinth (separate cores attached to the same field)
The maps currently actually implemented in this version
of the script are:
* Park

@author: MarioVX
Created on Tue Jan 5 2021.
"""
from time import monotonic
import json
from random import sample
from sys import argv
import multiprocessing as mp

# cellkeys = {0: 'buildable', 1: 'passable', 2: 'spawn', 3: 'core',
#			  4: 'impassable', 5: 'built'}

park = {'name':'Park',
		'map':((4,4,4,4,4,4,4,4,4,4,4,0,0,0,0,0),
			   (1,1,1,1,0,0,0,0,0,4,4,0,0,0,0,0),
			   (2,1,1,1,0,0,0,0,0,4,4,0,0,0,0,0),
			   (1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0),
			   (1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0),
			   (4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0),
			   (4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0),
			   (4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0),
			   (4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0),
			   (4,4,4,4,0,0,0,0,0,0,0,0,0,0,4,0),
			   (4,4,4,4,1,1,1,1,0,0,0,0,0,0,0,0),
			   (4,4,4,4,1,3,3,1,0,0,0,0,0,0,0,0),
			   (4,4,4,4,1,3,3,1,0,0,0,4,0,0,0,0),
			   (4,4,4,4,1,1,1,1,0,0,0,0,0,0,0,0)),
		'entrances':{(1,4):5, (2,4):4, (3,4):5, (4,4):6},
		'exits':{(9,4):3, (9,5):2, (9,6):2, (9,7):3,
				 (10,8):3, (11,8):2, (12,8):2, (13,8):3}}

maplib = {'Park': park,}

def printmap(ma: tuple):
	"""
	Nicely print a map

	Args:
		ma (tuple): The map to print.

	Returns:

	"""
	for x in ma:
		print(*x)
	print()
	return None

def neighbors(pos: tuple, ma: tuple, ob: bool) -> set:
	"""
	Find the pathable neighbors of a given cell in a map.

	Args:
		pos (tuple): coordinates of the cell whose neighbors to find.
		ma (tuple): map in which this takes place.
		ob (bool): whether or not to only accept buildable neighbors

	Returns:
		set: set of coordinates of neighbors.

	"""
	neighbors = set()
	for shift in ((1,0),(-1,0),(0,1),(0,-1)):
		x,y = pos[0]+shift[0], pos[1]+shift[1]
		if min(x,y)>=0 and x<len(ma) and y<len(ma[x]):
			if ma[x][y]==0 or ((not ob) and ma[x][y]<4):
				neighbors.add((x,y))
	return neighbors

def changed(ma: tuple, cha: dict) -> tuple:
	"""
	Give a new map with changes applied to it from a precursor.

	Args:
		ma (tuple): The precursor map to which to apply the changes.
		cha (dict): position -> value to set.

	Returns:
		tuple: The updated map.

	"""
	m = list(list(row) for row in ma)
	for c in cha:
		m[c[0]][c[1]] = cha[c]
	return tuple(tuple(row) for row in m)

def reachable(ma: tuple, dest: tuple, cp: tuple, ob: bool) -> bool:
	"""
	Check whether the destination can still be reached.

	Args:
		ma (tuple): The map on which reachibility is tested.
		dest (tuple): coordinates of the destination.
		cp (tuple): coordinates of the current position.
		ob (bool): Whether to only accept buildable neighbors.

	Returns:
		bool: True if the destination can still be reached, false if not.

	"""
	m = max(ma[cp[0]][cp[1]],ma[dest[0]][dest[1]])
	if (m > 3) or (ob and m > 0):
		return False
	if cp == dest:
		return True
	past, present, future = set(), {cp,}, set()
	while present:
		for pos in present:
			for pos2 in neighbors(pos, ma, ob):
				if pos2 == dest:
					return True
				if not (pos2 in past or pos2 in present):
					future.add(pos2)
			past.add(pos)
		present, future = future, set()
	return False

def getlength(path: tuple, ma: tuple, lo: int) -> int:
	"""
	Calculate the length of a given complete path.
	It is assumed that the path is paved with passable cells.

	Args:
		path (tuple): The completed path.
		ma (tuple): The original map.
		lo (int): Length from spawn to start + from destination to core.

	Returns:
		int: Total length of the path.

	"""
	l = lo
	for i in range(len(ma)):
		for j in range(len(ma[i])):
			if (path[i][j] == 1) and (ma[i][j] == 0):
				l += 1
	return l

def complete(sto: list, sta: list, dest: tuple, ma: tuple,
			 lo: int, t: float):
	"""
	Calculates all the completed paths from a given stack of incomplete paths.

	Args:
		sto (list): previous store of longest completed paths found thus far.
		sta (list): list of tuples (path, current end position)
		dest (tuple): Coordinates of the destination.
		ma (tuple): Original map.
		lo (int): Length from spawn to start + from destination to core.
		t (float): Time in minutes after which to terminate

	Returns:
		list: [list, list], new store and remaining stack.

	"""
	store, stack, end = sto, sta, monotonic()+t*60
	bestl = 0
	if store:
		bestl = getlength(sto[0], ma, lo)
	while stack and monotonic()<end:
		node = stack.pop()
		prepath = changed(node[0], {node[1]:1})
		if node[1] == dest:
			l = getlength(prepath, ma, lo)
			if l>bestl:
				store.clear()
				bestl = l
			if l==bestl:
				store.append(prepath)
		else:
			nbrs = neighbors(node[1], prepath, True)
			for newpos in nbrs:
				others = nbrs.difference({newpos,})
				changes = dict((p,5) for p in others)
				newpath = changed(prepath, changes)
				if reachable(newpath, dest, newpos, True):
					stack.append((newpath,newpos))
	return [store,stack]

def initiate(ma: dict, i: tuple, o: tuple, t: float) -> list:
	"""
	Initiate path generation on a given map from a particular entry
	to a particular exit.
	At the end of the time, saves its intermediary results to a json file.

	Args:
		ma (dict): The map dictionary on which to generate paths.
		i (tuple): coordinates of the particular entry to use.
		o (tuple): coordinates of the particular exit to use.
		t (float): Time in minutes after which to terminate

	Returns:
		list: [list, list], store of the longest paths and remaining stack.

	"""
	instoblock = set(x for x in ma['entrances'].keys() if x!=i)
	outstoblock = set(x for x in ma['exits'].keys() if x!=o)
	toblock = set.union(instoblock, outstoblock)
	changes = dict((x,5) for x in toblock)
	seed = changed(ma['map'], changes)
	results = complete([], [(seed,i),], o, ma['map'],
					   ma['entrances'][i]+ma['exits'][o], t)
	with open(ma['name']+str(i)+str(o)+".json", "x") as db:
		json.dump(results, db)
	return results

def contin(ma: dict, i: tuple, o: tuple, t: float) -> list:
	"""
	Continue the path generation  on the given map with entry and exit.

	Args:
		ma (dict): The map dictionary on which paths are generated.
		i (tuple): coordinates of the entry.
		o (tuple): coordinates of the exit.
		t (float): Time in minutes after which to terminate.

	Returns:
		list: [list, list], store of the longest paths and remaining stack.

	"""
	with open(ma['name']+str(i)+str(o)+".json", "r") as db:
		stosta = json.load(db)
		for node in stosta[1]:
			node[1] = tuple(node[1])
			node[0] = tuple(tuple(row) for row in node[0])
			node = tuple(node)
		for path in stosta[0]:
			path = tuple(tuple(row) for row in path)
	results = complete(stosta[0], stosta[1], o, ma['map'],
					   ma['entrances'][i]+ma['exits'][o], t)
	with open(ma['name']+str(i)+str(o)+".json", "w") as db:
		json.dump(results, db)
	return results

def workio(ma: dict, i: tuple, o: tuple, t: float) -> list:
	"""
	Work on the given in-out pair on the given map for some time.
	Calls contin or initiate, depending on whether a stosta file already
	exists in the working directory.

	Args:
		ma (dict): map dictionary to work on.
		i (tuple): entry coordinates.
		o (tuple): exit coordinates.
		t (float): time in minutes.

	Returns:
		list: [list, list], store of the longest paths and remaining stack.

	"""
	try:
		result = contin(ma, i, o, t)
	except:
		result = initiate(ma, i, o, t)
	return result

def integstores(stores: list, los: list, ma: tuple) -> list:
	"""
	Integrate a list of stores, i.e. select the one with longest paths and
	pool ties.

	Args:
		stores (list): a list of stores from various entry-exit pairs.
		los (list): a list of length offsets of the same pairs in same order.
		ma (tuple): original map, raw layout.

	Returns:
		list: [best length, store list]

	"""
	if len(stores)!=len(los):
		raise ValueError("stores and offsets need to be of same length.")
	res = list()
	bestl = 0
	for i in range(len(los)):
		if len(stores[i]):
			currentl = getlength(stores[i][0], ma, los[i])
			if currentl == bestl:
				res.extend(stores[i])
			if currentl > bestl:
				res = stores[i][:]
				bestl = currentl
	return [bestl, res]

def getstores(ios: list, ma: dict) -> list:
	"""
	Obtain a list of stores from files from a list of entry-exit pairs.

	Args:
		ios (list): list of entry-exit pairs.
		ma (dict): map dictionary.

	Returns:
		list: list of stores.

	"""
	stores = list()
	for io in ios:
		try:
			with open(ma['name']+str(io[0])+str(io[1])+".json", "r") as x:
				sto = json.load(x)[0]
				stores.append(sto)
		except:
			stores.append([])
	return stores

def getunfinishedios(ma: dict, maxnum: int) -> list:
	"""
	Get a sample of unfinished entry-exit coordinate pairs for the given map.

	Args:
		ma (dict): The map for which to do this.
		maxnum (int): Maximum size of the sample.

	Returns:
		list: [((i1x,i1y),(o1x,o1y)),...] of pairs.

	"""
	allpairs = list((x,y) for x in ma['entrances'] for y in ma['exits'])
	unfin = list()
	for pair in allpairs:
		try:
			with open(ma['name']+str(pair[0])+str(pair[1])+".json", "r") as x:
				nonempty = bool(json.load(x)[1])
				if nonempty:
					unfin.append(pair)
		except:
			unfin.append(pair)
	if len(unfin) <= maxnum:
		return unfin
	return sample(unfin, maxnum)

if __name__ == '__main__':
	ma = maplib[argv[1]]
	t = float(argv[2])
	print("Map:",ma['name'])
	print("Time:",str(int(t)),"minutes",str(int(60*(t-int(t)))),"seconds.")
	n = mp.cpu_count() - 1
	ios = getunfinishedios(ma, n)
	if not ios:
		print("Congratulations: the map",ma['name'],"has been solved!")
	processes = list()
	for io in ios:
		p = mp.Process(target=workio, args=(ma, io[0], io[1], t))
		processes.append(p)
		p.start()
	for p in processes:
		p.join()
		p.close()
	ios = list((x,y) for x in ma['entrances'] for y in ma['exits'])
	los = list(ma['entrances'][io[0]]+ma['exits'][io[1]] for io in ios)
	stores = getstores(ios, ma)
	intstore = integstores(stores, los, ma['map'])
	print("Longest path length:",intstore[0])
	print("Example path:")
	printmap(intstore[1][0])
	with open(ma['name']+".json", "w") as db:
		json.dump(intstore, db)
	print("Saved and done.")
