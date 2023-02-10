# %%
import numpy as np
import json
import matplotlib.pyplot as plt
import seaborn as sns
sns.set()

# %%
with open("../dss-examples/temp_results.json", 'r') as file:
    results = json.load(file)

fig, ax = plt.subplots()
ax.plot(
    results['history']['time'],
    results['m1']['history']['temp'][1:],
    label='m1',
)
ax.plot(
    results['history']['time'],
    results['m2']['history']['temp'][1:],
    label='m2',
)
ax.plot(
    results['history']['time'],
    results['m3']['history']['temp'][1:],
    label='m3',
)
ax.set_ylabel('Temperature [°C]')
ax.set_xlabel('Time [s]')
ax.legend()
# %%
