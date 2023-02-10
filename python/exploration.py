# %%
import numpy as np
import json
import matplotlib.pyplot as plt
import seaborn as sns
sns.set()

# %%

# to generate this file, run `cargo run` in dss-examples/
with open("../dss-examples/temp_results.json", 'r') as file:
    results = json.load(file)

# %%

fig, ax = plt.subplots()
ax.plot(
    results['history']['time'],
    results['m1']['history']['temp'],
    label='m1',
)
ax.plot(
    results['history']['time'],
    results['m2']['history']['temp'],
    label='m2',
)
ax.plot(
    results['history']['time'],
    results['m3']['history']['temp'],
    label='m3',
)
ax.set_ylabel('Temperature [°C]')
ax.set_xlabel('Time [s]')
ax.legend()
# %%

# conservation of energy based on constant thermal capacity

fig, ax = plt.subplots()
ax.plot(
    results['history']['time'],
    np.array(results['m1']['history']['temp']) * results['m1']['c'],
    label='m1',
)
ax.plot(
    results['history']['time'],
    (
        np.array(results['m1']['history']['temp']) * results['m1']['c'] +
        np.array(results['m2']['history']['temp']) * results['m2']['c']
    ),
    label='m1 + m2',
)
ax.plot(
    results['history']['time'],
    (
        np.array(results['m1']['history']['temp']) * results['m1']['c'] +
        np.array(results['m2']['history']['temp']) * results['m2']['c'] +
        np.array(results['m3']['history']['temp']) * results['m3']['c']
    ),
    label='m1 + m2 + m3',
)
ax.set_ylabel('Energy [J]')
ax.set_xlabel('Time [s]')
ax.legend()

