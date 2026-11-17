-- easel session "bench": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box giverny
--@ engine 9
--@ knife-skim
--@ palette-carried-paint
--@ limbs-join-nearest
--@ reload-finite
--@ blend-feather
--@ brush-skim
--@ blend-at-masks
--@ glaze-film
--@ liquid-film-levels
--@ sealed-drawing-guide
--@ drawing-optics
--@ rag-lift
--@ full-palette-refuses
--@ pastel-current-heap
--@ calls-agree
--@ drain
--@ paper

--@ chunk 1
canvas{size=600, aspect=1.5, linen={16,17}, seed=77, ground={{pile={{"lead white",12},{"yellow ochre",0.25}}, um=130, apply="brush"}}}
horizon = 380
poplars = {}
treesM = nil
for i, x in ipairs({118, 160, 196, 612, 650, 695, 735, 782}) do
  poplars[#poplars+1] = {x = x + rand(-8, 8), h = 200 + 55 * math.sin(i * 1.7) + (i % 3) * 18, w = 15 + rand(0, 7)}
end
local function tree(p) return poly({{p.x - p.w, horizon + 4}, {p.x - p.w*0.85, horizon - p.h*0.5}, {p.x - p.w*0.4, horizon - p.h*0.88}, {p.x, horizon - p.h}, {p.x + p.w*0.45, horizon - p.h*0.86}, {p.x + p.w*0.9, horizon - p.h*0.48}, {p.x + p.w, horizon + 4}}, true) end
for _, p in ipairs(poplars) do local t = tree(p); treesM = treesM and (treesM + t) or t end
treesM = treesM:roughen(4, 18, 3)
bankM = rect(0, horizon - 18, 1000, 28):roughen(5, 40, 5)
sky = above(function(x) return horizon - 6 end) - treesM - bankM
water = below(function(x) return horizon + 8 end)
-- reflections: under each poplar, a column fading downward, broken by ripples
reflM = mask(function(x, y)
  if y < horizon + 8 then return 0 end
  local v = 0
  for _, p in ipairs(poplars) do
    local d = math.abs(x - p.x) / (p.w * 0.9)
    local depth = (y - horizon) / (p.h * 0.8)
    if d < 1 and depth < 1 then v = math.max(v, (1 - d * d) * (1 - depth) ) end
  end
  return v
end)
-- lay-in: lean, with turpentine
work(sky, {hand="broad", pile=pile{{"lead white",2},{"carmine lake",0.35},{"barium yellow",0.35}, turps=0.6}, coverage=1, angle=0})
work(water, {hand="broad", pile=pile{{"lead white",1.2},{"cobalt blue",1},{"carmine lake",0.35}, turps=0.6}, coverage=1, angle=0})
work(treesM + bankM, {hand="body", pile=pile{{"ultramarine blue",1},{"viridian",0.8},{"carmine lake",0.5},{"lead white",0.3}, turps=0.6}, coverage=1.2, angle=math.pi/2, tool="filbert 8", length={30,80}})
print(wait(24*60))

--@ chunk 2
-- the sky: warm near the horizon, cooler above; a few close colors in long strokes, softened
local glowM = sky * rect(0, horizon - 200, 1000, 200):soften(90)
local highM = sky - rect(0, horizon - 150, 1000, 160):soften(90)
for _, p in ipairs({pile{{"lead white",4},{"barium yellow",0.7},{"vermilion",0.08}}, pile{{"lead white",4},{"carmine lake",0.4},{"barium yellow",0.2}}, pile{{"lead white",4},{"deep cadmium",0.25}}}) do
  work(glowM, {hand="body", pile=p, coverage=0.75, angle=0.01, length={50,130}, tool="filbert 14", streak=0.4, clip=sky})
end
for _, p in ipairs({pile{{"lead white",3},{"cobalt blue",0.7},{"carmine lake",0.25}}, pile{{"lead white",3},{"cobalt violet",0.8},{"cobalt blue",0.2}}, pile{{"lead white",3.5},{"cobalt blue",0.5},{"viridian",0.12}}}) do
  work(highM, {hand="body", pile=p, coverage=0.75, angle=0.03, length={60,150}, tool="filbert 16", streak=0.4, clip=sky})
end
blend(sky, {angle=0.02})
-- the poplars and bank, against the light: violet-blue with a green edge on the brush
local treeP = pile{{"ultramarine blue",1},{"carmine lake",0.55},{"viridian",0.35},{"lead white",0.7}}
local treeG = pile{{"viridian",1},{"cadmium yellow",0.35},{"carmine lake",0.15},{"lead white",0.5}}
work(treesM, {hand="body", pile=treeP, coverage=1.5, angle=math.pi/2, angle_jitter=0.12, length={20,60}, tool="filbert 6", second={pile=treeG, load=0.45, side=-1, share=0.45}, streak=0.5, clip=treesM:grow(2)})
work(bankM, {hand="body", pile=treeP, coverage=1.5, angle=0, length={20,50}, tool="filbert 7", second={pile=treeG, load=0.4, side=1, share=0.4}, clip=bankM:grow(2)})
-- the water: the sky mirrored, darker and cooler, in horizontal strokes
local nearM = water * rect(0, horizon + 140, 1000, 400):soften(80)
for _, p in ipairs({pile{{"lead white",2.5},{"cobalt blue",0.8},{"carmine lake",0.3}}, pile{{"lead white",3},{"carmine lake",0.45},{"barium yellow",0.15}}, pile{{"lead white",2.5},{"cobalt violet",0.8},{"cobalt blue",0.2}}}) do
  work(water, {hand="body", pile=p, coverage=0.7, angle=0, length={40,110}, tool="filbert 11", streak=0.4, clip=water})
end
work(nearM, {hand="body", pile=pile{{"lead white",1.5},{"cobalt blue",1},{"carmine lake",0.3},{"viridian",0.1}}, coverage=0.6, angle=0, length={40,120}, tool="filbert 12", broken=0.3, clip=water})
-- reflections of the poplars: short horizontal strokes, columns broken by ripples
work(reflM:map(function(v) return v > 0.12 and v or 0 end), {hand="body", pile=pile{{"ultramarine blue",1},{"carmine lake",0.5},{"viridian",0.3},{"lead white",1.4}}, coverage=1.1, angle=0, length={8,22}, tool="filbert 5", broken=0.4, second={pile=treeG, load=0.3, streak=0.9}, clip=water})
print(wait(4*24*60))

