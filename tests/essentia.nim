{.passc: gorge("pkg-config --cflags gaia2 essentia").}
{.passl: gorge("pkg-config --libs gaia2 essentia").}

type
  Real* = cfloat
  BufferInfo* {.importcpp: "essentia::streaming::BufferInfo", header: "types.h",
               bycopy.} = object
    size* {.importc: "size".}: cint
    maxContiguousElements* {.importc: "maxContiguousElements".}: cint
  BufferUsageType* {.size: sizeof(cint), importcpp: "essentia::streaming::BufferUsage::BufferUsageType",
                    header: "types.h".} = enum
    forSingleFrames, forMultipleFrames, forAudioStream, forLargeAudioStream
  AudioSample* = Real
  Tuple2*[T] {.importcpp: "essentia::Tuple2<\'0>", header: "types.h", bycopy.} = object
    first* {.importc: "first".}: T
    second* {.importc: "second".}: T
  StereoSample* = Tuple2[Real]
  FreesoundDescriptorSet* {.importcpp: "FreesoundDescriptorSet",
                           header: "FreesoundDescriptorsSet.h", bycopy.} = object of RootObj
  FreesoundLowlevelDescriptors* {.importcpp: "FreesoundLowlevelDescriptors",
                                 header: "FreesoundLowlevelDescriptors.h", bycopy.} = object of FreesoundDescriptorSet

proc constructFreesoundLowlevelDescriptors*(options: var Pool): FreesoundLowlevelDescriptors {.constructor, 
                                                                                               importcpp: "FreesoundLowlevelDescriptors(@)",
                                                                                               header: "FreesoundLowlevelDescriptors.h".}
proc destroyFreesoundLowlevelDescriptors*(this: var FreesoundLowlevelDescriptors) {.
    importcpp: "#.~FreesoundLowlevelDescriptors()",
    header: "FreesoundLowlevelDescriptors.h".}

proc init*() {.importcpp: "essentia::init(@)", header: "essentia.h".}
proc isInitialized*(): bool {.importcpp: "essentia::isInitialized(@)",
                           header: "essentia.h".}
proc shutdown*() {.importcpp: "essentia::shutdown(@)", header: "essentia.h".}

proc constructBufferInfo*(size: cint = 0; contiguous: cint = 0): BufferInfo {.constructor,
    importcpp: "essentia::streaming::BufferInfo(@)", header: "types.h".}

proc left*[T](this: Tuple2[T]): T {.noSideEffect, importcpp: "left", header: "types.h".}
proc right*[T](this: Tuple2[T]): T {.noSideEffect, importcpp: "right",
                                 header: "types.h".}
proc x*[T](this: Tuple2[T]): T {.noSideEffect, importcpp: "x", header: "types.h".}
proc y*[T](this: Tuple2[T]): T {.noSideEffect, importcpp: "y", header: "types.h".}
proc left*[T](this: var Tuple2[T]): var T {.importcpp: "left", header: "types.h".}
proc right*[T](this: var Tuple2[T]): var T {.importcpp: "right", header: "types.h".}
proc x*[T](this: var Tuple2[T]): var T {.importcpp: "x", header: "types.h".}
proc y*[T](this: var Tuple2[T]): var T {.importcpp: "y", header: "types.h".}

# const
#   essentiah = "<essentia/essentia.h>"
# type
#   cstringConstImpl {.importc:"extern const char*".} = cstring
#   constChar* = distinct cstringConstImpl
#   Algorithm {.header: essentiah,
#               importcpp: "essentia::BaseAlgorithm".} = object
#   Version {.importcpp: "essentia::version", header: essentiah.} = constChar
init()
echo(constructBufferInfo(100,100))
echo isInitialized()
shutdown()
