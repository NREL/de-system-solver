# %%
import dess_pyo3
import numpy as np
import matplotlib.pyplot as plt # type: ignore
import seaborn as sns # type: ignore
import time
sns.set()

# %%
# # Check that `dt` and `t_report` work as expected

m1 = dess_pyo3.ThermalMass(1.0, 10.5)
m2 = dess_pyo3.ThermalMass(2.0, 10.0)
h12 = dess_pyo3.Conductance(5.0)
m3 = dess_pyo3.ThermalMass(1.5, 12.0)
h23 = dess_pyo3.Conductance(5.0)

t_report = np.linspace(0.0, 1.0, 4).tolist()
dt_small = (t_report[1] - t_report[0]) / 1e2
dt_medium = (t_report[1] - t_report[0]) / 1e1
dt_large = (t_report[1] - t_report[0]) * 1.0

sys_small_dt = dess_pyo3.System3TM(
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

sys_medium_dt = dess_pyo3.System3TM(
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

sys_large_dt = dess_pyo3.System3TM(
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

sys_heuns_small_dt = dess_pyo3.System3TM(
    f'{{"HeunsMethod": {{"dt": {dt_small}}}}}',
    m1,
    m2,
    h12,
    m3,
    h23,
    t_report,
)
t0 = time.perf_counter()
sys_heuns_small_dt.walk()
print(f"dt={dt_small:.3g} s elapsed: {time.perf_counter() - t0:.3g} s")

sys_heuns_medium_dt = dess_pyo3.System3TM(
    f'{{"HeunsMethod": {{"dt": {dt_medium}}}}}',
    m1,
    m2,
    h12,
    m3,
    h23,
    t_report,
)
t0 = time.perf_counter()
sys_heuns_medium_dt.walk()
print(f"dt={dt_medium:.3g} s elapsed: {time.perf_counter() - t0:.3g} s")

sys_heuns_large_dt = dess_pyo3.System3TM(
    f'{{"HeunsMethod": {{"dt": {dt_large}}}}}',
    m1,
    m2,
    h12,
    m3,
    h23,
    t_report,
)
t0 = time.perf_counter()
sys_heuns_large_dt.walk()
print(f"dt={dt_large:.3g} s elapsed: {time.perf_counter() - t0:.3g} s")

sys_midpoint_small_dt = dess_pyo3.System3TM(
    f'{{"MidpointMethod": {{"dt": {dt_small}}}}}',
    m1,
    m2,
    h12,
    m3,
    h23,
    t_report,
)
t0 = time.perf_counter()
sys_midpoint_small_dt.walk()
print(f"dt={dt_small:.3g} s elapsed: {time.perf_counter() - t0:.3g} s")

sys_midpoint_medium_dt = dess_pyo3.System3TM(
    f'{{"MidpointMethod": {{"dt": {dt_medium}}}}}',
    m1,
    m2,
    h12,
    m3,
    h23,
    t_report,
)
t0 = time.perf_counter()
sys_midpoint_medium_dt.walk()
print(f"dt={dt_medium:.3g} s elapsed: {time.perf_counter() - t0:.3g} s")

sys_midpoint_large_dt = dess_pyo3.System3TM(
    f'{{"MidpointMethod": {{"dt": {dt_large}}}}}',
    m1,
    m2,
    h12,
    m3,
    h23,
    t_report,
)
t0 = time.perf_counter()
sys_midpoint_large_dt.walk()
print(f"dt={dt_large:.3g} s elapsed: {time.perf_counter() - t0:.3g} s")

sys_ralstons_small_dt = dess_pyo3.System3TM(
    f'{{"RalstonsMethod": {{"dt": {dt_small}}}}}',
    m1,
    m2,
    h12,
    m3,
    h23,
    t_report,
)
t0 = time.perf_counter()
sys_ralstons_small_dt.walk()
print(f"dt={dt_small:.3g} s elapsed: {time.perf_counter() - t0:.3g} s")

sys_ralstons_medium_dt = dess_pyo3.System3TM(
    f'{{"RalstonsMethod": {{"dt": {dt_medium}}}}}',
    m1,
    m2,
    h12,
    m3,
    h23,
    t_report,
)
t0 = time.perf_counter()
sys_ralstons_medium_dt.walk()
print(f"dt={dt_medium:.3g} s elapsed: {time.perf_counter() - t0:.3g} s")

sys_ralstons_large_dt = dess_pyo3.System3TM(
    f'{{"RalstonsMethod": {{"dt": {dt_large}}}}}',
    m1,
    m2,
    h12,
    m3,
    h23,
    t_report,
)
t0 = time.perf_counter()
sys_ralstons_large_dt.walk()
print(f"dt={dt_large:.3g} s elapsed: {time.perf_counter() - t0:.3g} s")

sys_rk4_small_dt = dess_pyo3.System3TM(
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

sys_rk4_medium_dt = dess_pyo3.System3TM(
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

sys_rk4_large_dt = dess_pyo3.System3TM(
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

dt_max = 10.
dt_init = 0.1
rtol = 1e-5
atol = 1e-9
max_iter = 5
solver = dess_pyo3.AdaptiveSolverConfig(
    dt_max=dt_max,
    dt_init=dt_init,
    rtol=rtol,
    atol=atol,
    max_iter=max_iter,
)

sys_rk45 = dess_pyo3.System3TM.new_rk45_cash_karp(
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

markersize = 3
default_colors = plt.rcParams['axes.prop_cycle'].by_key()['color']
#plot with three subplots, one for each dt
fig, ax = plt.subplots(3, 1, sharex=True)
ax[0].plot(
    sys_small_dt.history.time,
    np.array(sys_small_dt.m1.history.temp),
    label='euler',
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
ax[2].set_title(f'dt = {dt_large:.3g}')
ax[0].plot(
    sys_small_dt.history.time,
    np.array(sys_rk4_small_dt.m1.history.temp),
    label='rk4',
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

ax[0].plot(
    sys_small_dt.history.time,
    np.array(sys_heuns_small_dt.m1.history.temp),
    label='heuns',
    color=default_colors[3],
    markersize=markersize,
    linestyle='',
    marker='v',
)
ax[1].plot(
    sys_small_dt.history.time,
    np.array(sys_heuns_medium_dt.m1.history.temp),
    color=default_colors[3],
    markersize=markersize,
    marker='v',
    linestyle='',
)
ax[2].plot(
    sys_small_dt.history.time,
    np.array(sys_heuns_large_dt.m1.history.temp),
    color=default_colors[3],
    markersize=markersize,
    marker='v',
    linestyle='',
)

ax[0].plot(
    sys_small_dt.history.time,
    np.array(sys_midpoint_small_dt.m1.history.temp),
    label='midpoint',
    color=default_colors[4],
    markersize=markersize,
    linestyle='',
    marker='^',
)
ax[1].plot(
    sys_small_dt.history.time,
    np.array(sys_midpoint_medium_dt.m1.history.temp),
    color=default_colors[4],
    markersize=markersize,
    marker='^',
    linestyle='',
)
ax[2].plot(
    sys_small_dt.history.time,
    np.array(sys_midpoint_large_dt.m1.history.temp),
    color=default_colors[4],
    markersize=markersize,
    marker='^',
    linestyle='',
)

ax[0].plot(
    sys_small_dt.history.time,
    np.array(sys_ralstons_small_dt.m1.history.temp),
    label='ralstons',
    color=default_colors[5],
    markersize=markersize,
    linestyle='',
    marker='<',
)
ax[1].plot(
    sys_small_dt.history.time,
    np.array(sys_ralstons_medium_dt.m1.history.temp),
    color=default_colors[5],
    markersize=markersize,
    marker='<',
    linestyle='',
)
ax[2].plot(
    sys_small_dt.history.time,
    np.array(sys_ralstons_large_dt.m1.history.temp),
    color=default_colors[5],
    markersize=markersize,
    marker='<',
    linestyle='',
)

ax[0].plot(
    sys_rk45.history.time,
    np.array(sys_rk45.m1.history.temp),
    label='rk45',
    color=default_colors[0],
    markersize=markersize,
    linestyle='',
    marker='x',
)


ax[0].set_ylabel('Temp. [°C]')
ax[-1].set_xlabel('Time [s]')
ax[0].legend()

# %%

solver = dess_pyo3.AdaptiveSolverConfig(
    dt_max=dt_max,
    dt_init=dt_init,
    rtol=rtol,
    atol=atol,
    max_iter=max_iter,
    save=True
)

sys_rk45 = dess_pyo3.System3TM.new_rk45_cash_karp(
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
print(f"rk45 dt elapsed: {time.perf_counter() - t0:.3g} s")
print(f"rk45 rtol={sys_rk45.solver_conf.rtol}")
print(f"rk45 dt_init={dt_init}")
print("rk45 dt_mean", np.array(sys_rk45.solver_conf.history.dt).mean())

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
    color='k'
)
ax[1].set_ylabel('Norm Error')
ax[1].legend()

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