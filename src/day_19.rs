use core::cmp::Ordering;
use petgraph::prelude::*;
use petgraph::Graph;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;

fn find_boss(pairs: &Vec<Vec<&str>>) -> usize {
    let mut g = Graph::new_undirected();
    let mut lookup: HashMap<&str, NodeIndex> = HashMap::new();
    let mut rev_lookup: HashMap<NodeIndex, &str> = HashMap::new();
    let mut all_sets: HashSet<Vec<&str>> = HashSet::new();
    let mut visited: HashSet<NodeIndex> = HashSet::new();

    for node in pairs {
        let left = get_node_idx(&node[0], &mut lookup, &mut g);
        rev_lookup.insert(left, node[0]);
        let right = get_node_idx(&node[1], &mut lookup, &mut g);
        rev_lookup.insert(right, node[1]);

        g.add_edge(left, right, 1.0);
    }

    for (key, _value) in rev_lookup.clone() {
        create_three_sets(&key, &mut all_sets, &mut visited, &g, &rev_lookup);
    }

    all_sets
        .iter()
        .filter(|v| v[0].starts_with('t') || v[1].starts_with('t') || v[2].starts_with('t'))
        .count()
}

fn find_max_connections(pairs: &Vec<Vec<&str>>) -> String {
    let mut g = Graph::new_undirected();
    let mut lookup: HashMap<&str, NodeIndex> = HashMap::new();
    let mut rev_lookup: HashMap<NodeIndex, &str> = HashMap::new();
    let mut all_sets: HashSet<Vec<&str>> = HashSet::new();

    for node in pairs {
        let left = get_node_idx(&node[0], &mut lookup, &mut g);
        rev_lookup.insert(left, node[0]);
        let right = get_node_idx(&node[1], &mut lookup, &mut g);
        rev_lookup.insert(right, node[1]);

        g.add_edge(left, right, 1.0);
    }

    let mut all_sets: HashSet<Vec<&str>> = HashSet::new();
    let mut max_set: Vec<&str> = vec![];
    for (key, _value) in rev_lookup.clone() {
        let mut visited: HashSet<Vec<NodeIndex>> = HashSet::new();
        let mut set: Vec<NodeIndex> = vec![];

        maximum_set_for_node(&key, &vec![], &mut &mut set, &mut visited, &g, &rev_lookup);
        let mut new_names: Vec<&str> = vec![];
        for node in set.clone() {
            let node_name = rev_lookup.get(&node).unwrap_or(&"");
            new_names.push(node_name.clone());
        }
        new_names.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        if max_set.len() < new_names.len() {
            max_set = new_names.clone();
        }
        all_sets.insert(new_names.clone());
    }

    //println!("All sets: [{:?}]", all_sets);

    max_set.join(",")
}

fn maximum_set_for_node<'a>(
    node: &NodeIndex,
    path: &Vec<NodeIndex>,
    set: &mut Vec<NodeIndex>,
    visited: &mut HashSet<Vec<NodeIndex>>,
    g: &Graph<(), f32, Undirected>,
    rev_lookup: &HashMap<NodeIndex, &'a str>,
) -> () {
    let mut requested_path = path.clone();
    requested_path.push(node.clone());
    if visited.contains(&requested_path) {
        return;
    }

    visited.insert(requested_path.clone());

    for check in set.clone() {
        if !g.find_edge(node.clone(), check.clone()).is_some() {
            return;
        }
    }

    let neighbors = g.neighbors(node.clone());

    if neighbors.count() < set.len() {
        return;
    }

    set.push(node.clone());

    for (a, _) in rev_lookup {
        if a != node {
            maximum_set_for_node(&a, &requested_path, set, visited, g, rev_lookup);
        }
    }

    return;
}

fn create_three_sets<'a>(
    node: &NodeIndex,
    all_comb: &mut HashSet<Vec<&'a str>>,
    visited: &mut HashSet<NodeIndex>,
    g: &Graph<(), f32, Undirected>,
    rev_lookup: &HashMap<NodeIndex, &'a str>,
) -> () {
    if visited.contains(&node) {
        return;
    }

    visited.insert(node.clone());

    let node_name = rev_lookup.get(&node).unwrap_or(&"");
    for a in g.neighbors(node.clone()) {
        let a_name = rev_lookup.get(&a).unwrap_or(&"");
        for b in g.neighbors(a.clone()) {
            if b != a && b != *node {
                //is b neighbor with node too?
                for c in g.neighbors(node.clone()) {
                    if c == b {
                        //we have a set...
                        let b_name = rev_lookup.get(&b).unwrap_or(&"");
                        let mut new_set: Vec<&str> = vec![node_name, a_name, b_name];
                        new_set.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
                        all_comb.insert(new_set.clone());
                    }
                }
            }
        }
    }
}

fn parse_input(source: &str) -> Vec<Vec<&str>> {
    source
        .split('\n')
        .map(|line| line.split('-').collect())
        .collect()
}

fn get_node_idx<'a>(
    node: &'a str,
    lookup: &mut HashMap<&'a str, NodeIndex>,
    g: &mut Graph<(), f32, Undirected>,
) -> NodeIndex {
    let idx = if let Some(idx) = lookup.clone().get(node) {
        *idx
    } else {
        g.add_node(())
    };

    lookup.insert(node, idx);
    idx.clone()
}

