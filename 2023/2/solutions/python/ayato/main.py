import sys


def part_1(data):
    ans = 0
    color_limits = {"red": 12, "blue": 14, "green": 13}

    for line in data:
        game, parts = line.split(": ")
        _id = int(game.split(" ")[1])

        conditions = [
            all(
                int(num) <= color_limits[color]
                for num, color in [cubes.split(" ") for cubes in part.split(", ")]
            )
            for part in parts.split("; ")
        ]

        good = all(conditions)

        if good:
            ans += _id
    return ans


def part_2(data):
    ans = 0

    for line in data:
        game, parts = line.split(": ")

        r, g, b = 0, 0, 0

        for part in parts.split("; "):
            for cubes in part.split(", "):
                num, color = cubes.split(" ")
                num = int(num)

                if color == "red":
                    r = max(r, num)
                elif color == "blue":
                    b = max(b, num)
                elif color == "green":
                    g = max(g, num)
        ans += r * g * b
    return ans


if __name__ == "__main__":
    input_data = sys.stdin.read().strip().split("\n")

    result_part_1 = part_1(input_data)
    result_part_2 = part_2(input_data)

    print(result_part_1)
    print(result_part_2)
