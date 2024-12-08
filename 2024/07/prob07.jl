function parseinput(filename)
    input = open(filename, "r")
    content = readchomp(input)
    close(input)
    map(line -> begin
            target, values = split(line, ":")
            numbers = map(s -> parse(Int, s), split(strip(values), " "))
            [parse(Int, target), numbers]
        end, split(content, "\n"))
end

function combinations(numbers)
    n = length(numbers)
    if n == 1
        return numbers
    end
    first = numbers[1]
    equations = combinations(numbers[2:end])
    newEquations = []
    for equation in equations
        insert!(newEquations, 1, [first; "+"; equation])
        insert!(newEquations, 1, [first; "*"; equation])
    end
    newEquations
end
            
function eval(equation)
    ans=0
    op=""
    for el in equation
        if el isa String
            op = el
        else
            if op == ""
                ans = el
            else
                if op == "+"
                    ans = ans + el
                else
                    ans = ans * el
                end
            end
        end
    end
    ans
end

function part1(input)
    sum = 0
    for line in input
        target = line[1]
        equations = combinations(line[2])
        for equation in equations
            if eval(equation) == target
                sum += target
                break
            end
        end
    end
    println("sum = $sum")
end

input = parseinput("07.input")
part1(input)
