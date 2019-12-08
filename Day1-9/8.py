def render(layers, n, m):
    img = [[" " for _ in range(n)] for _ in range(m)]
    for l_idx in range(len(layers) - 1, -1, -1):
        for x in range(n):
            for y in range(m):
                px = layers[l_idx][(y * n) + x]
                if px == '0':
                    img[y][x] = " "
                elif px == '1':
                    img[y][x] = 'X'
    return "\n".join("".join(row) for row in img)


if __name__ == "__main__":
    with open("8.txt") as f:
        data = f.read()
    X, Y = 25, 6
    layer_size = X * Y
    layers = [
        data[i * layer_size: (i + 1) * layer_size]
        for i in range(len(data) // layer_size)
    ]
    zero_counts = [(l.count("0"), idx) for idx, l in enumerate(layers)]
    min_layer = min(zero_counts)[1]
    print(
        "Part 1: {}".format(layers[min_layer].count(
            "1") * layers[min_layer].count("2"))
    )

    print("Part 2:")
    print(render(layers, X, Y))
