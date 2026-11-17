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
canvas{size=400, aspect=1.25, linen=16, seed=5, ground={{pile={{"lead white",3},{"yellow ochre",0.5}}, um=60, apply="knife", texture=0.4, absorbent=0.5}}}
h = pencil("HB"); h:hatch(rect(100, 100, 300, 200), {angle=-1.1, pressure=0.35}); h:line({{80, 500}, {500, 470}, {900, 520}}, {pressure=0.5})

--@ chunk 2
p = pile{{"lead white",2},{"cobalt blue",1}, turps=0.4}
work(rect(50, 50, 600, 400), {hand="broad", pile=p, coverage=1.5, angle=0.1})
b = brush("filbert", 10); b:load(pile{{"vermilion",1},{"lead white",1}}, 0.8); b:stroke({{100, 600}, {500, 640}, {800, 650}}, {pressure=0.7})
stipple(rect(500, 300, 300, 300), {pile=pile{{"viridian",1},{"lead white",1}}, width=8, coverage=1})
blend(rect(50, 50, 600, 400), {angle=0.2})
r = rag(); r:wipe(rect(200, 150, 200, 150), {pressure=0.5}); r:blot(600, 500, {pressure=0.6})
k = knife{width=25}; k:load(pile{{"cadmium yellow",1},{"lead white",2}}, 0.8); k:lay({{150, 680}, {450, 700}}, {pressure={0.5, 0.7}})
print(wait(90))

--@ chunk 3
work(rect(100, 100, 700, 500), {hand="glaze", pile=pile({{"carmine lake",1}, medium=0.8}), coverage=0.6})
work(rect(300, 400, 400, 300), {hand="scumble", pile=pile{{"lead white",3},{"cobalt violet",1}}, coverage=0.7})
b2 = brush{kind="round", width=6, point=0.9, stiffness=0.55}; b2:load(pile{{"ultramarine blue",1}}, 0.9); b2:stroke({{120, 300}, {880, 280}}, {pressure={0.3, 0.8}})
print(wait(3*24*60))
