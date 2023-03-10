# %%
import dess_pyo3
import numpy as np
import matplotlib.pyplot as plt
import seaborn as sns
import time
sns.set()

# %%
# # Check that `dt` and `t_report` work as expected

m1 = dess_pyo3.ThermalReservoir(8.5)
m2 = dess_pyo3.ThermalMass(2.0, 10.0)
h12 = dess_pyo3.Conductance(5.0)
m3 = dess_pyo3.ThermalMass(1.5, 12.0)
h23 = dess_pyo3.Conductance(5.0)

t_report = np.linspace(0.0, 1.0, 11).tolist()
dt_small = (t_report[1] - t_report[0]) / 1e3
dt_medium = (t_report[1] - t_report[0]) / 1e1
dt_large = (t_report[1] - t_report[0]) * 1.0

sys_small_dt = dess_pyo3.System3TMWithBC(
    f'{{"EulerFixed": {{"dt": {dt_small}}}}}',
    m1,
    m2,
    h12,
    m3,
    h23,
    t_report,
)
t0 = time.perf_counter()
sys_small_dt.walk()
print(f"dt={dt_small:.3g} s elapsed: {time.perf_counter() - t0:.3g} s")

sys_medium_dt = dess_pyo3.System3TMWithBC(
    f'{{"EulerFixed": {{"dt": {dt_medium}}}}}',
    m1,
    m2,
    h12,
    m3,
    h23,
    t_report,
)
t0 = time.perf_counter()
sys_medium_dt.walk()
print(f"dt={dt_medium:.3g} s elapsed: {time.perf_counter() - t0:.3g} s")

sys_large_dt = dess_pyo3.System3TMWithBC(
    f'{{"EulerFixed": {{"dt": {dt_large}}}}}',
    m1,
    m2,
    h12,
    m3,
    h23,
    t_report,
)
t0 = time.perf_counter()
sys_large_dt.walk()
print(f"dt={dt_large:.3g} s elapsed: {time.perf_counter() - t0:.3g} s")

sys_rk4_small_dt = dess_pyo3.System3TMWithBC(
    f'{{"RK4Fixed": {{"dt": {dt_small}}}}}',
    m1,
    m2,
    h12,
    m3,
    h23,
    t_report,
)
t0 = time.perf_counter()
sys_rk4_small_dt.walk()
print(f"rk4 dt={dt_small:.3g} s elapsed: {time.perf_counter() - t0:.3g} s")

sys_rk4_medium_dt = dess_pyo3.System3TMWithBC(
    f'{{"RK4Fixed": {{"dt": {dt_medium}}}}}',
    m1,
    m2,
    h12,
    m3,
    h23,
    t_report,
)
t0 = time.perf_counter()
sys_rk4_medium_dt.walk()
print(f"rk4 dt={dt_medium:.3g} s elapsed: {time.perf_counter() - t0:.3g} s")

sys_rk4_large_dt = dess_pyo3.System3TMWithBC(
    f'{{"RK4Fixed": {{"dt": {dt_large}}}}}',
    m1,
    m2,
    h12,
    m3,
    h23,
    t_report,
)
t0 = time.perf_counter()
sys_rk4_large_dt.walk()
print(f"rk4 dt={dt_large:.3g} s elapsed: {time.perf_counter() - t0:.3g} s")

max_iter = 5
rtol = 1e-2
dt_init = 1e-3
solver = dess_pyo3.AdaptiveSolverConfig(
    dt_init=dt_init,
    max_iter=max_iter,
    rtol=rtol,
)

sys_rk45 = dess_pyo3.System3TMWithBC.new_rk45_cash_karp(
    solver,
    m1,
    m2,
    h12,
    m3,
    h23,
    t_report,
)
t0 = time.perf_counter()
sys_rk45.walk()
print(f"rk45 elapsed: {time.perf_counter() - t0:.3g} s")
print(f"rk45 rtol={sys_rk45.solver_conf.rtol}")
print(f"rk45 dt_init={dt_init}")


solver_save = dess_pyo3.AdaptiveSolverConfig(
    dt_init=dt_init,
    max_iter=max_iter,
    rtol=rtol,
    save=True,
)
sys_rk45_save = dess_pyo3.System3TMWithBC.new_rk45_cash_karp(
    solver_save,
    m1,
    m2,
    h12,
    m3,
    h23,
    t_report,
)
sys_rk45_save.walk()


markersize = 3
default_colors = plt.rcParams['axes.prop_cycle'].by_key()['color']

