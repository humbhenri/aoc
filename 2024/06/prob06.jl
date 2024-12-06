function parseinput(filename)
    input = open(filename, "r")
    content = readchomp(input)
    close(input)
    map(line -> split(line, ""), split(content, "\n"))
end

function getGuard(area)
    for (i, line) in pairs(IndexStyle(area), area)
        for (j, val) in pairs(IndexStyle(line), line)
            if (val in ["^", ">", "<", "v"])
                return (i, j)
            end
        end
    end
end

function walk(area, (x, y)::Tuple{Int, Int})
    dirs = ["^", ">", "v", "<"]
    next = [(-1, 0), (0, 1), (1, 0), (0, -1)]
    dir = area[x][y]
    index = findfirst(==(dir), dirs)
    dx, dy = next[index]
    X, Y = x + dx, y + dy
    if (area[X][Y] == "#")
        nextdir = index + 1 <= 4 ? index + 1 : 1
        area[x][y] = dirs[nextdir]
        return x, y
    end
    area[X][Y] = dir
    area[x][y] = "X"
    X, Y
end

function count(area)
    mapreduce(x -> x == "X", +, Iterators.flatten(area)) + 1
end

function part1(area)
    (x, y) = getGuard(area)
    while (true)
        try
            (x, y) = walk(area, (x, y))
            str = join(area, "\n")
        catch e
            if isa(e, BoundsError)
                distinct = count(area)
                println("distinct positions: $distinct")
                exit(0)
            else
                println(e)
                exit(1)
            end
        end
    end
end

area = parseinput("06.input")
part1(area)
