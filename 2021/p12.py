# Python program to print all paths from a source to destination.

from collections import defaultdict

# This class represents a directed graph
# using adjacency list representation
class Graph:

    def __init__(self):
        # No. of vertices
        #self.V = vertices
        
        # default dictionary to store graph
        self.graph = defaultdict(list)
        self.paths = 0

    # function to add an edge to graph
    def addEdge(self, u, v):
        self.graph[u].append(v)

    def addEdgeFromString(self, s):
        [v1, v2] = s.split("-")
        self.addEdge(v1, v2)
        self.addEdge(v2, v1)

    '''A recursive function to print all paths from 'u' to 'd'.
    visited[] keeps track of vertices in current path.
    path[] stores actual vertices and path_index is current
    index in path[]'''
    def printAllPathsUtil(self, u, d, visited, path):
        # TODO: uppercase vertices can be visited any times, 
        # lowercase vertices can be visited at most twice,
        # start and end can only be visited once
        if not u.isupper():
            visited[u] = True
        if u not in visited:
            # first time
            visited[u] = 1
        else:
            visited[u] = True
        path.append(u)

        # If current vertex is same as destination, then print
        # current path[]
        if u == d:
            print (path)
            self.paths += 1
        else:
            # If current vertex is not destination
            # Recur for all the vertices adjacent to this vertex
            for i in self.graph[u]:
                if i not in visited or visited[i]== False or visited[i] == 1:
                    self.printAllPathsUtil(i, d, visited, path)
                    
        # Remove current vertex from path[] and mark it as unvisited
        path.pop()
        if visited[u] == 1:
            visited[u]= False
        elif visited[u] == True:
            visited[u] = 1


    # Prints all paths from 's' to 'd'
    def printAllPaths(self, s, d):

        # Mark all the vertices as not visited
        visited ={}

        # Create an array to store paths
        path = []

        # Call the recursive helper function to print all paths
        self.printAllPathsUtil(s, d, visited, path)



# Create a graph given in the above diagram
input = """CI-hb
IK-lr
vr-tf
lr-end
XP-tf
start-vr
lr-io
hb-qi
end-CI
tf-YK
end-YK
XP-lr
XP-vr
lr-EU
tf-CI
EU-vr
start-tf
YK-hb
YK-vr
start-EU
lr-CI
hb-XP
XP-io
tf-EU"""
g = Graph()
for line in input.splitlines():
    g.addEdgeFromString(line.strip())
g.printAllPaths('start', 'end')
print(g.paths)
