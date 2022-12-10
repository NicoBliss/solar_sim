import pandas as pd
import matplotlib.pyplot as plt

def plot(num, name):
    planet = pd.read_csv('data/'+str(num)+".csv")
    ax.plot(planet['x'], planet['y'], planet['t'], label=name)

ax = plt.figure().add_subplot(projection="3d")

plot(2, "moon")

ax.axes.set_xlabel("x-distance (m)")
ax.axes.set_ylabel("y-distance (m)")
ax.axes.set_zlabel("time (s)")
#ax.axis('equal')

ax.legend()
plt.show()