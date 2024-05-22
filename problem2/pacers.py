import numpy as np
import matplotlib.pyplot as plt

# Parameters
h = 0.75  # rate of H-blocks per minute
delta = 0.4  # pacer interval
simulation_time = 200  # total simulation time in minutes

# Simulate H-block process
# np.random.seed(42)
h_block_times = np.cumsum(np.random.exponential(1/h, int(simulation_time * h)))  # the interval is exponential distribution; the arrival time can be obtained by cumulative sum

# Filter out H-blocks beyond simulation_time
h_block_times = h_block_times[h_block_times <= simulation_time]

# Identify pacers
pacers = [h_block_times[0]]
for time in h_block_times[1:]:
    if time >= pacers[-1] + delta:
        pacers.append(time)

# Plotting
plt.figure(figsize=(12, 6))
plt.plot(h_block_times, np.zeros_like(h_block_times), 'ro', label='H-blocks')
plt.plot(pacers, np.zeros_like(pacers), 'b+', markersize=10, label='Pacers')
plt.yticks([])
plt.xlabel('Time (minutes)')
plt.title('H-block Process with Pacers')
plt.legend()
plt.grid(True)
plt.show()
plt.savefig("pacers1.png")

# Calculate inter-arrival times of pacers
inter_arrival_times = np.diff(pacers)

# Estimate the average inter-arrival time
average_inter_arrival_time = np.mean(inter_arrival_times)

print("average_inter_arrival_time", average_inter_arrival_time)