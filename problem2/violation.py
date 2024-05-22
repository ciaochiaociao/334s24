import matplotlib.pyplot as plt

# Parameters for adversary
a = 0.25  # rate of A-blocks per minute
simulation_time = 200 # total simulation time in minutes
delta = 0.4  # pacer interval

# Simulate A-block process
np.random.seed(42)
a_block_times = np.cumsum(np.random.exponential(1/a, int(simulation_time * a * 2)))

# Filter out A-blocks beyond simulation_time
a_block_times = a_block_times[a_block_times <= simulation_time]

def simulate_and_plot(violate_safety):
    # Shift H-block times for maximal delay scenario
    delayed_h_block_times = []
    pacers = [h_block_times[0]]
    for time in h_block_times[1:]:
        if time >= pacers[-1] + delta:
            pacers.append(time)
            if violate_safety and len(delayed_h_block_times) < len(a_block_times) + 1:
                # Delay H-block to immediately after the adversarial blocks
                delayed_h_block_times.append(a_block_times[len(delayed_h_block_times)] + 1e-9)
            else:
                delayed_h_block_times.append(time)
    
    # Plotting
    plt.figure(figsize=(12, 6))
    plt.plot(h_block_times, np.zeros_like(h_block_times), 'ro', label='H-blocks')
    plt.plot(delayed_h_block_times, np.zeros_like(delayed_h_block_times), 'ro', alpha=0.3, label='Delayed H-blocks')
    plt.plot(pacers, np.zeros_like(pacers), 'b+', markersize=10, label='Pacers')
    plt.plot(a_block_times, np.zeros_like(a_block_times), 'go', label='A-blocks')
    plt.yticks([])
    plt.xlabel('Time (minutes)')
    plt.title(f'Mining Process with {"Violation" if violate_safety else "No Violation"} of Safety Height 1')
    plt.legend()
    plt.grid(True)
    plt.show()

# Plot case where safety of height 1 is violated
simulate_and_plot(violate_safety=True)

# Plot case where safety of height 1 is not violated
simulate_and_plot(violate_safety=False)
