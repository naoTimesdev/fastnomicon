import fastnomicon

ts = fastnomicon.parse_timestring("1h30s")
merged_duration = [t.as_duration() for t in ts]

# Merge list[timedelta] into one timedelta
print(merged_duration)
