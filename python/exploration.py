# %%
import numpy as np
import json
import matplotlib.pyplot as plt
import seaborn as sns
sns.set()

# %%

# to generate this file, run `cargo run` in dess-examples/
with open("../target/results/euler dt=0.005 s.json", 'r') as file:
    res_euler = json.load(file)

# to generate this file, run `cargo run` in dess-examples/
with open("../target/results/rk4 dt=0.02 s.json", 'r') as file:
    res_rk4 = json.load(file)

# %%

# Get the default color cycle
default_colors = plt.rcParams['axes.prop_cycle'].by_key()['color']
euler_markersize = 1
rk4_markersize = 2
rk4_marker = 'o'

fig, ax = plt.subplots()
# m1
# euler
ax.plot(
    res_euler['history']['time'],
    res_euler['m1']['history']['temp'],
    label='m1',
    color=default_colors[0],
    markersize=euler_markersize,
    linestyle='',
    marker=rk4_marker,
)
# rk4
ax.plot(
    res_rk4['history']['time'],
    res_rk4['m1']['history']['temp'],
    label='m1 rk4',
    color=default_colors[0],
    markersize=rk4_markersize,
    linestyle='',
    marker='s'
)
# m2
# euler
ax.plot(
    res_euler['history']['time'],
    res_euler['m2']['history']['temp'],
    label='m2',
    color=default_colors[1],
    markersize=euler_markersize,
    linestyle='',
    marker=rk4_marker,
)
# rk4
ax.plot(
    res_rk4['history']['time'],
    res_rk4['m2']['history']['temp'],
    label='m2 rk4',
    color=default_colors[1],
    markersize=rk4_markersize,
    linestyle='',
    marker='s'
)
# m3
# euler
ax.plot(
    res_euler['history']['time'],
    res_euler['m3']['history']['temp'],
    label='m3',
    color=default_colors[2],
    markersize=euler_markersize,
    linestyle='',
    marker=rk4_marker,
)
# rk4
ax.plot(
    res_rk4['history']['time'],
    res_rk4['m3']['history']['temp'],
    label='m3 rk4',
    color=default_colors[2],
    markersize=rk4_markersize,
    linestyle='',
    marker='s'
)
ax.set_ylabel('Temperature [°C]')
ax.set_xlabel('Time [s]')
ax.legend()
# %%

# conservation of energy based on constant thermal capacity

fig, ax = plt.subplots()
# m1
# euler
ax.plot(
    res_euler['history']['time'],
    np.array(res_euler['m1']['history']['temp']) * res_euler['m1']['c'],
    label='m1',
    color=default_colors[0],
    markersize=euler_markersize,
    linestyle='',
    marker=rk4_marker,
)
# rk4
ax.plot(
    res_rk4['history']['time'],
    np.array(res_rk4['m1']['history']['temp']) * res_rk4['m1']['c'],
    label='m1 rk4',
    color=default_colors[0],
    markersize=rk4_markersize,
    linestyle='',
    marker='s'
)
# m1 + m2
# euler
ax.plot(
    res_euler['history']['time'],
    (
        np.array(res_euler['m1']['history']['temp']) * res_euler['m1']['c'] +
        np.array(res_euler['m2']['history']['temp']) * res_euler['m2']['c']
    ),
    label='m1 + m2',
    color=default_colors[1],
    markersize=euler_markersize,
    linestyle='',
    marker=rk4_marker,
)
# rk4
ax.plot(
    res_rk4['history']['time'],
    (
        np.array(res_rk4['m1']['history']['temp']) * res_rk4['m1']['c'] +
        np.array(res_rk4['m2']['history']['temp']) * res_rk4['m2']['c']
    ),
    label='m1 + m2 rk4',
    color=default_colors[1],
    markersize=rk4_markersize,
    linestyle='',
    marker='s'
)
# m1 + m2 + m3
# euler
ax.plot(
    res_euler['history']['time'],
    (
        np.array(res_euler['m1']['history']['temp']) * res_euler['m1']['c'] +
        np.array(res_euler['m2']['history']['temp']) * res_euler['m2']['c'] +
        np.array(res_euler['m3']['history']['temp']) * res_euler['m3']['c']
    ),
    label='m1 + m2 + m3',
    color=default_colors[2],
    markersize=euler_markersize,
    linestyle='',
    marker=rk4_marker,
)
# euler
ax.plot(
    res_rk4['history']['time'],
    (
        np.array(res_rk4['m1']['history']['temp']) * res_rk4['m1']['c'] +
        np.array(res_rk4['m2']['history']['temp']) * res_rk4['m2']['c'] +
        np.array(res_rk4['m3']['history']['temp']) * res_rk4['m3']['c']
    ),
    label='m1 + m2 + m3 rk4',
    color=default_colors[2],
    markersize=rk4_markersize,
    linestyle='',
    marker='s'
)
ax.set_ylabel('Energy [J]')
ax.set_xlabel('Time [s]')
ax.legend()
