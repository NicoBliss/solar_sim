import pandas as pd
import matplotlib.pyplot as plt

def plot(num, name):
    planet = pd.read_csv('data/'+str(num)+".csv")
    ax.plot(planet['x'], planet['y'], planet['t'], label=name)


def plot_diff(num1, num2, name1, name2):
    planet1 = pd.read_csv('data/'+str(num1)+".csv")
    planet2 = pd.read_csv('data/'+str(num2)+".csv")
    for i in range(planet1['t'].shape[0]):
        planet2['x'][i] -= planet1['x'][i]
        planet2['y'][i] -= planet1['y'][i]
        planet2['z'][i] -= planet1['z'][i]
        planet1.iat[i,1] = 0
        planet1.iat[i,2] = 0
        planet1.iat[i,3] = 0

    ax.plot(planet1['x'], planet1['y'], planet1['t'], label=name1)
    ax.plot(planet2['x'], planet2['y'], planet2['t'], label=name2)

ax = plt.figure().add_subplot(projection="3d")

plot_diff(1, 2, "earth", "moon")

ax.axes.set_xlabel("x-distance (m)")
ax.axes.set_ylabel("y-distance (m)")
ax.axes.set_zlabel("time (s)")
#ax.axis('equal')

ax.legend()
plt.show()