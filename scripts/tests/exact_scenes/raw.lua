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
canvas{size=300, aspect=1.5, linen=14, seed=9, raw="cotton duck"}
work(rect(100, 100, 600, 400), {hand="broad", pile=pile{{"yellow ochre",1},{"ultramarine blue",0.3},{"lead white",1}, turps=0.5}, coverage=1.2})
print(soaked(300, 300))
print(wait(24*60))
