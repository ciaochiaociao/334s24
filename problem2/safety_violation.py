import numpy as np

# Constants
a = 0.25  # rate of A-blocks
h = 0.75  # rate of H-blocks
Delta = 0.4  # minimum time difference between pacers
k = 6  # confirmation depth
total_time = 200  # total simulation time in minutes
num_simulations = 1000  # number of simulations

def simulate_mining(a, h, total_time):
    # Simulate A-blocks
    A_times = np.cumsum(np.random.exponential(1/a, int(total_time*a*2)))  # extra factor for safety
    A_times = A_times[A_times <= total_time]
    
    # Simulate H-blocks
    H_times = np.cumsum(np.random.exponential(1/h, int(total_time*h*2)))  # extra factor for safety
    H_times = H_times[H_times <= total_time]
    
    return A_times, H_times

def find_pacers(H_times, Delta):
    pacers = [0]  # Genesis block at time 0
    for time in H_times:
        if time >= pacers[-1] + Delta:
            pacers.append(time)
    return pacers

def safety_violation(A_times, H_times, pacers, k):
    # Check if any A-block violates the safety of height 1 within the first k pacers
    for pacer_time in pacers[:k+1]:  # Include genesis pacer
        if np.any((A_times > pacer_time) & (A_times <= pacer_time + Delta)):
            return True
    return False

# Run simulations
violations = 0
for _ in range(num_simulations):
    A_times, H_times = simulate_mining(a, h, total_time)
    pacers = find_pacers(H_times, Delta)
    if safety_violation(A_times, H_times, pacers, k):
        violations += 1

# Calculate probability of safety violation
prob_safety_violation = violations / num_simulations
print(prob_safety_violation)
