import numpy as np
import matplotlib.pyplot as plt
import networkx as nx

# Parameters
probability = 0.001  # Probability of finding a child block in each slot
initial_block = 1  # Genesis block
time_limit = 60000  # Total number of time slots (1 minute)
np.random.seed(42)  # For reproducibility

# Function to simulate the growth of the NaS tree
def simulate_tree_growth(probability):
    G = nx.DiGraph()
    G.add_node(0)  # Genesis block
    current_nodes = [0]
    next_node_id = 1
    total_nodes = 1

    for timestamp in range(1, time_limit + 1):  # 60000
        new_blocks = 0
        # print("current_nodes", current_nodes)
        for node in current_nodes:  # binomial: N nodes
            if np.random.rand() < probability:  # each node conducts a bernouli process
                G.add_edge(node, next_node_id)
                next_node_id += 1
                new_blocks += 1

        if new_blocks > 0:
            print("next_node_id - new_blocks", next_node_id - new_blocks)
            print("next_node_id", next_node_id)
            current_nodes.extend(range(next_node_id - new_blocks, next_node_id))  # next_node_id - new_blocks is the id of the earliest new node of each iteration
            total_nodes += new_blocks

        if total_nodes > 100:
            break

    return G, total_nodes

# Run the simulation
tree, total_nodes = simulate_tree_growth(probability)

# Plot the NaS tree when it first exceeds 100 nodes
def plot_tree(tree):
    pos = nx.spring_layout(tree, seed=42)
    plt.figure(figsize=(12, 12))
    nx.draw(tree, pos, with_labels=True, node_size=500, node_color='skyblue', font_size=10, font_color='black', edge_color='gray')
    plt.title('NaS Tree Structure When Exceeding 100 Nodes')
    plt.show()

plot_tree(tree)
