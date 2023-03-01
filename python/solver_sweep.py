# %%
import dess_pyo3
import numpy as np
import matplotlib.pyplot as plt
import seaborn as sns
import time
sns.set()

# %%
# # Check that `dt` and `t_report` work as expected

m1 = dess_pyo3.ThermalMass(1.0, 10.5)
m2 = dess_pyo3.ThermalMass(2.0, 10.0)
h12 = dess_pyo3.Conductance(5.0)
m3 = dess_pyo3.ThermalMass(1.5, 12.0)
h13 = dess_pyo3.Conductance(5.0)

t_report = np.linspace(0.0, 1.0, 11).tolist()
dt_small = (t_report[1] - t_report[0]) / 1e3
dt_medium = (t_report[1] - t_report[0]) / 1e1
dt_large = (t_report[1] - t_report[0]) * 1.0

sys_small_dt = dess_pyo3.System(
    f'{{"EulerFixed": {{"dt": {dt_small}}}}}',
    m1,
    m2,
    h12,
    m3,
    h13,
    t_report,
)
sys_small_dt.walk()

sys_medium_dt = dess_pyo3.System(
    f'{{"EulerFixed": {{"dt": {dt_medium}}}}}',
    m1,
    m2,
    h12,
    m3,
    h13,
    t_report,
)
sys_medium_dt.walk()

sys_large_dt = dess_pyo3.System(
    f'{{"EulerFixed": {{"dt": {dt_large}}}}}',
    m1,
    m2,
    h12,
    m3,
    h13,
    t_report,
)
sys_large_dt.walk()

sys_rk4_small_dt = dess_pyo3.System(
    f'{{"RK4Fixed": {{"dt": {dt_small}}}}}',
    m1,
    m2,
    h12,
    m3,
    h13,
    t_report,
)
sys_rk4_small_dt.walk()

sys_rk4_medium_dt = dess_pyo3.System(
    f'{{"RK4Fixed": {{"dt": {dt_medium}}}}}',
    m1,
    m2,
    h12,
    m3,
    h13,
    t_report,
)
sys_rk4_medium_dt.walk()

sys_rk4_large_dt = dess_pyo3.System(
    f'{{"RK4Fixed": {{"dt": {dt_large}}}}}',
    m1,
    m2,
    h12,
    m3,
    h13,
    t_report,
)
sys_rk4_large_dt.walk()

solver = dess_pyo3.AdaptiveSolver.default()

sys_rk45 = dess_pyo3.System.new_rk45_cash_karp(
    solver,
    m1,
    m2,
    h12,
    m3,
    h13,
    t_report,
)
sys_rk45.walk()

markersize = 3
default_colors = plt.rcParams['axes.prop_cycle'].by_key()['color']

fig, ax = plt.subplots(3, 1, sharex=True)
ax[0].plot(
    sys_small_dt.history.time,
    np.array(sys_small_dt.m1.history.temp),
    label=f'euler',
    color=default_colors[0],
    markersize=markersize,
    linestyle='',
    marker='s',
)
ax[0].set_title(f'dt = {dt_small:.3g}')
ax[1].plot(
    sys_small_dt.history.time,
    np.array(sys_medium_dt.m1.history.temp),
    color=default_colors[0],
    markersize=markersize,
    marker='s',
    linestyle='',
)
ax[1].set_title(f'dt = {dt_medium:.3g}')
ax[2].plot(
    sys_small_dt.history.time,
    np.array(sys_large_dt.m1.history.temp),
    color=default_colors[0],
    markersize=markersize,
    marker='s',
    linestyle='',
)
ax[2].set_title(f'dt = {dt_large}')
ax[0].plot(
    sys_small_dt.history.time,
    np.array(sys_rk4_small_dt.m1.history.temp),
    label=f'rk4',
    color=default_colors[1],
    markersize=markersize,
    linestyle='',
    marker='o',
)
ax[1].plot(
    sys_small_dt.history.time,
    np.array(sys_rk4_medium_dt.m1.history.temp),
    color=default_colors[1],
    markersize=markersize,
    marker='o',
    linestyle='',
)
ax[2].plot(
    sys_small_dt.history.time,
    np.array(sys_rk4_large_dt.m1.history.temp),
    color=default_colors[1],
    markersize=markersize,
    marker='o',
    linestyle='',
)

ax[0].set_ylabel('Temp. [°C]')
ax[-1].set_xlabel('Time [s]')
ax[0].legend()

# %%
