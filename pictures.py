import pandas as pd
import matplotlib.pyplot as plt

earth = pd.read_csv('earth.csv')
sun = pd.read_csv('sun.csv')

earth_p = [earth['t'], earth['x'], earth['y'], earth['z']]
sun_p = [sun['t'], sun['x'], sun['y'], sun['z']]

ax = plt.figure().add_subplot(projection="3d")

ax.plot(earth_p[1], earth_p[2], earth_p[0], label='earth orbit')
ax.plot(sun_p[1], sun_p[2], sun_p[0], label='sun orbit')

ax.axes.set_xlabel("x-distance (m)")
ax.axes.set_ylabel("y-distance (m)")
ax.axes.set_zlabel("time (s)")
#ax.axis('equal')

ax.legend()
plt.show()