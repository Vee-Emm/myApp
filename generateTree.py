from pymerkle import *
from pymerkle import MerkleTree

tree = MerkleTree('leaf0leaf0leaf0leaf0leaf0leaf0le',
                    'leaf1leaf1leaf1leaf1leaf1leaf1le',
                    'leaf2leaf2leaf2leaf2leaf2leaf2le',
                    'leaf3leaf3leaf3leaf3leaf3leaf3le',
                    security=False)

print(tree)


#
# 
# └─ec01ad34fcdc08a0cd5af5761a8840c8f96a2397504229eff77607a1b3c64528
#      ├──2119cd5b5bbfa027f3f56de6bb1f5537c302bc4c31335a83d7726a5f8974a4af
#      │    ├──96f1ce1008b5c50024edbd0652c0e3b6213d38b8ee55c9b44a34cb95e5d05892
#      │    └──1b44ca99048d6e248d9ea1485b6a6a78aa55e668c0acdfce5ebb09c6c07e38f6
#      └──bb6bddb7d1bb9989b3cac31edb9fb937984028ded7a4538a0d0cc1af093d2b4a
#           ├──271cc86d8d22a063a64b8322b366477171aa90682fbae80d2871d64467fa1e8e
#           └──d27fbdec640e79f86586dcc772da0a5d375383e2b2e0e1f1a5db0d86f3745a8a
