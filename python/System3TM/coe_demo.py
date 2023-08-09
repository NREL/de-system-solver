# %%
import dess_pyo3
import numpy as np
import matplotlib.pyplot as plt # type: ignore
import seaborn as sns # type: ignore
import time
sns.set()


# %%
# # Check for reasonable behavior and conservation of energy

m1 = dess_pyo3.ThermalMass(1.0, 2.0)
m2 = dess_pyo3.ThermalMass(2.0, 10.0)
h12 = dess_pyo3.Conductance(5.0)
m3 = dess_pyo3.ThermalMass(1.5, 12.0)
h23 = dess_pyo3.Conductance(5.0)
t_report_euler = np.linspace(0.0, 1.0, 201).tolist()
t_report_rk4 = np.linspace(0.0, 1.0, 11).tolist()


sys_rk4 = dess_pyo3.System3TM(
    '{"RK4Fixed":{"dt":1.0}}',
    m1,
    m2,
    h12,
    m3,
    h23,
    t_report_rk4,)
t0 = time.perf_counter()
sys_rk4.walk()
t1 = time.perf_counter()
print(f"Elapsed time to run `sys_rk4.walk()`: {t1-t0:.3g}")

sys_euler = dess_pyo3.System3TM(
    '{"EulerFixed": {"dt": 1.0}}',
    m1,
    m2,
    h12,
    m3,
    h23,
    t_report_euler,)

t0 = time.perf_counter()
sys_euler.walk()
t1 = time.perf_counter()
print(f"Elapsed time to run `sys_euler.walk()`: {t1-t0:.3g}")


# %% plot to check for reasonable behavior and energy conservation

# Get the default color cycle
default_colors = plt.rcParams['axes.prop_cycle'].by_key()['color']
euler_markersize = 1
rk4_markersize = 2
rk4_marker = 'o'

fig, ax = plt.subplots()
# m1
# euler
ax.plot(
    sys_euler.history.time,
    sys_euler.m1.history.temp,
    label='m1',
    color=default_colors[0],
    markersize=euler_markersize,
    linestyle='',
    marker=rk4_marker,
)
# rk4
ax.plot(
    sys_rk4.history.time,
    sys_rk4.m1.history.temp,
    label='m1 rk4',
    color=default_colors[0],
    markersize=rk4_markersize,
    linestyle='',
    marker='s'
)
# m2
# euler
ax.plot(
    sys_euler.history.time,
    sys_euler.m2.history.temp,
    label='m2',
    color=default_colors[1],
    markersize=euler_markersize,
    linestyle='',
    marker=rk4_marker,
)
# rk4
ax.plot(
    sys_rk4.history.time,
    sys_rk4.m2.history.temp,
    label='m2 rk4',
    color=default_colors[1],
    markersize=rk4_markersize,
    linestyle='',
    marker='s'
)
# m3
# euler
ax.plot(
    sys_euler.history.time,
    sys_euler.m3.history.temp,
    label='m3',
    color=default_colors[2],
    markersize=euler_markersize,
    linestyle='',
    marker=rk4_marker,
)
# rk4
ax.plot(
    sys_rk4.history.time,
    sys_rk4.m3.history.temp,
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
    sys_euler.history.time,
    np.array(sys_euler.m1.history.temp) * sys_euler.m1.c,
    label='m1',
    color=default_colors[0],
    markersize=euler_markersize,
    linestyle='',
    marker=rk4_marker,
)
# rk4
ax.plot(
    sys_rk4.history.time,
    np.array(sys_rk4.m1.history.temp) * sys_rk4.m1.c,
    label='m1 rk4',
    color=default_colors[0],
    markersize=rk4_markersize,
    linestyle='',
    marker='s'
)
# m1 + m2
# euler
ax.plot(
    sys_euler.history.time,
    (
        np.array(sys_euler.m1.history.temp) * sys_euler.m1.c +
        np.array(sys_euler.m2.history.temp) * sys_euler.m2.c
    ),
    label='m1 + m2',
    color=default_colors[1],
    markersize=euler_markersize,
    linestyle='',
    marker=rk4_marker,
)
# rk4
ax.plot(
    sys_rk4.history.time,
    (
        np.array(sys_rk4.m1.history.temp) * sys_rk4.m1.c +
        np.array(sys_rk4.m2.history.temp) * sys_rk4.m2.c
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
    sys_euler.history.time,
    (
        np.array(sys_euler.m1.history.temp) * sys_euler.m1.c +
        np.array(sys_euler.m2.history.temp) * sys_euler.m2.c +
        np.array(sys_euler.m3.history.temp) * sys_euler.m3.c
    ),
    label='m1 + m2 + m3',
    color=default_colors[2],
    markersize=euler_markersize,
    linestyle='',
    marker=rk4_marker,
)
# euler
ax.plot(
    sys_rk4.history.time,
    (
        np.array(sys_rk4.m1.history.temp) * sys_rk4.m1.c +
        np.array(sys_rk4.m2.history.temp) * sys_rk4.m2.c +
        np.array(sys_rk4.m3.history.temp) * sys_rk4.m3.c
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
