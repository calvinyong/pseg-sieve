import sys
from io import StringIO

import matplotlib.pyplot as plt
import pandas as pd

if __name__ == "__main__":
    assert len(sys.argv) > 1

    df = pd.read_csv(StringIO("\n".join(sys.argv[1:])))
    print(df.to_markdown())

    fig, ax = plt.subplots()
    ax.set_title("Segmented Sieve: time vs segments (Limit: 1e9)")
    ax.set_xlabel("Number of segments")
    ax.set_ylabel("Time (s)")
    for alg in df.alg.unique():
        df_alg = df[df.alg == alg]
        ax.plot(df_alg.segments, df_alg.time, marker="o", label=alg)

    ax.legend()
    fig.savefig("imgs/seg-sieve.png")