fig, ax = plt.subplots(3, 1, sharex=True)
ax[0].plot(
    sys_small_dt.history.time,
    np.array(sys_small_dt.m2.history.temp),
    label=f'euler',
    color=default_colors[0],
    markersize=markersize,
    linestyle='',
    marker='s',
)
ax[0].plot(
    sys_rk45_save.solver_conf.history.t_curr,
    np.array([
        states[0]
        for states in sys_rk45_save.solver_conf.history.states]),
    label=f'm1',
    color=default_colors[0],
    markersize=markersize,
)
ax[0].plot(
    sys_rk45_save.solver_conf.history.t_curr,
    np.array([
        states[1]
        for states in sys_rk45_save.solver_conf.history.states]),
    label=f'm2 rk45',
    color=default_colors[2],
    markersize=markersize,
)
ax[0].set_title(f'dt = {dt_small:.3g}')
ax[1].plot(
    sys_small_dt.history.time,
    np.array(sys_medium_dt.m2.history.temp),
    color=default_colors[0],
    markersize=markersize,
    marker='s',
    linestyle='',
)
ax[1].set_title(f'dt = {dt_medium:.3g}')
ax[2].plot(
    sys_small_dt.history.time,
    np.array(sys_large_dt.m2.history.temp),
    color=default_colors[0],
    markersize=markersize,
    marker='s',
    linestyle='',
)
ax[2].set_title(f'dt = {dt_large:.3g}')
ax[0].plot(
    sys_small_dt.history.time,
    np.array(sys_rk4_small_dt.m2.history.temp),
    label=f'rk4',
    color=default_colors[1],
    markersize=markersize,
    linestyle='',
    marker='o',
)
ax[1].plot(
    sys_small_dt.history.time,
    np.array(sys_rk4_medium_dt.m2.history.temp),
    color=default_colors[1],
    markersize=markersize,
    marker='o',
    linestyle='',
)
ax[2].plot(
    sys_small_dt.history.time,
    np.array(sys_rk4_large_dt.m2.history.temp),
    color=default_colors[1],
    markersize=markersize,
    marker='o',
    linestyle='',
)

ax[0].plot(
    sys_rk45.history.time,
    np.array(sys_rk45.m2.history.temp),
    label=f'rk45',
    color=default_colors[0],
    markersize=markersize + 2,
    linestyle='',
    marker='x',
)


ax[0].set_ylabel('Temp. [°C]')
ax[-1].set_xlabel('Time [s]')
ax[0].legend()

# %%

solver = dess_pyo3.AdaptiveSolverConfig(
    dt_init=dt_init,
    max_iter=max_iter,
    rtol=rtol,
    save=True,
)

sys_rk45 = dess_pyo3.System3TMWithBC.new_rk45_cash_karp(
    solver,
    m1,
    m2,
    h12,
    m3,
    h23,
    t_report,
)
t0 = time.perf_counter()
sys_rk45.walk()
print(f"rk45 elapsed: {time.perf_counter() - t0:.3g} s")
print(f"rk45 rtol={sys_rk45.solver_conf.rtol}")
print(f"rk45 dt_init={dt_init}")

fig, ax = plt.subplots(3, 1, sharex=True)
ax[0].plot(
    np.array(sys_rk45.solver_conf.history.t_curr),
    np.array(sys_rk45.solver_conf.history.n_iter),
    linestyle='',
    marker='x',
)
ax[0].set_ylabel('n_iter')

ax[1].plot(
    np.array(sys_rk45.solver_conf.history.t_curr),
    np.array(sys_rk45.solver_conf.history.norm_err),
    linestyle='',
    marker='x',
    label='absolute',
)
ax[1].plot(
    np.array(sys_rk45.solver_conf.history.t_curr),
    np.array(sys_rk45.solver_conf.history.norm_err_rel),
    linestyle='',
    marker='o',
    label='relative',
)
ax[1].plot(
    [sys_rk45.history.time[0], sys_rk45.history.time[-1]],
    [sys_rk45.solver_conf.rtol] * 2,
    label='rtol',
    color='k',
)
ax[1].set_ylabel('Norm Error')
ax[1].legend(loc='right')

ax[-1].plot(
    np.array(sys_rk45.solver_conf.history.t_curr),
    np.array(sys_rk45.solver_conf.history.dt),
    linestyle='',
    marker='x',
)
ax[-1].set_ylabel('dt')
ax[-1].set_xlabel('Sim. Time [s]')

# %%

fig, ax = plt.subplots()
ax.plot(
    sys_rk45.solver_conf.history.t_curr,
    np.array(sys_rk45.solver_conf.history.norm_err_rel) < sys_rk45.solver_conf.rtol
)
ax.set_xlabel("Time [s]")
ax.set_ylabel('rtol met')
