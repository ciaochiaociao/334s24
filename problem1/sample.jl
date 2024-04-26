# generating 5 exponential(1/2) as inter-arrival times

using Distributions
sample = rand(Exponential(0.5),5)
print(sample)
#[0.021907272304307474, 0.21703943412886162, 0.08307746218558654, 0.5667115195460581, 1.4584361639686763]

# generating arrival times vector (arrival) from previous inter-arrival times vector (intarrival)
intarrival = [0.021907272304307474, 0.21703943412886162, 0.08307746218558654, 0.5667115195460581, 1.4584361639686763]
size = length(intarrival)
arrival = zeros(size)
arrival[1] = intarrival[1]
for i in 2:size
    arrival[i] = intarrival[i]+arrival[i-1]
end
print(arrival)
# [0.021907272304307474, 0.2389467064331691, 0.3220241686187556, 0.8887356881648137, 2.34717185213349]

# plot some arrival times coming from two processes with different color dots
using Gadfly
arrival_1 = [0.021907272304307474, 0.3220241686187556]
arrival_2 = [0.2389467064331691, 0.8887356881648137, 2.34717185213349]
yaxis_1 = zeros(length(arrival_1))
yaxis_2 = zeros(length(arrival_2))
p=Gadfly.plot(layer(x=arrival_1,y = yaxis_1,color=[colorant"blue"],
Geom.point),layer(x=arrival_2,y = yaxis_2,color=[colorant"red"], Geom.point))
display(p)