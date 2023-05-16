# %%
import dess_pyo3
import numpy as np
import matplotlib.pyplot as plt # type: ignore
import seaborn as sns # type: ignore
sns.set()

# %%

# # Check that `dt` and `t_report` work as expected

m1 = dess_pyo3.ThermalMass(1.0, 2.0)
m2 = dess_pyo3.ThermalMass(2.0, 10.0)
h12 = dess_pyo3.Conductance(5.0)
m3 = dess_pyo3.ThermalMass(1.5, 12.0)
h23 = dess_pyo3.Conductance(5.0)

t_report_dt_sweep = np.linspace(0.0, 1.0, 21).tolist()
dt_small = (t_report_dt_sweep[1] - t_report_dt_sweep[0]) / 100.0
dt_medium = (t_report_dt_sweep[1] - t_report_dt_sweep[0]) * 0.9
dt_ultra_medium = (t_report_dt_sweep[1] - t_report_dt_sweep[0]) * 1.0
dt_large = (t_report_dt_sweep[1] - t_report_dt_sweep[0]) * 2.0


def trim_t_report(dt) -> np.ndarray:
    """
    Returns `t_report` with spacing appropriate for dt
    """
    return np.arange(t_report_dt_sweep[0], t_report_dt_sweep[-1] + dt, dt)


sys_small_dt = dess_pyo3.System3TM(
    f'{{"RK4Fixed": {{"dt": {dt_small}}}}}',
    m1,
    m2,
    h12,
    m3,
    h23,
    trim_t_report(dt_small),
)
sys_small_dt.walk()

sys_medium_dt = dess_pyo3.System3TM(
    f'{{"RK4Fixed": {{"dt": {dt_medium}}}}}',
    m1,
    m2,
    h12,
    m3,
    h23,
    trim_t_report(dt_medium),
)
sys_medium_dt.walk()

sys_ultra_medium_dt = dess_pyo3.System3TM(
    f'{{"RK4Fixed": {{"dt": {dt_ultra_medium}}}}}',
    m1,
    m2,
    h12,
    m3,
    h23,
    trim_t_report(dt_ultra_medium),
)
sys_ultra_medium_dt.walk()

sys_large_dt = dess_pyo3.System3TM(
    f'{{"RK4Fixed": {{"dt": {dt_large}}}}}',
    m1,
    m2,
    h12,
    m3,
    h23,
    trim_t_report(dt_large),
)
sys_large_dt.walk()

# # assert that any errors are likely due solely to floating point rounding
# assert ((np.array(sys_ultra_medium_dt.m1.history.temp) -
#         np.array(sys_large_dt.m1.history.temp)) < 1e-14).all()

markersize = 3
default_colors = plt.rcParams['axes.prop_cycle'].by_key()['color']

fig, ax = plt.subplots()
ax.plot(
    sys_small_dt.history.time,
    np.array(sys_small_dt.m1.history.temp),
    label=f'dt = {dt_small:.3g}',
    color=default_colors[0],
    markersize=markersize,
    linestyle='',
    marker='o',
)
ax.plot(
    sys_medium_dt.history.time,
    np.array(sys_medium_dt.m1.history.temp),
    label=f'dt = {dt_medium:.3g}',
    color=default_colors[1],
    markersize=markersize,
    marker='s',
    linestyle='',
)
ax.plot(
    sys_ultra_medium_dt.history.time,
    np.array(sys_ultra_medium_dt.m1.history.temp),
    label=f'dt = {dt_ultra_medium:.3g}',
    color=default_colors[2],
    markersize=markersize,
    marker='v',
    linestyle='',
)
ax.plot(
    sys_large_dt.history.time,
    np.array(sys_large_dt.m1.history.temp),
    label=f'dt = {dt_large}',
    color=default_colors[3],
    markersize=markersize,
    marker='d',
    linestyle='',
)

ax.set_ylabel('Temperature [°C]')
ax.set_xlabel('Time [s]')
ax.legend()
ax.text(0.2, 5.5, "RK4 is very robust!")

# %%
