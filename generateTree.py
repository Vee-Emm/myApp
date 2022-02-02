#https://github.com/fmerg/pymerkle/blob/master/USAGE.rst

from pymerkle import *
from pymerkle import MerkleTree

tree = MerkleTree("leaf0","leaf1","leaf2","leaf3")
print(tree)

#
# Example input  : "leaf0","leaf1","leaf2","leaf3"
# 
# Example Output : 
#  └─345f29830744114d98109fdb09d306586e02197549bab266a4d23d1a52579f8a
#      ├──507c3bb6c20bec99a271b837c489809d7c0f42160cbe3781020e732b575382fa
#      │    ├──e6c410a9745b0151d82d1a9f007b81f378a1588c3fb63dc634a2ab001379c3d2
#      │    └──116af79823b7adaaa73481ee191803ceba570272f809decdcdf5340426f1ace9
#      └──56640a8541a5f092e015814d96c2ef4635f406068519fa4e0af06d4ceaa61d6a
#           ├──30415163f9aea87a7f53b3679c4d75318ee1367567efb6b2183c0e875ab02b4e
#           └──f1fbbbe36a7c26642bf89e87d44e785402b9e723cd9b190566ff6a5f8a1de294
