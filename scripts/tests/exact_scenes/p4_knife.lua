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
canvas{size=500, aspect=1.5, linen={16,17}, seed=21, ground={{pile={{"lead white",12},{"yellow ochre",0.15}}, um=120, apply="roller"}}}

--@ chunk 2
-- The knife: slabs at three pressures / scraping wet paint / a light pull over dry impasto
local Wb = pile({{"lead white",5},{"cobalt blue",0.7}, blot=0.3})
local R  = pile({{"lead white",3},{"vermilion",1}, blot=0.3})
local D  = pile({{"ultramarine blue",2},{"viridian",1},{"carmine lake",0.4},{"lead white",0.6}, blot=0.3})
local L  = pile{{"lead white",4},{"barium yellow",1}}
local kn = knife{width=36}
for i, pr in ipairs({0.25, 0.5, 0.8}) do kn:wipe(); for _ = 1, 3 do kn:load(Wb, 1.0) end; kn:lay({{40, 60 + (i-1)*70}, {200, 52 + (i-1)*70}, {330, 64 + (i-1)*70}}, {pressure={pr, pr}}) end
work(rect(380, 30, 270, 240), {hand="body", pile=R, coverage=2.2, angle=0.4, length={25,60}, load=1.0, tool={kind="filbert", width=12, lay=4}})
kn:wipe(); kn:scrape({{390, 90}, {640, 100}}, {pressure={1, 1}})
kn:wipe(); kn:scrape({{390, 200}, {640, 215}}, {pressure={0.55, 0.55}})
work(rect(40, 320, 920, 300), {hand="body", pile=D, coverage=2.4, angle=0.5, length={25,60}, load=1.0, tool={kind="filbert", width=12, lay=5}})
print(wait(10*24*60))
for i, pr in ipairs({0.55, 0.35, 0.2}) do kn:wipe(); for _ = 1, 3 do kn:load(L, 1.0) end; kn:lay({{60, 360 + (i-1)*90}, {500, 370 + (i-1)*90}, {940, 355 + (i-1)*90}}, {pressure={pr, pr}}) end
print(wait(60))

