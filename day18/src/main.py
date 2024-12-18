import heapq
from collections import defaultdict

data_path = "../data/data.txt"

with open(data_path, "r") as file:
    data_raw = file.read()

data = []
for line in data_raw.splitlines():
    s = line.split(",")
    data.append((int(s[0]), int(s[1])))


class Simulation:
    def __init__(
        self,
        size: int,
        byte_locations: list[tuple[int, int]],
        start_pos: tuple[int, int],
        end_pos: tuple[int, int],
    ):
        self._width = size
        self._height = size
        self._byte_locations = byte_locations
        self._byte_count = len(byte_locations)
        self._simulated_bytes = 0
        self._start_pos = start_pos
        self._end_pos = end_pos
        self._grid = self._generate_grid()

    def _generate_grid(self) -> list[list[str]]:
        return [list("." * self._width) for _ in range(self._height)]

    def simulate_bytes(self, count: int):
        assert count <= self._byte_count - self._simulated_bytes
        new_count = self._simulated_bytes + count
        for byte_col, byte_row in self._byte_locations[
            self._simulated_bytes : new_count
        ]:
            self._grid[byte_row][byte_col] = "#"

        self._simulated_bytes = new_count

    def get_last_byte(self):
        return self._byte_locations[self._simulated_bytes - 1]

    def print_grid(self):
        print("\n".join(["".join(line) for line in self._grid]))

    def reset_grid(self):
        self._grid = self._generate_grid()
        s._simulated_bytes = 0

    def find_shortest_path(self):
        # Dijkstra’s Algorithm https://www.geeksforgeeks.org/dijkstras-shortest-path-algorithm-greedy-algo-7/
        queue = []
        heapq.heappush(queue, (0, self._start_pos))
        directions = [(-1, 0), (0, 1), (1, 0), (0, -1)]

        distance_map = defaultdict(lambda: float("inf"))
        distance_map[self._start_pos] = 0

        while queue:
            distance, (row, col) = heapq.heappop(queue)
            for dr, dc in directions:
                nr, nc = row + dr, col + dc
                # skip out of bound
                if nr < 0 or nc < 0 or self._height <= nr or self._width <= nc:
                    continue
                # skip corrupted memory cell
                if self._grid[nr][nc] == "#":
                    continue
                if distance_map[(nr, nc)] > distance_map[(row, col)] + 1:
                    distance_map[(nr, nc)] = distance_map[(row, col)] + 1
                    heapq.heappush(queue, (distance_map[(nr, nc)], (nr, nc)))
        return distance_map[self._end_pos]


s = Simulation(71, data, (0, 0), (70, 70))
s.simulate_bytes(1024)
print("Part 1:", s.find_shortest_path())
# check all bytes until path cost is infinite
for i in range(1024, s._byte_count):
    # add new byte
    s.simulate_bytes(1)
    # find shortest path again
    path_cost = s.find_shortest_path()
    # if no shortest path return last byte
    if path_cost >= float("inf"):
        b = s.get_last_byte()
        print("Part 2:", f"{b[0]},{b[1]}")
        break
