import collections
import itertools

data_path = "../data/data.txt"

data_raw = """kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"""

# with open(data_path, "r") as file:
#     data_raw = file.read()


def parse_data(data: str):
    connections = collections.defaultdict(set)
    for line in data.splitlines():
        split = line.split("-")
        connections[split[0]].add(split[1])
        connections[split[1]].add(split[0])
    return connections


connections = parse_data(data_raw)


def bors_kerbosch(R, P, X, G, C):

    if len(P) == 0 and len(X) == 0:
        if len(R) > 2:
            C.append(R)
        return

    for v in list(P):
        bors_kerbosch(R.union({v}), P.intersection(G[v]), X.intersection(G[v]), G, C)
        P.remove(v)
        X.add(v)


c = connections
C1 = []
bors_kerbosch(set(), set(c.keys()), set(), c, C1)

out = set()
for c1 in C1:
    c1_len = len(c1)
    if c1_len == 3:
        out.add(tuple(sorted(c1)))
    elif c1_len > 3:
        for i in itertools.combinations(c1, 3):
            out.add(tuple(sorted(i)))

part1 = len([i for i in out if any(e.startswith("t") for e in i)])
print("Part 1:", part1)
print("Part 2:", ",".join(sorted(max(C1, key=len))))
