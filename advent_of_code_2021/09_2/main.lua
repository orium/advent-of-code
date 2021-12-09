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
    return g(i - 1, j) > v and g(i + 1, j) > v and g(i, j - 1) > v and g(i, j + 1) > v
end

function go(i, j, visited)
    if g(i, j) >= 9 or visited[i + columns * j] then
        return 0
    end

    local v = grid[i][j]
    local c = 1

    visited[i + columns * j] = true

    c = c + (g(i - 1, j) > v and go(i - 1, j, visited) or 0)
    c = c + (g(i + 1, j) > v and go(i + 1, j, visited) or 0)
    c = c + (g(i, j - 1) > v and go(i, j - 1, visited) or 0)
    c = c + (g(i, j + 1) > v and go(i, j + 1, visited) or 0)

    return c
end

local sizes = {}

for i = 1, #grid do
    for j = 1, columns do
        if is_low(i, j) then
            local visited = {}
            local c = go(i, j, visited)

            table.insert(sizes, c)
        end
    end
end

table.sort(sizes)

local r = 1

for i = 1, 3 do
    r = r * sizes[#sizes - i + 1]
end

print(r)
