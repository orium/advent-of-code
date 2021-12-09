#!/usr/bin/lua

local grid={}
local i = 1
local columns

while true do
    s = io.read("*l")
    if not s then
        break
    end

    local line = {}

    columns = string.len(s)
    for j = 1, columns do
        local ch = string.sub(s, j, j)
        table.insert(line, tonumber(ch))
    end

    grid[i] = line

    i = i + 1
end

function g(i, j)
    return (grid[i] or {})[j] or 10
end

function is_low(i, j)
    local v = grid[i][j]
    return g(i-1, j) > v and g(i+1, j) > v and g(i, j-1) > v and g(i, j+1) > v
end

local risk = 0

for i = 1, #grid do
    for j = 1, columns do
        if is_low(i, j) then
            risk = risk + 1 + g(i, j)
        end
    end
end

print(risk)