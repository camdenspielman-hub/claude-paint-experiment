-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ engine 3

--@ chunk 1
-- Sitting 1. Ground and canvas.
canvas{size=400, aspect=1.25, linen={16, 16}, seed=7,
  ground={{pile={{"lead white", 3}, {"raw umber", 1.1}, {"yellow ochre", 0.6}}, um=38, apply="knife", texture=0.3}}}

-- brushes kept as globals for the rest of the painting
bBroad   = brush("filbert", 22)
bBody    = brush("filbert", 9)
bSoft    = brush("filbert", 26, 0.4)
bRound   = brush{kind="round", width=6, point=0.9, stiffness=0.55}
bRigger  = brush{kind="rigger", width=2.4, point=1, stiffness=0.5}
bBlender = brush("badger", 40)

print(W, H, "brushes ready")

--@ chunk 2
-- layout: jug left of centre, orange and folded cloth to the right
hPen = pencil("HB")
hp2   = pencil("H")

-- table's back edge, a touch below the middle
hPen:line({{-20, 468}, {300, 462}, {640, 470}, {1020, 464}}, {pressure=0.3})
hp2:hatch(below(function(x) return 462 + 6*math.sin(x/180) end), {angle=-0.06, pressure=0.12, spacing=9, length=22})

-- jug: belly, shoulder, neck, rim
hPen:line({{262, 572}, {258, 520}, {268, 460}, {292, 408}, {316, 378}, {322, 344},
           {318, 320}, {312, 296}, {316, 276}, {340, 268}, {378, 268}, {400, 278},
           {400, 300}, {394, 326}, {396, 352}, {408, 384}, {432, 416}, {446, 462},
           {448, 522}, {440, 572}}, {pressure={0.35, 0.3, 0.4}})

-- handle off the right shoulder
hPen:line({{392, 306}, {432, 318}, {452, 352}, {448, 392}, {420, 412}}, {pressure=0.32})

-- base line of the jug
hPen:line({{248, 574}, {340, 578}, {444, 574}}, {pressure=0.35})

-- orange
hPen:line({{588, 592}, {590, 566}, {608, 548}, {638, 542}, {668, 550}, {682, 572},
           {682, 598}, {666, 620}, {638, 628}, {608, 620}, {590, 600}},
          {pressure=0.35})

-- folded cream cloth: a low mound with a few fold ridges
hPen:line({{470, 618}, {492, 566}, {546, 526}, {624, 508}, {712, 512}, {778, 534},
           {818, 574}, {826, 620}, {812, 646}, {700, 660}, {566, 656}, {484, 644}},
          {pressure=0.3})
hPen:line({{512, 610}, {566, 552}, {648, 528}}, {pressure=0.25})
hPen:line({{628, 646}, {676, 578}, {748, 556}}, {pressure=0.25})
hPen:line({{744, 524}, {768, 578}, {800, 610}}, {pressure=0.25})

-- the group's shadow, thrown to the right
hPen:hatch(poly{{440, 570}, {560, 578}, {840, 640}, {700, 690}, {470, 640}},
           {angle=0.16, pressure=0.1, spacing=14, length=40})
print("layout drawn")

--@ chunk 3
-- palette, mixed once and kept in globals
pWall   = pile{{"lead white", 4}, {"smalt", 1.6}, {"raw umber", 1.0}, {"yellow ochre", 0.4}}
pWallD  = pile{{"smalt", 1.4}, {"raw umber", 1.8}, {"lead white", 1.2}, medium=0.15}
pWood   = pile{{"raw umber", 3}, {"vermilion", 1.2}, {"bone black", 0.8}, {"yellow ochre", 0.6}}
pWoodD  = pile{{"raw umber", 2}, {"bone black", 1.5}, {"vermilion", 0.5}}
pWoodL  = pile{{"raw umber", 1.2}, {"vermilion", 1.2}, {"yellow ochre", 0.8}, {"lead white", 0.5}}
pCast   = pile{{"raw umber", 1.6}, {"bone black", 1.3}, {"cobalt blue", 0.4}}
pJug    = pile{{"cobalt blue", 2}, {"smalt", 1.2}, {"lead white", 1.0}}
pJugL   = pile{{"cobalt blue", 1}, {"smalt", 0.8}, {"lead white", 3}}
pJugD   = pile{{"Prussian blue", 1.2}, {"cobalt blue", 1.5}, {"raw umber", 0.8}, medium=0.1}
pCloth  = pile{{"lead white", 4}, {"yellow ochre", 0.8}, {"raw umber", 0.25}}
pClothS = pile{{"lead white", 1.6}, {"raw umber", 0.9}, {"smalt", 0.3}, {"yellow ochre", 0.4}}
pOrange = pile{{"vermilion", 3}, {"chrome yellow", 1.6}, {"lead white", 0.6}, {"red earth", 0.4}}

tableEdge = function(x) return 464 + 5*math.sin(x/200) end
mWall = above(tableEdge)
dark = mask(function(x, y)
  return clamp((x - 180)/620 * 0.8 + (y - 60)/420 * 0.45, 0, 1) end)
mWallDark = ((mWall * dark):blur(26))

work(mWall, {hand="broad", tool="filbert 22", pile=pWall, coverage=1, angle=1.5, edge="soft"})
work(mWallDark, {hand="broad", tool="filbert 22", pile=pWallD, coverage=1, angle=1.5,
                 edge="lost", clip=true, pressure={0.5, 0.9}})
print("wall laid in")

--@ chunk 4
-- the wall wants to be a quiet MID grey; the ground proved lighter than expected
pWall2  = pile{{"smalt", 1.6}, {"raw umber", 2.2}, {"lead white", 0.8}}
pWall2L = pile{{"smalt", 1.2}, {"raw umber", 1.3}, {"lead white", 1.5}}

mWallLit = ((mWall * mask(function(x, y)
  return clamp(1.15 - (x - 60)/620 - (y - 20)/430, 0, 1) end)):blur(30))

work(mWall, {hand="broad", tool="filbert 22", pile=pWall2, coverage=1.3, angle=1.5,
             edge="soft", pressure={0.6, 1.0}})
work(mWall, {hand="broad", tool="filbert 22", pile=pWall2, coverage=0.5, angle=-0.4,
             edge="soft", pressure={0.5, 0.9}})
work(mWallLit, {hand="broad", tool="filbert 22", pile=pWall2L, coverage=1, angle=1.5,
                edge="lost", clip=true, pressure={0.5, 0.9}})
print("wall darkened, light side upper left")