fn main() {
    let source = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    assert_eq!(7, find_boss(&parse_input(&source)));
    assert_eq!(
        "co,de,ka,ta".to_string(),
        find_max_connections(&parse_input(&source))
    );

    let source = "lq-xs
vx-pm
ea-ee
tp-jz
iv-nn
kk-zw
es-ni
ml-cy
jr-tl
lu-ot
cd-bv
wo-pp
bv-ee
ii-ij
mz-uw
cf-uo
au-gy
lt-up
ox-jl
wy-ij
pa-br
gm-yj
tl-cy
cg-qw
vy-ql
gx-iv
yh-bi
zl-qt
me-wg
pj-dt
sa-nc
sl-mt
lo-za
po-zl
qw-hy
ze-fq
er-fe
xs-vw
ak-rm
nr-lp
ja-ga
ey-bu
pa-de
xs-ep
zj-fn
ti-fy
im-zt
dw-oo
jp-ll
ky-zk
iu-pn
qp-xk
na-lb
og-tf
ni-oz
dp-jp
pp-tg
uq-to
hw-ci
xd-ov
hy-na
ii-fh
fk-lg
uq-up
gy-kh
pb-ra
oc-xr
qr-ho
qm-zl
nc-uk
mb-lo
rd-sk
oo-lm
fo-rl
iv-sl
ij-io
iz-tv
ao-ft
nw-ml
ey-gw
yl-nh
et-dk
qr-ee
ey-wr
tb-ua
fu-ln
hr-gu
fq-tv
ue-lh
fm-iy
xp-cw
us-ol
ts-xe
jo-il
gn-cq
ad-bv
hj-kx
xp-zj
ng-ak
yf-qo
ra-qw
iv-cu
jm-gn
uq-mc
gt-lu
ak-jz
oc-xb
fv-ac
ih-pl
de-ox
lh-mj
do-eb
wp-iv
si-fl
od-pu
co-kq
se-zx
az-px
rj-cp
fl-nk
bp-jo
ss-jg
wx-jx
lu-tf
fe-jc
et-ze
at-sn
qp-yn
sn-qt
kp-lu
ui-zk
kc-la
vo-su
vr-aw
iz-lc
ov-cm
tw-qu
wr-gw
pu-qj
ri-lw
rn-ig
qm-at
yx-gm
zt-cz
xt-cp
yy-kf
oq-sz
ye-ih
nw-cy
tu-uo
zn-wp
md-rp
lc-aw
il-nt
pp-zs
dg-vu
hq-ex
gj-sq
as-kk
ar-pr
qs-wx
ru-mk
ms-an
qf-iz
gx-yr
gv-ku
jc-za
pc-mu
qo-ba
sb-ef
jo-mo
ag-lr
tk-pp
wx-ct
xh-gg
cl-us
bg-fy
ah-nf
rl-ft
vz-im
sg-of
pd-jg
th-xn
zl-us
nl-fl
wl-dh
yu-bp
wz-nw
ww-jq
ui-ky
ge-yp
vk-cz
yd-ov
wk-bx
ui-um
sb-vj
zo-wr
fk-xt
og-gv
nr-ou
br-qm
zn-ki
kz-nc
wr-gq
qs-nf
oc-mf
up-ra
ja-gf
md-ss
ar-qy
cx-it
ho-hj
cs-me
ck-ts
lq-ka
tw-hw
np-dm
ny-ge
tj-kx
ho-iz
ho-ad
kk-kn
kg-by
nu-zt
wo-tk
vk-in
lb-et
bb-ou
ak-zr
az-gv
ux-gq
wv-am
oz-ib
ei-gq
vy-ip
ou-cw
if-mq
ay-vu
zi-xq
ra-sb
ac-yu
tl-wz
uq-ru
ly-xh
ly-rj
jb-nn
wx-nf
tc-gt
kg-yt
tk-sz
tf-px
ok-bi
yy-ct
rp-kz
al-of
ud-ai
xe-ok
vp-nw
bc-ag
dd-nb
px-lu
me-rj
tp-rm
mb-fe
ay-zj
si-ew
rg-bt
rh-et
by-yt
ib-sk
xl-zf
af-dw
xy-hp
gf-ga
vo-ev
ad-xv
vu-zj
zw-cx
od-qg
zg-rh
dm-gx
ol-qt
ef-pb
wv-qu
qm-sn
mg-td
nh-vm
or-yy
rd-qk
xu-tu
le-yq
ac-te
nw-qj
zo-tq
jc-uz
bb-fn
mu-zf
nw-ch
la-vw
lq-as
oj-sq
ms-xx
gb-ed
cl-qk
oj-ia
lm-al
sf-ep
rv-xr
gx-nn
ju-aq
ed-uh
tl-ch
pz-uc
tj-ee
sr-cm
ig-kk
ix-bw
nz-wo
ul-ue
yh-ef
tr-yj
ip-hk
yo-zn
dv-ge
nn-ur
xk-jw
op-gy
kk-xs
ul-de
yu-av
sh-ie
kv-kk
tu-dv
ip-pp
of-dw
tm-gp
mw-nx
gc-yt
mk-zc
ec-fo
oc-pd
vb-do
um-de
zu-mn
uo-dv
nu-iq
wv-my
vk-or
lp-cw
gu-pt
oj-vo
pl-xq
uw-co
tm-ql
yy-zt
hc-cd
yo-yr
jh-rv
cf-dv
cl-qm
og-kp
na-bc
xv-hq
cw-dg
oj-ry
jb-dw
at-us
ec-ms
tq-gw
qk-qt
pu-zm
vq-ew
oq-wn
xh-me
yx-ll
vq-lf
vk-au
xk-kz
ni-vb
xg-dh
if-ti
yd-fo
tj-hj
if-ew
jh-xb
up-wf
we-hq
in-zt
gq-gu
bh-jx
wl-dl
rw-fs
we-sr
az-tc
gm-re
mi-sd
tk-ql
ow-kk
kz-gs
yi-jq
ig-va
bj-fg
dk-qg
zr-rm
nz-ql
mg-gi
gn-wl
mb-sj
ex-ee
od-yd
ov-od
kp-se
ra-bi
ea-bm
yy-cz
hc-ee
re-tr
as-pe
ia-ev
iy-oe
cp-me
zm-tl
yh-xe
we-bx
sr-mj
ab-jc
um-zk
fm-io
gt-rn
nc-yn
or-tc
fx-hn
xh-cp
mw-pn
cm-vn
wd-qq
bh-oj
xr-pr
ki-xd
ph-yi
cu-sl
nt-yi
zt-vz
jp-gm
oy-fj
ck-cm
ef-bi
yz-md
bi-sb
an-aq
ry-cx
vy-zq
tq-rs
yp-rs
za-fe
ao-cu
ht-pc
gf-mi
ge-uo
lw-qq
bp-mo
ot-gv
uw-kq
fj-nh
ck-sr
yi-ri
ht-hr
jz-hx
io-oe
lm-of
rm-cv
uf-cm
eh-sw
pe-zw
wo-wn
sb-xe
fh-wy
if-fl
sz-wy
yg-my
ma-fj
qm-ol
pn-tt
wd-qt
xy-jm
cx-sq
sa-uk
yv-bp
od-aq
bz-zk
ta-mi
bm-gy
in-or
fg-tp
lb-dk
no-uq
mi-fk
lq-kv
rj-kq
lh-we
su-fh
ly-cp
hj-ad
nx-wx
tg-ma
py-ar
ad-ex
gi-ba
gu-zo
bo-bb
fu-pg
zs-vy
cq-gz
dp-yx
se-az
ok-vj
ns-kr
kd-vr
kc-xf
vn-sr
sa-md
qt-sk
xr-mf
cf-dt
ve-ry
ei-ab
au-or
yi-wd
we-ue
qg-uh
xf-ih
te-qv
bv-hc
hw-ib
gq-bu
vm-rn
ol-po
nf-ct
ni-do
cd-qr
xx-an
uw-xh
es-mj
cs-kq
td-mi
nn-mt
pq-yt
iq-in
xu-dv
yn-rp
cm-jv
nr-zj
kn-sf
yw-tu
lm-gc
js-ei
dk-rj
co-cp
ql-hk
ns-pl
zg-lb
xq-wm
qu-yg
xs-as
qv-ro
ux-yp
st-ug
ve-vo
vk-im
wl-jk
vx-zf
oe-ii
kh-tm
ah-jd
se-gt
ux-zo
he-sg
gf-wa
zr-jz
dt-uo
ty-pa
og-se
rn-oy
gs-ss
uh-lr
tr-tb
vf-ef
iw-nb
kd-ze
vs-yp
jb-sg
ss-sa
mo-vs
ws-jr
yf-gi
yj-ua
ta-bt
lg-mg
tx-pl
ef-ra
vx-xt
zs-hk
si-nk
lc-np
so-pd
kq-mz
lg-rg
bw-gf
rv-jg
nu-au
jw-kz
gy-ie
dd-bw
ai-ui
qp-md
lu-gv
kx-xv
vk-ry
jz-pz
ch-pu
es-jv
gj-ia
se-lu
wp-ur
to-dj
vr-dm
ye-zi
eh-uy
cq-ds
yd-ki
mn-wd
wl-jm
ri-qc
uz-er
od-an
ty-um
rj-cs
cx-oj
op-vl
xp-ou
ur-zn
fy-si
af-sg
np-fq
re-ua
gz-jm
ue-vn
fn-lp
gp-op
bh-ev
jq-dt
fq-aa
nb-ix
jx-ah
ei-fe
ko-nn
va-kt
qu-kg
yl-rn
vx-pc
ai-ma
lg-yf
uk-di
it-dt
vq-hv
ni-rl
vl-mw
va-tg
kk-sf
pb-fb
md-di
fe-sj
ar-mf
vu-lp
mb-ab
vy-pp
zn-ko
iy-vf
tw-st
ci-rl
bm-tm
qm-xq
mn-il
ht-mu
np-kd
ns-xq
qf-np
ux-gu
tq-pt
hx-zr
wn-sz
xo-of
ww-dv
uq-ys
es-we
qy-qq
vj-pb
kh-gp
hj-ee
oo-al
hq-ho
bh-ry
ev-nm
pq-ug
rh-qg
fe-cj
qr-ex
gm-rg
yp-pt
ol-qk
za-ei
ao-sl
fq-vd
ln-bp
jl-pa
lw-zu
nx-ah
sr-bx
zl-ol
gy-fc
zu-il
aa-np
qc-yb
ur-mt
hr-xt
by-qu
fj-ig
uw-jd
nn-cu
xv-ee
yg-by
ia-qj
dp-eh
cy-ch
ua-eh
gj-vo
ud-tg
qi-ng
pt-gw
ru-dj
pq-am
px-tc
ga-iw
yd-ms
tj-qr
lx-jx
kx-hq
il-qy
uh-ag
py-gr
fr-ws
mq-hv
po-sk
dh-cq
td-rg
yr-ko
hj-cd
lo-js
bm-vl
cz-ld
si-hv
xe-ef
iq-au
ro-yu
tb-dp
xn-fh
ex-xv
pa-vd
od-xx
ty-ky
yl-ud
iy-wy
xu-ou
ab-cj
se-ku
ck-ue
lp-dd
yx-sw
ix-ga
ip-nz
ns-wm
my-mr
yw-pj
rs-bu
cy-fr
af-of
ij-oe
qc-nt
vb-le
tq-ux
wv-ug
jr-cy
xh-co
vq-mq
yh-vj
ec-aq
vk-kf
zf-xt
ra-fb
zj-cr
qu-st
wr-tq
hr-pm
di-qp
vp-cy
vf-fm
kk-pw
af-gc
bt-lg
oz-eb
pc-xt
oy-ig
le-hw
zk-jl
gv-tf
xv-bv
gi-jz
wz-ch
dp-re
ch-zm
jb-of
up-mk
xd-xx
af-oo
gp-fc
cw-vu
ys-io
jd-pn
ck-es
fr-nw
rh-lr
ny-pj
il-yb
lb-rh
pb-gg
pm-ht
kf-vz
gu-ey
mr-qu
sg-lm
gg-rj
kd-aw
hx-uc
ur-yo
th-lq
ay-fn
ij-vf
kc-wm
bt-sd
yy-vk
nz-tk
jo-av
pu-vp
le-do
rh-xn
kq-me
td-ta
hz-jc
ol-ms
st-my
lc-ze
hp-bo
de-az
uf-vr
iw-df
ng-bj
cv-pz
su-ia
jx-nx
us-qk
dj-yz
zq-nz
gb-dk
xb-pd
ln-mo
dr-aq
nm-sq
in-vz
jw-sa
cj-va
pc-xl
sa-rp
jq-yw
xp-nr
mf-so
ai-yl
ni-le
mu-hr
qp-nc
ar-so
jl-um
wk-oo
ri-nt
uc-zr
mj-ts
vn-ck
cr-dg
yn-jw
fm-fh
dh-gn
jw-ss
yg-st
yh-qw
ak-hx
te-av
wv-st
ou-lp
us-br
ja-lz
ue-sr
ow-lq
aa-kd
gp-bm
ar-xb
ug-us
iw-zr
br-po
yp-wr
pp-nz
ih-kc
hq-cd
wx-tt
mk-yz
yb-ph
ri-qq
ge-pj
qc-wd
ci-vb
il-qc
mo-av
nt-mn
uw-cs
bz-jl
ui-he
lw-qy
zl-at
ms-aq
fw-ns
wn-tf
mw-jd
jx-mw
ds-xy
cs-cp
qc-wa
yg-kg
ue-uf
ok-qw
lo-jc
kd-qf
ot-kp
yq-ib
no-up
yq-do
az-kp
bm-fc
nx-nf
my-kg
pg-ln
fc-ie
wv-kg
pz-tp
wo-vy
mq-nk
ud-nh
rl-ib
ts-sr
ma-po
vz-vk
wo-ql
ib-zk
ri-ph
ys-mk
hk-tk
bz-de
qk-po
qf-dm
he-af
mu-xr
bu-yp
kv-kn
vf-oe
ib-vb
ow-xs
ll-sw
ds-wl
he-gc
yz-wf
wf-zc
rp-jw
im-cz
qm-us
fq-gk
mr-ug
tu-ge
sb-pb
cg-hy
jg-xr
tw-kg
zg-lr
rq-bw
me-ir
py-jh
eh-tb
yb-nt
hn-pm
ka-xs
jl-ky
yg-mr
bt-qo
wd-lw
ln-te
ie-yv
cr-vu
qr-hj
oz-do
or-kf
xx-ki
es-ue
iz-bh
iv-ao
ph-zu
qr-hc
re-nr
ra-xe
ze-np
yp-gw
od-fo
tk-ip
ge-qb
rm-fg
vd-ui
fh-io
lb-lr
bx-lh
sh-kh
ga-uo
df-dd
nr-fn
dp-jh
wj-gf
oc-ar
mw-ct
um-vd
pm-fv
sq-ev
or-nu
td-ri
tw-wv
ex-ho
ak-ds
xo-gh
fm-xn
jc-js
ny-jq
yq-ci
so-xb
fn-cw
ny-tb
gs-jw
lm-he
ny-yw
xs-kv
nf-mw
pw-sf
nc-yw
kr-kc
dl-cq
al-gh
uo-xu
oa-dx
sl-gx
ty-zk
tg-ig
lw-mn
gb-fy
in-cz
sz-ql
al-jb
gq-tq
mq-gj
dl-ds
bz-ui
qi-hx
xf-pl
wf-no
yx-eh
cs-mz
oa-oe
mc-zc
cd-ho
aq-xx
wj-rq
gz-dh
lo-uz
ir-cp
gh-gc
sj-za
uz-ei
tu-jq
ni-ci
ux-ey
nh-ai
sw-gm
ec-ov
pa-zk
er-cj
nc-md
wz-vp
bi-xe
nx-jd
wd-yb
vr-qf
cf-yw
fk-ba
vq-fy
zw-xs
pl-ye
hr-pc
sy-mu
zo-rs
yd-an
ys-zc
gr-so
qv-jo
kn-pe
hq-qr
wz-ml
iu-ct
tr-jp
kg-ug
iu-mw
ui-de
ow-pw
bh-nm
nm-ve
zf-fx
uc-ng
wa-ga
zc-lt
pe-kk
pr-jh
uc-fg
jg-xb
no-zc
lt-wf
tu-dt
tg-oy
lo-er
nf-pn
fg-ng
dp-sw
mz-gg
nx-lx
ro-mo
yn-xk
ld-iq
yy-im
le-ci
qy-ph
sk-qm
pq-kg
oo-gh
fl-hv
kz-uk
pe-lq
yx-uy
ct-pn
kc-qr
gy-sh
ng-cv
ui-ty
ra-yh
ov-ju
ds-bo
yn-uk
ij-xn
bo-xg
qo-td
bo-cq
qc-lw
zf-hn
ll-uy
pz-fg
ul-bz
ta-yf
ij-nu
ay-bb
gz-ll
rp-gs
ur-yr
jk-xy
ny-tu
im-ag
sn-zl
fy-fl
rm-uc
ws-wz
wn-ip
tu-as
zj-pv
ac-jo
cx-bh
te-bp
jz-uc
pj-cf
lu-az
qs-jd
vz-cz
xu-cf
ea-gp
ma-ig
mn-ri
tq-oc
ab-er
ti-nl
ex-tk
tb-gm
by-my
kh-ie
lq-pw
hy-pb
kr-do
mb-jc
of-wk
gg-kq
cg-pb
qi-zr
tp-uc
gr-xb
kk-ep
la-wm
rs-vs
pr-pd
ni-yq
ds-gz
zm-nw
py-pr
fb-bi
ns-la
xp-av
lx-ct
pw-kv
ti-vq
lb-uh
io-xn
rw-fc
zo-bu
so-py
wk-jb
ep-ka
ua-jp
pg-av
il-qq
fj-tg
jo-pg
rs-gu
co-ir
jv-lh
iu-lx
pw-as
kh-op
wa-nb
hx-bj
ri-il
oz-ft
zq-pp
jv-vn
hk-wo
og-ni
ml-zm
js-fe
ht-hn
ul-ty
nw-oa
tp-ak
rp-nc
nk-lf
ey-zo
nc-jw
cl-sk
th-wy
bw-wa
ri-qy
ij-dx
oa-vf
zi-kc
sj-uz
zx-tf
yy-iq
ws-qj
rl-do
pb-bi
rg-ba
ix-gr
by-jv
of-he
md-gs
nr-bb
ie-op
pv-dg
su-sq
ul-ox
uq-mk
gs-qp
ec-ju
ve-oj
bh-vo
pp-hk
xr-xb
ih-er
fs-op
ly-uw
ll-gm
pv-ht
ea-rd
nm-it
qb-xu
nr-ay
tr-yx
xl-xt
ad-hc
gm-eh
gs-sa
qr-ad
sg-xo
qj-wz
ww-yw
sf-kv
cr-cw
zt-or
cp-mz
tx-ky
by-pq
tx-jl
qc-qy
lf-ti
su-bh
fn-pv
zo-gw
yw-ge
cy-wz
gy-gp
hy-ef
rh-ag
qt-gk
ac-bp
st-kg
ba-ta
xu-ww
wj-lz
ix-gf
wg-cs
ea-fs
sf-as
nb-wj
qg-gb
zs-tx
ep-lq
ua-sw
gx-cu
yv-yu
kh-fs
gj-su
dx-xn
fu-yv
we-ck
gx-wp
ko-wp
cu-zn
ki-ec
dh-dl
cd-xv
as-kh
pg-ac
oa-ij
hq-tj
hw-do
vr-ze
yz-mc
lf-ii
ay-cw
hv-if
mf-mk
hv-fy
ld-yh
yi-qy
tt-ij
ua-yx
re-uy
gs-uc
ki-ju
tb-yj
zi-fw
fm-th
jg-pr
et-na
vj-hy
zc-wj
rv-pr
hz-uz
vw-ns
md-uk
yf-fk
zt-ip
es-uf
gs-xk
wv-yt
tr-dp
za-uz
sa-qp
di-sa
vi-ci
lc-vr
oc-so
wm-kt
ve-it
bp-av
iz-dm
xt-hn
ph-nt
ss-xk
ig-yl
hv-iq
pz-zu
gg-cs
co-mz
ow-kn
ag-qg
cl-rd
hv-bg
cx-ia
ja-rq
kt-nh
uy-gm
aa-aw
lr-gb
bb-vu
si-kn
oc-jh
qw-bi
tx-ty
wa-iw
vi-do
jg-gr
zw-pw
ft-do
nb-rq
vo-sq
rq-lz
kv-gv
jx-qs
gu-wr
wj-ix
oy-yl
an-dr
lz-df
wd-qy
hx-tp
qb-qo
zg-dk
qs-ah
sg-wk
au-yy
fo-xx
ii-iy
th-io
kd-gk
cl-po
np-vr
az-og
iw-ix
mf-gr
th-ii
gi-rg
cy-pu
ok-fb
px-ot
jr-zm
dr-od
sk-ol
zg-gb
wv-pq
vm-yl
ip-oq
dg-ey
us-po
pm-xl
sf-zw
sz-wo
jm-hp
xx-yd
gt-ot
jb-gh
tf-kp
nb-ga
mj-ue
zs-wn
kx-hc
xu-yw
ns-ih
at-lm
br-at
it-sq
bb-cr
uk-yb
pw-uh
co-sf
xg-gn
fe-uz
rv-gr
zk-vd
nk-vq
mc-dj
ml-ws
po-qm
ve-sq
tc-ku
rp-uk
kh-vl
zo-cq
hr-sy
ta-rg
gf-df
ht-fx
xv-yl
wo-oq
vp-tl
su-ev
gs-nc
qt-cl
ge-xu
ma-kt
cg-ef
sd-lg
lz-nb
xk-rp
tt-jx
yb-yi
gs-uk
ju-ms
ko-ur
vy-hk
hw-yq
wp-yr
sl-zn
ta-to
bg-vq
zq-hk
ve-su
lf-fl
jr-pu
uf-mj
po-at
wy-xn
bm-ie
nr-dg
bi-cg
ao-gx
gz-gn
sl-nn
ew-nl
oy-ma
fk-ta
kp-gv
wy-ii
ju-xd
pj-tu
qp-kz
ey-vs
td-yf
yb-lw
zw-ow
yj-jp
uf-ck
ie-gp
yi-mn
xx-ov
lm-jb
po-rd
bv-kx
ka-as
vi-eb
eb-ib
nx-qs
iz-fq
au-in
or-iq
gk-aw
sj-er
zx-gt
py-oc
kq-ti
pw-ka
jk-xg
lw-il
jq-cf
ed-dk
uq-dj
cz-ch
gy-vl
cs-xh
uf-bx
oz-yq
gy-rw
ry-nm
pq-my
mk-dj
vm-ud
lp-pv
tx-vd
yv-av
tt-ah
jk-cq
og-lu
ak-bj
dv-jq
tu-cf
ny-dv
yf-mi
mo-qv
rl-le
mt-iv
ei-hz
yv-ru
rp-ss
we-mj
pc-pm
gf-dd
kd-tv
le-ft
xl-sw
so-jh
my-tw
ku-kp
ll-re
ma-yl
qi-vx
ao-nn
vz-ep
ou-dg
cj-js
xs-pe
vq-si
zi-pl
yo-cu
am-st
kt-yl
cl-zl
ys-dj
wn-tk
kn-lq
pb-qw
mz-rj
wn-ql
qu-pq
ex-hj
zs-oq
ov-aq
dd-ja
sy-vx
ir-cs
bv-hq
qr-xv
dl-jk
bj-zr
er-js
yy-ld
gr-nz
ns-zi
sj-lo
mt-zn
wp-nn
tf-tc
gy-ea
ms-ov
tv-qf
so-cf
ov-an
ys-wf
yu-ln
fw-pl
yd-xd
ka-zw
cm-ue
cv-zr
tx-zk
lt-dj
gq-pt
kx-ex
cp-uw
bo-xy
in-nu
rs-ey
hc-hj
he-oo
tm-vl
cf-ny
mq-fy
aq-fo
ei-jc
um-tx
we-ts
tm-sh
sl-yo
aa-gk
df-ga
kn-zw
lh-es
yv-ac
fu-yu
tv-wv
to-ys
ug-by
gr-ar
xv-hc
qj-ch
vd-ky
if-si
sl-ko
tg-nh
ku-zx
pn-jx
di-xk
ky-bz
rj-ir
vx-fx
wy-oe
ml-pu
wm-ye
io-oa
sb-qw
mc-ys
jv-we
av-ro
rw-tm
ga-bw
ny-uo
fr-zm
ft-eb
ud-vn
jr-fb
my-yt
lt-mk
ei-er
vo-sb
sw-tb
hx-dv
df-wj
ud-rn
tu-ww
kv-as
bu-pt
zx-px
xq-kr
gj-it
gg-uw
wd-nt
ag-lb
tp-cv
gf-iw
gc-sg
gk-np
vd-ty
ty-ox
yh-hy
nl-nk
rs-pt
fv-hr
pw-ep
vr-aa
wk-af
pq-st
ef-qw
wd-ph
eb-ci
gh-af
ci-do
na-lr
md-kz
dx-fm
iy-dx
mq-ew
zw-kv
zn-iv
js-mb
vx-ht
tw-pq
vj-cg
za-js
nh-ig
ur-ao
ok-sb
bo-jk
qj-zm
hj-pe
ig-kt
sk-us
ld-im
gm-ua
zu-qy
xk-tr
px-kp
qb-pj
hz-cj
pa-um
sa-kz
fj-yl
vs-gq
nk-if
qw-vj
xn-oa
uo-yw
fn-cr
dp-uy
tm-fs
le-ib
bu-gh
he-xo
kh-ea
lf-ew
hj-xv
oq-qs
zk-de
ph-qc
fg-ak
ei-cj
gn-xy
to-lt
ds-dh
op-ea
tx-bz
ua-tr
fr-wz
rq-dd
nx-kn
cm-lh
fx-ot
fb-yo
iu-nf
gr-jh
ju-fo
cm-ts
va-nh
bt-yf
rm-gw
fl-fj
po-sn
ql-zs
tr-ll
st-yt
zg-qg
pm-mu
mj-bx
tc-kp
xs-sf
pr-ng
yd-vj
iz-vr
xl-ht
aw-kq
rw-oy
ul-vd
eh-tr
qg-lr
yq-vb
nh-cw
lm-af
av-ln
yr-ao
pq-mr
vw-fw
tv-ze
pz-qi
to-zc
yz-to
od-ki
hj-bv
kr-pl
fq-vr
vp-fr
hz-er
mt-yr
gv-tc
tt-ct
op-tm
yp-tq
oy-kt
fu-mo
iz-aw
iq-zt
xy-cq
nt-qy
gj-nm
yr-cu
mf-jg
eb-vb
cy-nl
kf-iq
dr-oe
eh-lc
kq-ir
mc-pc
ng-pz
jb-oo
ve-gj
ou-vu
iy-oa
mc-up
wn-nz
av-fu
pd-rv
uy-sw
mg-fk
wa-ix
od-ec
yr-zn
wg-rj
vq-nl
iv-vm
ho-bv
vf-ii
gp-rw
es-sr
im-au
qw-fb
ao-wp
gb-ag
sn-hj
zq-wo
zn-nn
hn-pc
rl-hw
hw-vi
rq-gf
op-sh
gt-ku
ee-ad
fm-fs
fq-qf
gi-td
sy-pm
nr-cw
tr-sw
lp-zj
vm-tg
dd-wj
ab-sj
ep-kn
wk-al
wy-oa
ia-sq
fn-vu
kt-ud
xo-al
jm-bp
um-bz
cq-jm
rd-ck
ru-zc
qi-jz
nf-tt
xg-sy
ba-mi
pr-so
jx-ct
wy-dx
js-sj
ni-vi
pt-df
gu-gw
tl-pu
ys-lt
xt-mu
kd-iz
ft-ci
ql-zq
zj-qq
uz-ab
ox-um
xb-mf
ye-vw
px-se
eb-le
kt-rn
fw-of
kp-qk
jg-jh
su-ry
td-ba
sj-jc
yb-zu
ac-qv
lz-fu
pa-bz
ka-kk
fg-pg
xo-ch
iu-ah
ry-it
di-rp
sy-fv
wl-hp
tj-bv
bb-dg
zx-og
nu-cz
rm-bj
ja-ix
rq-hn
hr-vx
ho-kx
iy-ij
ss-qp
lw-ph
aa-iz
lm-gh
vy-oq
oz-ci
td-bt
pz-hx
jk-hp
vx-hn
lt-yz
iu-zg
dw-sg
bu-gu
ev-cx
jq-qb
ru-no
la-hk
uc-cv
xe-qw
eh-re
ml-vp
up-zc
qb-pm
xl-sy
ph-mn
wv-yg
ti-fl
ui-jl
rh-ed
my-qu
hv-nk
kc-pl
fj-va
fo-ov
pg-ro
bx-vn
ah-wx
sd-ba
uf-sr
uk-qp
vf-io
dx-ii
gn-bo
bo-dh
zc-dj
oe-fh
qb-yw
ms-dr
et-gb
tx-pa
fm-wy
ir-uw
ev-ry
jc-op
rv-ar
az-ku
ee-kx
bw-ja
lc-aa
mt-wp
fv-vx
jq-uo
vb-lo
nk-fy
tr-gm
ve-bh
es-bx
tt-lx
cv-pv
dg-zj
ie-rw
gq-gw
wg-gg
zi-wm
fw-wm
vy-nz
in-im
he-dw
no-lt
qs-mw
ih-fw
xp-lp
pb-yh
jh-mf
it-su
eb-il
zm-iu
ra-vj
xk-md
nf-lx
zt-vk
zk-ox
pl-la
uy-yj
sb-yh
qk-sn
ak-cv
fo-ms
zx-gv
zt-kf
ma-rn
er-jc
nt-zu
kv-ka
ab-zx
ua-ld
tm-fc
am-my
ab-hc
pt-wr
mc-lt
sn-ol
gb-rh
lb-qg
an-fo
sz-vy
um-ky
rv-oc
ta-mg
di-nr
qb-ww
cw-zj
lf-bg
gh-of
la-ih
lm-dw
bh-ia
df-nb
qc-yi
bc-zn
ei-lo
oy-ai
nr-vu
ro-ac
ex-tj
fu-qv
sf-ka
ye-xq
uq-wf
jq-ge
jo-ln
vp-jr
no-mk
uh-dk
pv-xp
ux-rs
rh-dk
sd-qp
qf-gk
xv-tj
rh-bc
oj-gj
ac-av
yv-jo
zk-ul
ad-kx
ft-yq
gw-bu
vp-ws
xv-ho
ey-gq
jx-jd
ou-ay
im-kf
ft-vb
jw-ro
wj-ja
ns-sh
uq-zc
np-aw
gc-dw
sy-hn
si-ti
tj-bz
nn-yo
xf-ye
vm-ai
ky-de
dm-aw
yj-eh
kn-xs
js-uz
pt-zo
yz-uq
su-nm
rm-pz
pm-zf
mr-kg
zg-et
ki-ov
ru-yz
gg-cp
af-xo
ra-hy
ly-fh
ma-va
cy-zm
ta-qo
gk-lc
to-mc
yh-cg
cg-fb
qk-zl
yn-gs
de-jl
yr-sl
ws-zm
wf-cr
ws-nw
pv-ay
fe-ab
dh-xy
sh-rw
dx-vf
js-hz
ot-zx
xx-ju
fe-hz
mz-wg
wg-uw
zr-tp
ow-pe
xf-kr
ny-xu
rl-yq
tv-aa
oc-pr
rv-vu
kq-cp
kr-vw
lg-tl
dt-ge
iy-th
jo-fu
dp-ll
yf-ba
kt-fj
aa-ze
uq-ph
dl-xy
no-mc
dt-dv
ho-ee
qi-ak
sk-br
fx-fv
qu-yt
og-tc
dp-ua
ok-jb
ie-fs
ll-yj
yx-re
oc-jg
nu-kf
ki-aq
dw-gh
vm-va
wn-vy
ab-lo
tt-jd
bi-hy
pj-ww
fs-sh
fy-nl
xx-ec
ys-no
yn-sa
ww-uo
if-vq
lr-dk
uh-et
mt-ao
mc-wf
vl-gp
to-mk
vy-tk
az-tf
ql-oq
dg-lp
sj-ei
we-cm
ur-cu
wr-rs
tp-uh
gr-pd
dm-gk
np-iz
lo-hz
ou-pv
nc-ss
tt-ew
fy-ew
tc-zx
yn-di
xl-fx
aw-tv
xu-dt
fw-la
sn-rd
vw-xf
ii-oa
fj-ai
mw-lx
gz-xy
yy-vz
ay-lp
fl-mq
tk-zs
jr-nw
mk-mc
fs-fc
bh-sq
us-rd
pr-mf
uh-gb
mo-te
ti-ew
sz-zs
ie-vl
lz-ga
nt-lw
gn-qu
ry-sq
le-oz
ok-hy
wl-xg
he-jb
do-ib
vn-lh
pv-cr
aw-ze
gx-ur
iv-ur
gj-bh
bt-mi
pj-pg
in-ld
bb-cw
ii-xn
oo-sg
al-sg
fj-vm
jk-dh
yq-gc
kc-ns
zq-zs
mo-yu
vs-pt
aa-fr
ua-ll
rn-ai
vd-ox
au-cz
na-zg
ni-eb
pl-wm
rm-ng
cj-jc
vw-pl
te-pg
al-he
iv-yo
ih-kr
pj-dv
te-jo
py-xb
vl-sh
qc-zu
ad-cd
za-ka
xb-pr
xq-la
le-vi
oa-fm
gb-bc
oo-gc
bm-rw
hq-ad
nf-uz
or-cz
lz-gf
wf-mk
ul-tx
pw-xs
se-tc
kc-ay
xl-hn
sr-jv
di-nc
mb-hz
ka-kn
ro-te
wj-ga
oe-dx
sd-ta
lf-if
ld-vk
dk-bc
xe-vj
xd-an
ie-tm
zx-lu
nl-mq
gt-tf
dr-yd
bt-gi
wl-gz
pe-kv
me-mz
ld-au
es-cm
bc-ed
hx-cv
px-og
yw-dv
pb-ok
xt-sy
me-ly
tg-ai
by-mr
mr-wv
bi-wx
xn-iy
vi-yq
ch-jr
nu-im
si-lf
xf-lu
er-mb
tt-mw
bw-lz
pc-zf
nk-sj
ix-dd
jv-ts
sy-pc
fv-hn
ye-kc
nl-if
bg-mq
dw-aq
fl-bg
kr-wm
wg-cp
yg-tw
qk-at
au-kf
ko-mz
op-bm
hp-cq
yb-mn
bp-ro
vs-gu
ju-yd
nx-iu
gk-iz
ly-kq
co-wg
jv-bx
hz-sj
fq-dm
ds-hp
gb-na
ro-fu
br-ol
za-er
yt-am
qv-ln
vw-zi
md-yn
tr-uy
qv-bp
dd-wa
ea-sh
uf-lh
hp-dh
zs-nz
dt-ww
at-qt
yo-wp
ky-ul
rh-uh
xu-pj
gh-sg
qv-bt
me-uw
lx-qs
cg-sb
jg-py
fc-vl
vw-ih
ww-cf
lx-ah
qv-pg
in-yy
de-tx
lb-ed
ro-yv
xh-ir
gh-wk
kf-ir
ck-lh
se-ot
jv-mj
df-rq
ct-qs
vz-mb
si-nl
or-ld
sh-fc
qj-jr
oo-of
ws-am
bx-ue
gr-oc
jk-kz
lx-pn
kx-qr
nh-oy
mr-lb
tw-mr
pe-pw
ju-an
tg-rn
dk-ag
wa-rq
od-xd
wl-xy
tl-qj
yi-ht
za-hz
bc-lb
ys-sw
pp-wn
to-wf
th-oe
lg-td
tw-am
yp-ey
tb-yx
cq-xg
jl-ty
cx-su
qk-qm
pp-oq
au-zt
ib-vi
qq-yi
ze-qf
hk-sz
nw-pu
bw-lx
xq-xf
te-fu
fx-xt
pn-ku
og-gt
bj-yr
ex-cd
wa-df
sf-lq
nm-ia
kx-pd
iw-ky
pz-bj
ko-ao
th-vf
fw-if
ln-ro
ln-ac
gk-vr
cg-ok
gv-px
mt-ko
xf-zi
vo-nm
nt-vs
tq-bu
il-wd
xh-kq
ef-ok
hv-lf
jz-rm
yo-mt
sy-fx
ve-cx
jd-nf
yw-dt
ku-ot
xg-hp
hp-gn
pz-ak
wg-ir
fq-kd
he-gh
bv-ex
iw-lz
ti-mq
xr-pd
nk-ti
qw-le
mu-fx
vp-zg
uk-jw
wg-xh
il-ph
et-bc
fq-aw
or-im
ve-wo
vs-wr
mf-py
dl-gn
io-dx
uh-bc
bg-nk
uw-rj
jk-jm
mg-bt
nk-ew
ot-dx
fs-gp
jx-nf
mi-rg
jd-iu
sg-hz
df-ja
fr-ml
xp-fn
hr-xl
rd-br
jg-so
wr-bu
zq-sz
fk-rg
lx-jd
cl-gx
mg-ba
rn-va
zs-wo
xe-cg
ec-gi
hw-ft
ki-dr
ib-ci
ty-de
wp-um
pm-xt
nx-ct
kh-fc
yv-mo
tv-vr
ww-ny
kr-zi
wz-pu
gx-yo
ac-mo
yu-te
fr-mj
hw-eb
gj-ev
vs-zo
rg-yf
cl-br
eb-yq
fc-ew
ih-wm
xp-ay
qy-mn
dm-ze
jp-tb
co-cs
qs-tt
xp-cr
ox-tx
vz-iq
cj-mb
xx-dr
ws-ch
dm-lc
ye-ns
dk-na
oz-rl
ev-oj
ms-ki
ru-to
ed-it
bv-qr
rp-qp
bj-no
pn-qs
ts-uf
ny-dt
ri-yb
rw-vl
lt-ru
dd-lz
tq-gu
dl-gz
aq-yd
qq-yb
ds-jk
th-fh
lr-bc
cv-bj
fs-gy
fk-qo
gq-rs
hc-ex
wn-hk
od-ju
gj-cx
kv-ep
qi-bj
te-yv
jo-yu
lr-ed
ja-jp
mg-yf
pa-ul
nc-xk
vz-or
am-mr
ww-dj
sn-us
ba-lg
vm-kt
by-tw
gc-of
tv-np
wy-vf
ly-gg
rq-ix
in-ye
pg-bp
oo-ze
bz-vd
cy-ws
ih-xq
ox-ui
dr-fo
dw-al
it-ia
qf-oz
ba-bt
dm-kd
ip-zq
qb-dv
aq-xd
pc-fx
gm-dp
xp-dg
zw-ep
pb-xe
xp-bb
xo-gc
cs-xx
jd-ct
zf-hr
na-ed
lr-et
qf-lc
wx-pn
cs-ly
jr-wz
nu-vz
ow-as
xq-fw
ta-lg
nt-qq
gk-ze
ou-zj
as-ep
fc-op
ed-et
yh-ok
gz-hp
br-sn
uz-cj
ip-sz
ll-tb
hy-xe
jm-dh
ko-iv
iu-jx
nr-cr
ts-me
vb-oz
fq-lc
vs-bu
zc-yz
tw-ug
ky-pa
yt-ug
zg-bc
xe-fb
vl-ea
pr-bo
gy-tm
mb-za
gp-sq
ey-tq
qt-po
xy-sr
fg-zr
vd-de
cl-at
es-vn
ug-qu
se-tf
bc-qg
ao-yo
as-zw
yu-qv
si-mq
fg-cv
mi-lg
pw-kn
ga-dd
tu-qb
rh-na
tg-yl
yn-nb
ml-jr
py-xr
sa-xk
qv-yv
xl-fv
vf-xn
bb-lp
gz-jk
up-dj
xq-vw
fk-td
st-jl
io-ii
oj-it
av-qv
kh-rw
bz-ty
uc-qi
bv-al
ty-bm
vp-ch
oz-vi
ie-ea
rv-xb
lh-ts
hp-up
rv-py
ag-zg
ed-ag
ky-ox
cu-mt
nl-hv
aa-qf
dm-aa
to-no
kc-vw
ds-xg
ly-co
dd-wl
so-xr
hw-oz
pv-cw
sh-gp
qf-aw
cj-lo
va-oy
jz-ng
vj-bi
zw-lq
tc-ot
sk-sn
kh-bm
vm-oy
ul-jl
gi-ta
cl-sn
la-xf
cu-wp
ow-ep
rl-eb
al-af
yf-py
ho-tj
zu-yi
vn-zi
sj-cj
uy-tp
qi-fg
dl-xg
fy-lf
ch-ml
ib-ft
gt-gv
my-ug
gn-ds
iq-im
ss-uk
hq-hc
cd-tj
au-vz
yj-ux
bg-nl
mg-rg
xh-rj
wl-bo
yg-am
nh-rn
tw-yt
wv-by
ts-es
gi-lg
ld-nu
an-ah
gs-di
dx-th
fg-hx
ru-mc
ql-pp
ts-ue
hc-ho
lz-ix
kf-in
jq-pj
fx-pm
pe-sf
iy-io
az-zx
xo-dw
kp-gt
qo-mi
kg-am
kt-tg
ao-zn
sz-pp
bo-gz
kf-cz
ix-df
gw-mg
ko-yo
xf-ns
bj-jz
md-jw
xl-vx
ul-ui
ye-la
va-yl
ve-ia
jv-ck
fr-tl
vn-uf
vu-pv
fg-jz
ec-yd
jo-ro
sd-yf
ku-og
fn-ba
qg-et
fs-vl
qm-rd
ru-up
uf-jv
ev-ve
jm-xg
ul-um
yh-st
we-uf
yt-yg
ra-cg
lw-yx
ti-bg
ee-cd
bm-sh
ah-mw
se-gv
ld-kf
sd-fk
xd-yg
fv-mu
wd-ri
ng-zr
fo-xd
qg-na
gw-rs
ir-mz
tv-dm
hv-ew
mj-vn
ja-nb
mw-wx
qo-rg
dl-bo
nx-tt
hq-px
az-ot
ml-wg
mi-mg
wj-wa
ij-fm
qi-rm
oj-su
mg-sd
wx-iu
ys-yz
wm-xf
jp-re
ol-cl
qg-ed
ak-uc
pd-py
it-vo
dl-jm
nb-gf
lf-mq
uq-lt
dt-qb
mb-uz
iq-vk
hk-oq
ef-fb
fn-dg
ig-ud
by-am
kz-di
gj-ry
qc-qq
hy-fb
fl-vq
lm-wk
vd-jl
cm-bx
ln-yj
dr-ju
dl-hp
qc-mn
qm-qt
tk-zq
re-yj
zf-fv
tk-oq
ou-fn
bw-wj
hn-mu
vs-tq
et-ag
sb-hy
js-zq
wf-ru
nx-pn
kk-lq
bm-fs
ki-fo
ud-va
cj-za
nw-tl
zl-br
dl-uo
oj-nm
sw-jp
bw-df
yp-gu
wg-ly
dp-yj
az-gt
op-rw
ug-yg
an-ki
ar-jg
jh-pd
gc-al
uk-xk
bu-ux
fc-ea
ow-wo
zq-oq
fk-gi
vw-wm
nu-yy
oe-xn
ci-ad
fb-sb
ka-ow
na-uh
zu-wd
ot-tf
bg-si
lw-yi
dj-no
ea-rw
wj-iw
cu-wz
ku-lu
if-bg
ko-cu
cy-qj
la-kr
mt-lh
ql-ip
bj-uc
iw-rq
bt-fk
nm-ju
gq-zo
ia-vo
vo-ry
jk-gn
qo-gi
fj-rn
dr-ec
ig-ai
oe-fm
hq-ee
up-to
ed-zg
vs-ux
th-oa
cr-ou
gg-me
nu-vk
vq-vy
us-qt
pq-iy
af-jb
ud-fj
mb-ei
tl-ws
kz-yn
fu-ac
zj-bb
gt-px
ar-xr
ec-an
ig-vm
fu-bp
mr-st
cv-jz
nn-yr
ew-fl
co-gg
zl-rd
zx-kp
hk-nz
kd-ms
bx-ck
so-rv
am-qu
yz-up
hx-rm
yt-mr
xr-jh
hn-hr
xh-kg
sl-my
fv-xt
lm-xo
vn-we
ia-ry
di-jw
fv-pc
th-ij
pu-ws
te-xb
kv-ow
sk-zl
ar-jh
nb-bw
wk-he
qo-mg
vb-vi
bg-yu
xo-oo
pz-zr
ld-zt
qy-yb
tj-ad
ol-rd
jr-fr
wx-jd
vf-fh
wy-io
pe-ka
mu-vx
hv-ti
ip-zs
fh-iy
zf-sy
fh-oa
za-ab
lf-nl
cg-np
yn-ss
cd-kx
zu-qq
cr-ay
di-ss
ph-qq
sd-rg
ly-mz
tm-se
dx-fh
mn-jm
rl-vb
yz-no
pe-ep
qb-cf
ok-ra
tv-gk
na-ag
tb-re
dh-mi
ip-lr
lg-qo
xp-vu
eh-ll
uo-pj
jb-xo
lo-fe
wx-lx
it-ev
ss-kz
jw-qp
qj-fr
fw-ye
ux-gw
sd-gi
wn-zq
pq-yg
pn-ah
gq-yp
qb-ny
uy-ua
ml-qj
ow-sf
lp-cr
vb-hw
fw-xf
zu-ri
mt-gx
sy-ht
ey-pt
zf-ht
jm-ds
zl-js
gc-wk
ng-tp
vj-ef
wp-sl
ni-ft
dr-xd
qq-mn
va-ai
la-zi
bx-ts
wr-lt
mj-cm
ab-hz
iv-yr
wk-dw
rl-vi
co-me
gb-lb
rd-at
sk-qk
wk-xo
ev-wg
ja-wa
pd-mf
kc-xq
qs-iu
sk-at
tf-ku
ud-ma
ah-ct
ox-fe
wa-lz
ww-ge
xy-xg
cx-nm
bb-pv
uy-tb
ku-px
ge-cf
cq-wl
iw-bw
re-sw
tj-hc
vp-zm
by-oj
qk-br
hx-ng
vo-cx
sd-qo
zi-ih
fw-kr
hw-ni
ux-wr
pt-ux
gz-xg
ga-rq
vm-ma
jp-yx
br-qt
tv-lc
cd-ur
ox-pa
pg-yu
fb-vj
ur-sl
sz-nz
wf-dj
ar-pd
jq-xu
fr-pu
gr-xr
ii-fm
ml-tl
ir-gg
kd-lc
tp-qi
fv-ht
vp-qj
td-sd
if-fy
pg-mo
ys-ru
ui-pa
ir-ly
cv-qi
uy-jp
iw-ja
hr-fx
xh-mz
ox-bz
cz-iq
lh-sr
yx-yj
ol-at
xd-ms
ov-dr
yp-zo
kt-ai
ug-am
ay-dg
rs-sa
ck-mj
kr-ye
co-rj
ue-jv
rp-zf
xl-mu
lu-tc
ft-vi
wz-zm
rv-mf
xd-ec
ud-af
ma-nh
nz-oq
eh-jp
og-bg
ko-gx
yv-ln
oy-vi";

    assert_eq!(1467, find_boss(&parse_input(&source)));
    assert_eq!(
        "co,de,ka,ta".to_string(),
        find_max_connections(&parse_input(&source))
    );
}
