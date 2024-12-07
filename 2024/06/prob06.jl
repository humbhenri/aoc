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

function count(area)
    mapreduce(x -> x == "X", +, Iterators.flatten(area)) + 1
end

function part1(area)
    x, y = getGuard(area)
    while (true)
        dirs = ["^", ">", "v", "<"]
        next = [(-1, 0), (0, 1), (1, 0), (0, -1)]
        dir = area[x][y]
        index = findfirst(==(dir), dirs)
        dx, dy = next[index]
        X, Y = x + dx, y + dy
        if (X == 0 || Y == 0 || X == length(area)+1 || Y == length(area[1])+1)
            distinct = count(area)
            println("distinct positions: $distinct")
            break
        elseif (area[X][Y] == "#")
            nextdir = index + 1 <= 4 ? index + 1 : 1
            area[x][y] = dirs[nextdir]
        else
            area[X][Y] = dir
            area[x][y] = "X"
            x, y = X, Y
        end
    end
end

area = parseinput("06.input")
part1(area)
