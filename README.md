# Projektna naloga pri Programiranju 2
Hana Lah, Tinkara Bostič

## Opis projekta

Igrica po navdihu znane igrice **Geometry Dash**, napisana v programskem jeziku `Rust` s pomočjo knjižnice `Macroquad`.

## Navodila za igranje
Igralec preskakuje različne ovire, ki se mu približujejo, pri čemer lahko pristane na ovirah z ravnim vrhom in takoj umre ob stiku s trikotniki/špičastim vrhom ovire. Igrica traja poljubno dolgo, točke pa se štejejo glede na čas igranja. Ob pavzi je viden najvišji dosedaj dosežen rezultat.

Na voljo sta 2 stopnji: **Beginner** in **Advanced**, ki ju je možno zamenjati ob pavzi. Na stopnji **Beginner** lahko igralec naredi 1 skok naenkrat in preskakuje le kvadrate in trikotnike različnih velikosti. Na stopnji **Advanced** pa lahko igralec naredi 3 skoke zaporedoma (brez vmesnega pristanka), preskakuje pa skupine trikotnikov oz. kvadratov, stolpe in stopnice.

Za skok lahko igralec uporabi tipko za presledek ali klik z miško.

## Navodila za zagon
**1. možnost**: Igrico se lahko prenese iz **GitHub Releases**, kjer so na voljo ZIP datoteke za različne operacijske sisteme. Po razpakiranju uporabnik zažene izvršno datoteko.

**2. možnost**: Če ima uporabnik nameščen Rust, lahko igrico zažene v urejevalniku z gumbom **Run** oz. v ukazni vrstici z ukazom `cargo run --release`.

Izvorno kodo lahko uporabnik pridobi z ukazom:

`git clone https://github.com/bostic-tinkara/GeometryDash.git`

Nato se premakne v mapo projekta:

`cd GeometryDash`

in zažene igrico:

`cargo run --release`.
