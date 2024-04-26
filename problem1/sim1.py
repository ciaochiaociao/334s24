import pandas as pd
block_data = pd.read_csv('block_list_2024-03-19_2024-04-19.csv')
# Convert 'Time (UTC)' column to datetime
block_data['Time (UTC)'] = pd.to_datetime(block_data['Time (UTC)'])

# Calculate the time differences between consecutive blocks (inter-block mining times)
block_data['Inter-block Time (Minutes)'] = block_data['Time (UTC)'].diff(-1).dt.total_seconds() / 60
block_data['Inter-block Time (Minutes)'] = block_data['Inter-block Time (Minutes)'].abs()  # Absolute values for time differences

# Calculate the standard deviation of the inter-block times
std_dev_inter_block_time = block_data['Inter-block Time (Minutes)'].std()

print(std_dev_inter_block_time)
