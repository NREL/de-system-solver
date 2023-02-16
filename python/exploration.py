# %%
import numpy as np
import json
import matplotlib.pyplot as plt
import seaborn as sns
sns.set()

# %%

# to generate this file, run `cargo run` in dss-examples/
with open("../target/results dt=0.01 s.json", 'r') as file:
    res_euler = json.load(file)

# to generate this file, run `cargo run` in dss-examples/
with open("../target/rk4 results dt=0.04 s.json", 'r') as file:
    res_rk4 = json.load(file)

# %%

# Get the default color cycle
default_colors = plt.rcParams['axes.prop_cycle'].by_key()['color']

fig, ax = plt.subplots()
ax.plot(
    res_euler['history']['time'],
    res_euler['m1']['history']['temp'],
    label='m1',
    color=default_colors[0],
)
ax.plot(
    res_rk4['history']['time'],
    res_rk4['m1']['history']['temp'],
    label='m1 rk4',
    color=default_colors[0],
    linestyle='--'
)
ax.plot(
    res_euler['history']['time'],
    res_euler['m2']['history']['temp'],
    label='m2',
    color=default_colors[1],
)
ax.plot(
    res_rk4['history']['time'],
    res_rk4['m2']['history']['temp'],
    label='m2 rk4',
    color=default_colors[1],
    linestyle='--'
)
ax.plot(
    res_euler['history']['time'],
    res_euler['m3']['history']['temp'],
    label='m3',
    color=default_colors[2],
)
ax.plot(
    res_rk4['history']['time'],
    res_rk4['m3']['history']['temp'],
    label='m3 rk4',
    color=default_colors[2],
    linestyle='--'
)
ax.set_ylabel('Temperature [°C]')
ax.set_xlabel('Time [s]')
ax.legend()
# %%

# conservation of energy based on constant thermal capacity

fig, ax = plt.subplots()
ax.plot(
    res_euler['history']['time'],
    np.array(res_euler['m1']['history']['temp']) * res_euler['m1']['c'],
    label='m1',
)
ax.plot(
    res_euler['history']['time'],
    (
        np.array(res_euler['m1']['history']['temp']) * res_euler['m1']['c'] +
        np.array(res_euler['m2']['history']['temp']) * res_euler['m2']['c']
    ),
    label='m1 + m2',
)
ax.plot(
    res_euler['history']['time'],
    (
        np.array(res_euler['m1']['history']['temp']) * res_euler['m1']['c'] +
        np.array(res_euler['m2']['history']['temp']) * res_euler['m2']['c'] +
        np.array(res_euler['m3']['history']['temp']) * res_euler['m3']['c']
    ),
    label='m1 + m2 + m3',
)
ax.set_ylabel('Energy [J]')
ax.set_xlabel('Time [s]')
ax.legend()


# %%
