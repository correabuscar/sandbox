#!/usr/bin/env python3

#untested, requirements require too much compilation at this time and I'm unwilling to do, in Gentoo, heck even gcc needs to be recompiled (for fortran USE flag!)

import matplotlib.pyplot as plt
import numpy as np

# Define function to draw a DNA strand
def draw_dna(ax, x_start, y_start, length, compact):
    for i in range(length):
        angle = np.pi * i / 10
        radius = 0.5 if compact else 1.5
        x = x_start + i * 0.5
        y = y_start + radius * np.sin(angle)
        ax.plot([x, x], [y, y + 0.2], color='blue')
        ax.plot([x, x], [y, y - 0.2], color='red')

# Create figure and axis
fig, ax = plt.subplots(1, 2, figsize=(14, 6))

# Draw slow rotation (extended DNA)
ax[0].set_title('Extended DNA - Slow Rotation')
draw_dna(ax[0], x_start=0, y_start=0, length=20, compact=False)
ax[0].set_xlim(-1, 11)
ax[0].set_ylim(-2, 2)
ax[0].axis('off')

# Draw fast rotation (compact DNA)
ax[1].set_title('Compact DNA - Fast Rotation')
draw_dna(ax[1], x_start=0, y_start=0, length=20, compact=True)
ax[1].set_xlim(-1, 11)
ax[1].set_ylim(-2, 2)
ax[1].axis('off')

# Display the plot
plt.tight_layout()
plt.show()

