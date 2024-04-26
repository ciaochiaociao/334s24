using Distributions
using Gadfly
using Cairo
using Fontconfig
using Colors
# Generate 100 with rate lambda = 1/4 (theta = 4) as inter-arrival times for at least 25 samples (n > 25) approximated by 100 time units multiplied by rate (1/4 samples per time unit)
sample = rand(Exponential(4), 35)


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

# Generate 100 with rate lambda = 3/4 (theta = 4/3) as inter-arrival times for at least 75 samples (n > 75) approximated by 100 time units multiplied by rate (3/4 samples per time unit)
sample2 = rand(Exponential(4/3), 90)
size2 = length(sample2)
arrival2 = zeros(size2)
arrival2[1] = sample2[1]
for i in 2:size2
    t2 = sample2[i] + arrival2[i-1]
    println(t2)
    if t2 > 100  # Stop when the arrival time exceeds 100
        println("break")
        break
    else
        arrival2[i] = t2
    end
end


# Plot the arrival times as red dots
yaxis = zeros(length(arrival))
yaxis2 = zeros(length(arrival2))
p = Gadfly.plot(layer(x=arrival, y=yaxis, color=[RGBA(1,0,0,0.1)], Geom.point, Theme(point_size=1pt)),
    layer(x=arrival2, y=yaxis2, color=[RGBA(0,0,1,0.1)], Geom.point, Theme(point_size=1pt)),
    Guide.xlabel("Arrival Time"),
    Guide.manual_color_key("Process", ["with rate 1/4", "with rate 3/4"], ["red", "blue"])
    )
display(p)

# # Save the plot to a file
draw(PDF("arrival_times2.pdf", 6inch, 4inch), p)