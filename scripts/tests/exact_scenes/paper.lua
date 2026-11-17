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
canvas{size=300, aspect=1.4, paper={}}
p = pastel(pile{{"vermilion", 1}, {"lead white", 2}}, {kind="soft"})
p:stroke({{100, 200}, {800, 260}}, {force=2, alt=60, azimuth=45, speed=80})
p:side({{150, 400}, {700, 420}}, {force=1.5})
q = pastel(pile{{"cobalt blue", 1}}, {kind="pencil"}); q:line({{100, 550}, {900, 520}}, {pressure=0.5})

--@ chunk 2
smudge({{200, 220}, {500, 240}}, {force=1})
fix(rect(0, 0, 1000, 350))
h = pencil("2B"); h:line({{100, 600}, {900, 620}}, {pressure=0.5}); erase({{300, 610}, {500, 615}}, {strength=0.9, width=9})
p:stroke({{150, 300}, {850, 330}}, {force=3, alt=40, azimuth=30, speed=60})
