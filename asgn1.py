#!/usr/bin/env python3

import subprocess


def render(to, *args):
    cli = ["just", "run-release", "asgn1", "-input",
           "data/asgn1/FerrisWheel.jpg"] + list(args) + ["-output", to + ".png"]
    subprocess.check_call(cli)
    print('<div class="card"><img class="card-img-top" src="pictures/{}.png"></img><div class="card-body"><p class="card-text"><code>{}</code></p></div></div>'.format(to, " ".join(cli)))


render("b2", "-brighten", "2")
render("b5", "-brighten", "0.5")
render("blur", "-blur")
render("c0", "-contrast", "0")
render("c01", "-contrast", "0.1")
render("c2", "-contrast", "2")
render("chanb", "-channel", "blue")
render("chang", "-channel", "green")
render("chanr", "-channel", "red")
render("dither-random", "-random-dither", "1")
render("dither-floyd-steinberg", "-floyd-steinberg-dither", "1")
render("edges", "-edge-detect")
render("edges-base", "-edge-detect-base")
render("gray", "-grayscale")
render("noise", "-random-noise")
render("s0", "-saturation", "0")
render("s4", "-saturation", "4")
render("sharpen", "-sharpen")
for s in ("bilinear", "gaussian", "point"):
    render("crop-" + s, "-sample", s, "-crop", "300", "300", "200", "200")
    render("rotate-" + s, "-sample", s, "-rotate", "20")
    render("scale-1-" + s, "-sample", s, "-scale", "1", "3")
    render("scale-2-" + s, "-sample", s, "-scale", "2", "1")
