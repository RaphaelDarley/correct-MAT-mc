using Plots

num_correct = collect(1:10)
correct_count = [235160, 331092, 240561, 98816, 23907, 3305, 262, 12, 0, 0]



bar(num_correct, correct_count)