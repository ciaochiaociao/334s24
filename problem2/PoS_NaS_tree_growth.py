import numpy as np
import matplotlib.pyplot as plt
from tqdm import tqdm
import random

# Constants
time_slots = 20000  # 1 minute in milliseconds
probability = 0.001

# Simulation setup
block_counts = [1]  # the number of blocks at each height in the NaS tree; start with the genesis block at height 0
heights = [0]  # the heights of each node

total_blocks = [1]  # track the total number of blocks at each timestamp
max_heights = [0]  # track the height of the tree at each timestamp

for t in tqdm(range(1, time_slots)):
    new_blocks = np.random.binomial(sum(block_counts), probability)
    block_counts.append(new_blocks)
    total_blocks.append(total_blocks[-1] + new_blocks)
    # print(total_blocks)

    # binomial process only specify the number of blocks that succeed. It does not say which blocks are successful.
    # Therefore, we choose nodes based on the number
    node_indexes = random.sample(range(len(heights)), new_blocks)

    if new_blocks > 0:
        last_node_index = len(heights)
        for i in range(new_blocks):
            # print(i)
            parent_node_index = node_indexes[i]  # choose a random node as the parent node

            # new node
            node_index = last_node_index + i

            # add the new node to the tree
            assert node_index >= len(heights)
            if node_index >= len(heights):
                heights.append(0)
            heights[node_index] = heights[parent_node_index] + 1  # grow a new node from the parent node
    max_heights.append(max(heights))

# Plot the results
fig, axs = plt.subplots(2, 1, figsize=(12, 10))

# Plot total number of blocks over time
axs[0].plot(range(time_slots), total_blocks)
axs[0].set_yscale('log')
axs[0].set_xlabel('Time (milliseconds)')
axs[0].set_ylabel('Total Number of Blocks')
axs[0].set_title('Growth of Total Number of Blocks Over Time')

# Plot tree height over time
axs[1].plot(range(time_slots), max_heights)
axs[1].set_yscale('log')
axs[1].set_xlabel('Time (milliseconds)')
axs[1].set_ylabel('Tree Height')
axs[1].set_title('Growth of Height of the Tree Over Time')

plt.tight_layout()
plt.show()

# save to png
fig.savefig('tree_growth.png')
