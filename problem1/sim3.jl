using Distributions
using Gadfly
using Cairo
using Fontconfig
using Colors
# Generate 100 with rate lambda = 1 as inter-arrival times for at least 100 samples (n > 100) approximated by 100 time units multiplied by rate (1 samples per time unit)
sample = rand(Exponential(1), 120)

# Generate arrival times vector (arrival) from previous inter-arrival times vector (intarrival)
size = length(sample)
arrival = zeros(size)
arrival[1] = sample[1]
for i in 2:size
    t = sample[i] + arrival[i-1]
    println(t)
    if t > 100  # Stop when the arrival time exceeds 100
        println("break")
        break
    else
        arrival[i] = t
    end
end

# Generate random colors for each arrival
colors = [rand() < 0.25 ? RGBA(1,0,0,0.1) : RGBA(0,0,1,0.1) for i in 1:length(arrival)]

# Plot the arrival times with random colors
yaxis = zeros(length(arrival))
p = Gadfly.plot(layer(x=arrival, y=yaxis, color=colors, Geom.point, Theme(point_size=1pt)),
    Guide.xlabel("Arrival Time")
    )
display(p)