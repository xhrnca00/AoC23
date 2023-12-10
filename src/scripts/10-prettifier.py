FILE = "./data/10.in"
NEW = "./data/10.pretty.txt"


def main():
    with open(FILE) as f:
        data = f.read()
    data = (
        data.replace("-", "─")
        .replace("|", "│")
        .replace("7", "┐")
        .replace("F", "┌")
        .replace("L", "└")
        .replace("J", "┘")
    )
    with open(NEW, "w", encoding="utf-8") as f:
        f.write(data)


if __name__ == "__main__":
    main()
