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

visited::Dict{Tuple{Int,Int}, String} = Dict()

# return true if a loop is detected
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
            if (get(visited, (x, y), "") == dir)
                println("loop detected: $x $y $dir")
                return true
            end
            visited[(x, y)] = dir
            x, y = X, Y
        end
    end
    return false
end

function part2(filename)
    # for every visited place in part 1, add a block in the area, if a loop is detect increment the count
    count = 0
    originalvis = copy(visited)
    for (x, y) in keys(originalvis)
        area = parseinput(filename)
        area[x][y] = "#"
        loop = part1(area)
        if (loop)
            count += 1
        end
    end
    println("count = $count")
end


area = parseinput("example")
@showtime part1(area)
@showtime part2("example")
