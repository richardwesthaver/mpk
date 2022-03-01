##
##  Copyright (C) 2006-2021  Music Technology Group - Universitat Pompeu Fabra
##
##  This file is part of Essentia
##
##  Essentia is free software: you can redistribute it and/or modify it under
##  the terms of the GNU Affero General Public License as published by the Free
##  Software Foundation (FSF), either version 3 of the License, or (at your
##  option) any later version.
##
##  This program is distributed in the hope that it will be useful, but WITHOUT
##  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
##  FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
##  details.
##
##  You should have received a copy of the Affero GNU General Public License
##  version 3 along with this program.  If not, see http://www.gnu.org/licenses/
##

import
  streamingalgorithm

type
  Copy*[TokenType] {.importcpp: "essentia::streaming::Copy<\'0>", header: "copy.h",
                    bycopy.} = object of Algorithm


proc constructCopy*[TokenType](): Copy[TokenType] {.constructor,
    importcpp: "essentia::streaming::Copy<\'*0>(@)", header: "copy.h".}
proc declareParameters*[TokenType](this: var Copy[TokenType]) {.
    importcpp: "declareParameters", header: "copy.h".}
proc process*[TokenType](this: var Copy[TokenType]): AlgorithmStatus {.
    importcpp: "process", header: "copy.h".}
##  namespace streaming

##  namespace essentia

##
##  Copyright (C) 2006-2021  Music Technology Group - Universitat Pompeu Fabra
##
##  This file is part of Essentia
##
##  Essentia is free software: you can redistribute it and/or modify it under
##  the terms of the GNU Affero General Public License as published by the Free
##  Software Foundation (FSF), either version 3 of the License, or (at your
##  option) any later version.
##
##  This program is distributed in the hope that it will be useful, but WITHOUT
##  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
##  FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
##  details.
##
##  You should have received a copy of the Affero GNU General Public License
##  version 3 along with this program.  If not, see http://www.gnu.org/licenses/
##

import
  ../streamingalgorithm

type
  DevNull*[TokenType] {.importcpp: "essentia::streaming::DevNull<\'0>",
                       header: "devnull.h", bycopy.} = object of Algorithm


proc constructDevNull*[TokenType](): DevNull[TokenType] {.constructor,
    importcpp: "essentia::streaming::DevNull<\'*0>(@)", header: "devnull.h".}
proc declareParameters*[TokenType](this: var DevNull[TokenType]) {.
    importcpp: "declareParameters", header: "devnull.h".}
proc process*[TokenType](this: var DevNull[TokenType]): AlgorithmStatus {.
    importcpp: "process", header: "devnull.h".}
type
  DevNullConnector* {.size: sizeof(cint),
                     importcpp: "essentia::streaming::DevNullConnector",
                     header: "devnull.h".} = enum
    NOWHERE, DEVNULL


## *
##  Connect a source (eg: the output of an algorithm) to a DevNull, so the data
##  the source outputs does not block the whole processing.
##

proc connect*(source: var SourceBase; devnull: DevNullConnector) {.
    importcpp: "essentia::streaming::connect(@)", header: "devnull.h".}
proc `>>`*(source: var SourceBase; devnull: DevNullConnector) {.importcpp: "(# >> #)",
    header: "devnull.h".}
## *
##  Disconnect a source (eg: the output of an algorithm) from a DevNull.
##

proc disconnect*(source: var SourceBase; devnull: DevNullConnector) {.
    importcpp: "essentia::streaming::disconnect(@)", header: "devnull.h".}
##  namespace streaming

##  namespace essentia

##
##  Copyright (C) 2006-2021  Music Technology Group - Universitat Pompeu Fabra
##
##  This file is part of Essentia
##
##  Essentia is free software: you can redistribute it and/or modify it under
##  the terms of the GNU Affero General Public License as published by the Free
##  Software Foundation (FSF), either version 3 of the License, or (at your
##  option) any later version.
##
##  This program is distributed in the hope that it will be useful, but WITHOUT
##  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
##  FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
##  details.
##
##  You should have received a copy of the Affero GNU General Public License
##  version 3 along with this program.  If not, see http://www.gnu.org/licenses/
##

import
  ../streamingalgorithm

type
  DiskWriter*[T] {.importcpp: "essentia::streaming::DiskWriter<\'0>",
                  header: "diskwriter.h", bycopy.} = object of Algorithm


proc constructDiskWriter*[T](filename: string): DiskWriter[T] {.constructor,
    importcpp: "essentia::streaming::DiskWriter<\'*0>(@)", header: "diskwriter.h".}
proc destroyDiskWriter*[T](this: var DiskWriter[T]) {.importcpp: "#.~DiskWriter()",
    header: "diskwriter.h".}
proc declareParameters*[T](this: var DiskWriter[T]) {.
    importcpp: "declareParameters", header: "diskwriter.h".}
proc process*[T](this: var DiskWriter[T]): AlgorithmStatus {.importcpp: "process",
    header: "diskwriter.h".}
##  namespace streaming

##  namespace essentia

##
##  Copyright (C) 2006-2021  Music Technology Group - Universitat Pompeu Fabra
##
##  This file is part of Essentia
##
##  Essentia is free software: you can redistribute it and/or modify it under
##  the terms of the GNU Affero General Public License as published by the Free
##  Software Foundation (FSF), either version 3 of the License, or (at your
##  option) any later version.
##
##  This program is distributed in the hope that it will be useful, but WITHOUT
##  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
##  FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
##  details.
##
##  You should have received a copy of the Affero GNU General Public License
##  version 3 along with this program.  If not, see http://www.gnu.org/licenses/
##

import
  ../streamingalgorithm, ../../streamutil

proc writeBinary*[TokenType](stream: ptr Ostream; value: TokenType) =
  discard

## !!!Ignored construct:  template < > [end of template] void inline write_binary < std :: vector < Real > > ( std :: ostream * _stream , const std :: vector < Real > & value ) { _stream -> write ( ( const char * ) & value [ 0 ] , value . size ( ) * sizeof ( Real ) ) ; } template < typename TokenType , typename StorageType = TokenType > class FileOutput : public Algorithm { protected : Sink < TokenType > _data ; std :: ostream * _stream ; std :: string _filename ; bool _binary ; public : FileOutput ( ) : Algorithm ( ) , _stream ( NULL ) { setName ( FileOutput ) ; declareInput ( _data , 1 , data , the incoming data to be stored in the output file ) ; declareParameters ( ) ; } ~ FileOutput ( ) { if ( _stream != & std :: cout ) delete _stream ; } void declareParameters ( ) { declareParameter ( filename , the name of the output file (use '-' for stdout) ,  , out.txt ) ; declareParameter ( mode , output mode , {text,binary} , text ) ; } void configure ( ) { if ( ! parameter ( filename ) . isConfigured ( ) ) { throw EssentiaException ( FileOutput: please provide the 'filename' parameter ) ; } _filename = parameter ( filename ) . toString ( ) ; if ( _filename ==  ) { throw EssentiaException ( FileOutput: empty filenames are not allowed. ) ; } _binary = ( parameter ( mode ) . toString ( ) == binary ) ; } void createOutputStream ( ) { if ( _filename == - ) { _stream = & std :: cout ; } else { _stream = _binary ? new std :: ofstream ( _filename . c_str ( ) , std :: ofstream :: binary ) : new std :: ofstream ( _filename . c_str ( ) ) ; if ( _stream -> fail ( ) ) { throw EssentiaException ( FileOutput: Could not open file for writing:  , _filename ) ; } } } AlgorithmStatus process ( ) { if ( ! _stream ) { createOutputStream ( ) ; } EXEC_DEBUG ( process() ) ; if ( ! _data . acquire ( 1 ) ) return NO_INPUT ; write ( _data . firstToken ( ) ) ; _data . release ( 1 ) ; return OK ; } void write ( const TokenType & value ) { if ( ! _stream ) throw EssentiaException ( FileOutput: not configured properly ) ; if ( _binary ) { write_binary ( _stream , value ) ; } else { * _stream << value <<
##  ; } } } ;
## Error: token expected: ; but got: <!!!

##  namespace streaming

##  namespace essentia

##
##  Copyright (C) 2006-2021  Music Technology Group - Universitat Pompeu Fabra
##
##  This file is part of Essentia
##
##  Essentia is free software: you can redistribute it and/or modify it under
##  the terms of the GNU Affero General Public License as published by the Free
##  Software Foundation (FSF), either version 3 of the License, or (at your
##  option) any later version.
##
##  This program is distributed in the hope that it will be useful, but WITHOUT
##  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
##  FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
##  details.
##
##  You should have received a copy of the Affero GNU General Public License
##  version 3 along with this program.  If not, see http://www.gnu.org/licenses/
##

import
  ../streamingalgorithm, ../../pool

type
  PoolStorageBase* {.importcpp: "essentia::streaming::PoolStorageBase",
                    header: "poolstorage.h", bycopy.} = object of Algorithm


proc constructPoolStorageBase*(pool: ptr Pool; descriptorName: string;
                              setSingle: bool = false): PoolStorageBase {.
    constructor, importcpp: "essentia::streaming::PoolStorageBase(@)",
    header: "poolstorage.h".}
proc destroyPoolStorageBase*(this: var PoolStorageBase) {.
    importcpp: "#.~PoolStorageBase()", header: "poolstorage.h".}
proc descriptorName*(this: PoolStorageBase): string {.noSideEffect,
    importcpp: "descriptorName", header: "poolstorage.h".}
proc pool*(this: PoolStorageBase): ptr Pool {.noSideEffect, importcpp: "pool",
    header: "poolstorage.h".}
## !!!Ignored construct:  template < typename TokenType , typename StorageType = TokenType > [end of template] class PoolStorage : public PoolStorageBase { protected : Sink < TokenType > _descriptor ; public : PoolStorage ( Pool * pool , const std :: string & descriptorName , bool setSingle = false ) : PoolStorageBase ( pool , descriptorName , setSingle ) { setName ( PoolStorage ) ; declareInput ( _descriptor , 1 , data , the input data ) ; } ~ PoolStorage ( ) { } void declareParameters ( ) { } AlgorithmStatus process ( ) { EXEC_DEBUG ( process(), for desc:  << _descriptorName ) ; int ntokens = std :: min ( _descriptor . available ( ) , _descriptor . buffer ( ) . bufferInfo ( ) . maxContiguousElements ) ; ntokens = std :: max ( ntokens , 1 ) ;  for singleFrames buffer usage, the size of the phantom zone may be zero,
##  thus need to  +1. And we're still on the safe side, see acquireForRead (phantombuffer_impl.cpp) EXEC_DEBUG ( trying to acquire  << ntokens <<  tokens ) ; if ( ! _descriptor . acquire ( ntokens ) ) { return NO_INPUT ; } EXEC_DEBUG ( appending tokens to pool ) ; if ( ntokens > 1 ) { _pool -> append ( _descriptorName , _descriptor . tokens ( ) ) ; } else { addToPool ( ( StorageType ) _descriptor . firstToken ( ) ) ; } EXEC_DEBUG ( releasing ) ; _descriptor . release ( ntokens ) ; return OK ; } template < typename T > void addToPool ( const std :: vector < T > & value ) { if ( _setSingle ) { for ( int i = 0 ; i < ( int ) value . size ( ) ; ++ i ) _pool -> add ( _descriptorName , value [ i ] ) ; } else _pool -> add ( _descriptorName , value ) ; } void addToPool ( const std :: vector < Real > & value ) { if ( _setSingle ) _pool -> set ( _descriptorName , value ) ; else _pool -> add ( _descriptorName , value ) ; } template < typename T > void addToPool ( const T & value ) { if ( _setSingle ) _pool -> set ( _descriptorName , value ) ; else _pool -> add ( _descriptorName , value ) ; } template < typename T > void addToPool ( const TNT :: Array2D < T > & value ) { _pool -> add ( _descriptorName , value ) ;
##       if (_setSingle) {
##       throw EssentiaException("PoolStorage::addToPool, setting Array2D as single value"
##                               " is not supported by Pool.");
##       }
##       else _pool->add(_descriptorName, value);
##  } template < typename T > void addToPool ( const Tensor < T > & value ) { _pool -> add ( _descriptorName , value ) ; } void addToPool ( const StereoSample & value ) { if ( _setSingle ) { throw EssentiaException ( PoolStorage::addToPool, setting StereoSample as single value  is not supported by Pool. ) ; } else { _pool -> add ( _descriptorName , value ) ; } } } ;
## Error: token expected: > [end of template] but got: =!!!

## *
##  Connect a source (eg: the output of an algorithm) to a Pool, and use the given
##  name as an identifier in the Pool.
##

proc connect*(source: var SourceBase; pool: var Pool; descriptorName: string) {.
    importcpp: "essentia::streaming::connect(@)", header: "poolstorage.h".}
type
  PoolConnector* {.importcpp: "essentia::streaming::PoolConnector",
                  header: "poolstorage.h", bycopy.} = object


proc constructPoolConnector*(p: var Pool; descName: string): PoolConnector {.
    constructor, importcpp: "essentia::streaming::PoolConnector(@)",
    header: "poolstorage.h".}
const
  PC* = poolConnector

##  The reason why this function is defined with a const PC& as argument is described here:
##  http://herbsutter.com/2008/01/01/gotw-88-a-candidate-for-the-most-important-const/

proc `>>`*(source: var SourceBase; pc: PoolConnector) {.importcpp: "(# >> #)",
    header: "poolstorage.h".}
## *
##  Connect a source (eg: the output of an algorithm) to a Pool, and use the given
##  name as an identifier in the Pool. Forces the use of the Pool::set method,
##  instead of Pool::add.
##

proc connectSingleValue*(source: var SourceBase; pool: var Pool; descriptorName: string) {.
    importcpp: "essentia::streaming::connectSingleValue(@)",
    header: "poolstorage.h".}
## *
##  Disconnect a source (eg: the output of an algorithm) from a Pool.
##

proc disconnect*(source: var SourceBase; pool: var Pool; descriptorName: string) {.
    importcpp: "essentia::streaming::disconnect(@)", header: "poolstorage.h".}
##  namespace streaming

##  namespace essentia

##
##  Copyright (C) 2006-2021  Music Technology Group - Universitat Pompeu Fabra
##
##  This file is part of Essentia
##
##  Essentia is free software: you can redistribute it and/or modify it under
##  the terms of the GNU Affero General Public License as published by the Free
##  Software Foundation (FSF), either version 3 of the License, or (at your
##  option) any later version.
##
##  This program is distributed in the hope that it will be useful, but WITHOUT
##  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
##  FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
##  details.
##
##  You should have received a copy of the Affero GNU General Public License
##  version 3 along with this program.  If not, see http://www.gnu.org/licenses/
##

import
  ../streamingalgorithm

type
  RingBufferInput* {.importcpp: "essentia::streaming::RingBufferInput::RingBufferImpl",
                    header: "ringbufferinput.h", bycopy.} = object of Algorithm


## !!!Ignored construct:  class RingBufferImpl * _impl ;
## Error: token expected: ; but got: *!!!

## !!!Ignored construct:  public : RingBufferInput ( ) ;
## Error: identifier expected, but got: )!!!

## !!!Ignored construct:  ~ RingBufferInput ( ) ;
## Error: invalid destructor!!!

proc add*(this: var RingBufferInputRingBufferImpl; inputData: ptr Real; size: cint) {.
    importcpp: "add", header: "ringbufferinput.h".}
proc process*(this: var RingBufferInputRingBufferImpl): AlgorithmStatus {.
    importcpp: "process", header: "ringbufferinput.h".}
proc shouldStop*(this: var RingBufferInputRingBufferImpl; stop: bool) {.
    importcpp: "shouldStop", header: "ringbufferinput.h".}
proc declareParameters*(this: var RingBufferInputRingBufferImpl) {.
    importcpp: "declareParameters", header: "ringbufferinput.h".}
proc configure*(this: var RingBufferInputRingBufferImpl) {.importcpp: "configure",
    header: "ringbufferinput.h".}
proc reset*(this: var RingBufferInputRingBufferImpl) {.importcpp: "reset",
    header: "ringbufferinput.h".}
##  namespace streaming

##  namespace essentia

##
##  Copyright (C) 2006-2021  Music Technology Group - Universitat Pompeu Fabra
##
##  This file is part of Essentia
##
##  Essentia is free software: you can redistribute it and/or modify it under
##  the terms of the GNU Affero General Public License as published by the Free
##  Software Foundation (FSF), either version 3 of the License, or (at your
##  option) any later version.
##
##  This program is distributed in the hope that it will be useful, but WITHOUT
##  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
##  FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
##  details.
##
##  You should have received a copy of the Affero GNU General Public License
##  version 3 along with this program.  If not, see http://www.gnu.org/licenses/
##

import
  ../streamingalgorithm

type
  RingBufferOutput* {.importcpp: "essentia::streaming::RingBufferOutput::RingBufferImpl",
                     header: "ringbufferoutput.h", bycopy.} = object of Algorithm


## !!!Ignored construct:  class RingBufferImpl * _impl ;
## Error: token expected: ; but got: *!!!

## !!!Ignored construct:  public : RingBufferOutput ( ) ;
## Error: identifier expected, but got: )!!!

## !!!Ignored construct:  ~ RingBufferOutput ( ) ;
## Error: invalid destructor!!!

proc get*(this: var RingBufferOutputRingBufferImpl; outputData: ptr Real; max: cint): cint {.
    importcpp: "get", header: "ringbufferoutput.h".}
proc process*(this: var RingBufferOutputRingBufferImpl): AlgorithmStatus {.
    importcpp: "process", header: "ringbufferoutput.h".}
proc declareParameters*(this: var RingBufferOutputRingBufferImpl) {.
    importcpp: "declareParameters", header: "ringbufferoutput.h".}
proc configure*(this: var RingBufferOutputRingBufferImpl) {.importcpp: "configure",
    header: "ringbufferoutput.h".}
proc reset*(this: var RingBufferOutputRingBufferImpl) {.importcpp: "reset",
    header: "ringbufferoutput.h".}
##  namespace streaming

##  namespace essentia

##
##  Copyright (C) 2006-2021  Music Technology Group - Universitat Pompeu Fabra
##
##  This file is part of Essentia
##
##  Essentia is free software: you can redistribute it and/or modify it under
##  the terms of the GNU Affero General Public License as published by the Free
##  Software Foundation (FSF), either version 3 of the License, or (at your
##  option) any later version.
##
##  This program is distributed in the hope that it will be useful, but WITHOUT
##  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
##  FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
##  details.
##
##  You should have received a copy of the Affero GNU General Public License
##  version 3 along with this program.  If not, see http://www.gnu.org/licenses/
##

import
  ../streamingalgorithm

type
  RingBufferVectorOutput* {.importcpp: "essentia::streaming::RingBufferVectorOutput::RingBufferImpl",
                           header: "ringbuffervectoroutput.h", bycopy.} = object of Algorithm


## !!!Ignored construct:  class RingBufferImpl * _impl ;
## Error: token expected: ; but got: *!!!

## !!!Ignored construct:  public : RingBufferVectorOutput ( ) ;
## Error: identifier expected, but got: )!!!

## !!!Ignored construct:  ~ RingBufferVectorOutput ( ) ;
## Error: invalid destructor!!!

proc get*(this: var RingBufferVectorOutputRingBufferImpl; outputData: ptr Real;
         max: cint): cint {.importcpp: "get", header: "ringbuffervectoroutput.h".}
proc process*(this: var RingBufferVectorOutputRingBufferImpl): AlgorithmStatus {.
    importcpp: "process", header: "ringbuffervectoroutput.h".}
proc declareParameters*(this: var RingBufferVectorOutputRingBufferImpl) {.
    importcpp: "declareParameters", header: "ringbuffervectoroutput.h".}
proc configure*(this: var RingBufferVectorOutputRingBufferImpl) {.
    importcpp: "configure", header: "ringbuffervectoroutput.h".}
proc reset*(this: var RingBufferVectorOutputRingBufferImpl) {.importcpp: "reset",
    header: "ringbuffervectoroutput.h".}
##  namespace streaming

##  namespace essentia

##
##  Copyright (C) 2006-2021  Music Technology Group - Universitat Pompeu Fabra
##
##  This file is part of Essentia
##
##  Essentia is free software: you can redistribute it and/or modify it under
##  the terms of the GNU Affero General Public License as published by the Free
##  Software Foundation (FSF), either version 3 of the License, or (at your
##  option) any later version.
##
##  This program is distributed in the hope that it will be useful, but WITHOUT
##  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
##  FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
##  details.
##
##  You should have received a copy of the Affero GNU General Public License
##  version 3 along with this program.  If not, see http://www.gnu.org/licenses/
##

import
  ../streamingalgorithm

## !!!Ignored construct:  template < typename TokenType , int acquireSize = 1 > [end of template] class VectorInput : public Algorithm { protected : Source < TokenType > _output ; const std :: vector < TokenType > * _inputVector ; bool _ownVector ; int _idx ; int _acquireSize ; public : VectorInput ( const std :: vector < TokenType > * input = 0 , bool own = false ) : _inputVector ( input ) , _ownVector ( own ) { setName ( VectorInput ) ; setAcquireSize ( acquireSize ) ; declareOutput ( _output , _acquireSize , data , the values read from the vector ) ; reset ( ) ; } VectorInput ( std :: vector < TokenType > * input , bool own = false ) : _inputVector ( input ) , _ownVector ( own ) { setName ( VectorInput ) ; setAcquireSize ( acquireSize ) ; declareOutput ( _output , _acquireSize , data , the values read from the vector ) ; reset ( ) ; } template < typename Array > VectorInput ( const Array & inputArray , bool own = true ) { setName ( VectorInput ) ; _inputVector = new std :: vector < TokenType > ( arrayToVector < TokenType > ( inputArray ) ) ; _ownVector = true ; setAcquireSize ( acquireSize ) ; declareOutput ( _output , _acquireSize , data , the values read from the vector ) ; reset ( ) ; }  TODO: This constructor takes in an Array2D but it converts it to a
##  vector-vector to work with the existing code. Ideally, we would keep the
##  Array2D (don't forget to turn off _ownVector) and read from it directly. VectorInput ( const TNT :: Array2D < Real > & input ) { setName ( VectorInput ) ;  convert TNT array to vector-vector std :: vector < TokenType > * inputVector = new std :: vector < TokenType > ( ) ; inputVector -> resize ( input . dim1 ( ) ) ; for ( int i = 0 ; i < input . dim1 ( ) ; ++ i ) { ( * inputVector ) [ i ] . resize ( input . dim2 ( ) ) ; for ( int j = 0 ; j < input . dim2 ( ) ; ++ j ) { ( * inputVector ) [ i ] [ j ] = input [ i ] [ j ] ; } } _inputVector = inputVector ; _ownVector = true ; setAcquireSize ( acquireSize ) ; declareOutput ( _output , _acquireSize , data , the values read from the vector ) ; reset ( ) ; } ~ VectorInput ( ) { clear ( ) ; } void clear ( ) { if ( _ownVector ) delete _inputVector ; _inputVector = 0 ; } *
##  TODO: Should we make a copy of the vector here or only keep the ref?
##  void setVector ( const std :: vector < TokenType > * input , bool own = false ) { clear ( ) ; _inputVector = input ; _ownVector = own ; } void setAcquireSize ( const int size ) { _acquireSize = size ; _output . setAcquireSize ( _acquireSize ) ; _output . setReleaseSize ( _acquireSize ) ; } void reset ( ) { Algorithm :: reset ( ) ; _idx = 0 ; _output . setAcquireSize ( _acquireSize ) ; _output . setReleaseSize ( _acquireSize ) ; } bool shouldStop ( ) const { return _idx >= ( int ) _inputVector -> size ( ) ; } AlgorithmStatus process ( ) {  no more data available in vector. shouldn't be necessary to check,
##  but it doesn't cost us anything to be sure EXEC_DEBUG ( process() ) ; if ( shouldStop ( ) ) { return PASS ; }  if we're at the end of the vector, just acquire the necessary amount of
##  tokens on the output source if ( _idx + _output . acquireSize ( ) > ( int ) _inputVector -> size ( ) ) { int howmuch = ( int ) _inputVector -> size ( ) - _idx ; _output . setAcquireSize ( howmuch ) ; _output . setReleaseSize ( howmuch ) ; } EXEC_DEBUG ( acquiring  << _output . acquireSize ( ) <<  tokens ) ; AlgorithmStatus status = acquireData ( ) ; if ( status != OK ) { if ( status == NO_OUTPUT ) { throw EssentiaException ( VectorInput: internal error: output buffer full ) ; }  should never get there, right? return NO_INPUT ; } TokenType * dest = ( TokenType * ) _output . getFirstToken ( ) ; const TokenType * src = & ( ( * _inputVector ) [ _idx ] ) ; int howmuch = _output . acquireSize ( ) ; fastcopy ( dest , src , howmuch ) ; _idx += howmuch ; releaseData ( ) ; EXEC_DEBUG ( released  << _output . releaseSize ( ) <<  tokens ) ; return OK ; } void declareParameters ( ) { } } ;
## Error: token expected: > [end of template] but got: =!!!

proc connect*[T](v: var VectorInput[T]; sink: var SinkBase) =
  discard

proc `>>`*(v: var VectorInput[T]; sink: var SinkBase) {.importcpp: "(# >> #)",
    header: "vectorinput.h".}
##  TODO: in order to use this function runGenerator should be able to be called
##  with a vector

proc connect*[T](v: var Vector[T]; sink: var SinkBase) =
  discard

proc `>>`*(v: var Vector[T]; sink: var SinkBase) {.importcpp: "(# >> #)",
    header: "vectorinput.h".}
##  namespace streaming

##  namespace essentia

##
##  Copyright (C) 2006-2021  Music Technology Group - Universitat Pompeu Fabra
##
##  This file is part of Essentia
##
##  Essentia is free software: you can redistribute it and/or modify it under
##  the terms of the GNU Affero General Public License as published by the Free
##  Software Foundation (FSF), either version 3 of the License, or (at your
##  option) any later version.
##
##  This program is distributed in the hope that it will be useful, but WITHOUT
##  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
##  FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
##  details.
##
##  You should have received a copy of the Affero GNU General Public License
##  version 3 along with this program.  If not, see http://www.gnu.org/licenses/
##

import
  ../streamingalgorithm

## *
##  VectorOutput class that pushes all data coming at its input into a std::vector.
##  Note that you don't need to configure the VectorOutput to an optimized acquireSize,
##  as it will figure out by itself what's the maximum number of tokens it can acquire
##  at once, and this in a smart dynamic way.
##

## !!!Ignored construct:  template < typename TokenType , typename StorageType = TokenType > [end of template] class VectorOutput : public Algorithm { protected : Sink < TokenType > _data ; std :: vector < TokenType > * _v ; public : VectorOutput ( std :: vector < TokenType > * v = 0 ) : Algorithm ( ) , _v ( v ) { setName ( VectorOutput ) ; declareInput ( _data , 1 , data , the input data ) ; } ~ VectorOutput ( ) { } void declareParameters ( ) { } void setVector ( std :: vector < TokenType > * v ) { _v = v ; } AlgorithmStatus process ( ) { if ( ! _v ) { throw EssentiaException ( VectorOutput algorithm has no output vector set... ) ; } EXEC_DEBUG ( process() ) ; int ntokens = std :: min ( _data . available ( ) , _data . buffer ( ) . bufferInfo ( ) . maxContiguousElements ) ; ntokens = std :: max ( 1 , ntokens ) ; EXEC_DEBUG ( acquiring  << ntokens <<  tokens ) ; if ( ! _data . acquire ( ntokens ) ) { return NO_INPUT ; }  copy tokens in the vector int curSize = _v -> size ( ) ; _v -> resize ( curSize + ntokens ) ; TokenType * dest = & _v -> front ( ) + curSize ; const TokenType * src = & _data . firstToken ( ) ; fastcopy ( dest , src , ntokens ) ; _data . release ( ntokens ) ; return OK ; } void reset ( ) { _acquireSize = acquireSize; } } ;
## Error: token expected: > [end of template] but got: =!!!

proc connect*[TokenType; StorageType](source: var SourceBase;
                                    v: var VectorOutput[TokenType, StorageType]) =
  discard

proc `>>`*(source: var SourceBase; v: var VectorOutput[TokenType, StorageType]) {.
    importcpp: "(# >> #)", header: "vectoroutput.h".}
## *
##  Connect a source (eg: the output of an algorithm) to a vector that will
##  serve as storage.
##

proc connect*[T](source: var SourceBase; v: var Vector[T]) =
  discard

proc `>>`*(source: var SourceBase; v: var Vector[T]) {.importcpp: "(# >> #)",
    header: "vectoroutput.h".}
##  namespace streaming

##  namespace essentia

##
##  Copyright (C) 2006-2021  Music Technology Group - Universitat Pompeu Fabra
##
##  This file is part of Essentia
##
##  Essentia is free software: you can redistribute it and/or modify it under
##  the terms of the GNU Affero General Public License as published by the Free
##  Software Foundation (FSF), either version 3 of the License, or (at your
##  option) any later version.
##
##  This program is distributed in the hope that it will be useful, but WITHOUT
##  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
##  FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
##  details.
##
##  You should have received a copy of the Affero GNU General Public License
##  version 3 along with this program.  If not, see http://www.gnu.org/licenses/
##

import
  essentia/streaming/sourcebase, essentia/pool, essentia/types, essentia/algorithm,
  essentia/scheduler/network, essentia/streaming/streamingalgorithm,
  essentia/algorithmfactory, essentia/streaming/algorithms/poolstorage,
  essentia/streaming/algorithms/vectorinput

## using statement

## using statement

## using statement

type
  FreesoundDescriptorSet* {.importcpp: "FreesoundDescriptorSet",
                           header: "FreesoundDescriptorsSet.h", bycopy.} = object


##
##  Copyright (C) 2006-2021  Music Technology Group - Universitat Pompeu Fabra
##
##  This file is part of Essentia
##
##  Essentia is free software: you can redistribute it and/or modify it under
##  the terms of the GNU Affero General Public License as published by the Free
##  Software Foundation (FSF), either version 3 of the License, or (at your
##  option) any later version.
##
##  This program is distributed in the hope that it will be useful, but WITHOUT
##  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
##  FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
##  details.
##
##  You should have received a copy of the Affero GNU General Public License
##  version 3 along with this program.  If not, see http://www.gnu.org/licenses/
##

import
  freesoundDescriptorsSet, essentia/essentiamath

## using statement

type
  FreesoundLowlevelDescriptors* {.importcpp: "FreesoundLowlevelDescriptors",
                                 header: "FreesoundLowlevelDescriptors.h", bycopy.} = object of FreesoundDescriptorSet


proc constructFreesoundLowlevelDescriptors*(options: var Pool): FreesoundLowlevelDescriptors {.
    constructor, importcpp: "FreesoundLowlevelDescriptors(@)",
    header: "FreesoundLowlevelDescriptors.h".}
proc destroyFreesoundLowlevelDescriptors*(this: var FreesoundLowlevelDescriptors) {.
    importcpp: "#.~FreesoundLowlevelDescriptors()",
    header: "FreesoundLowlevelDescriptors.h".}
proc createNetwork*(this: var FreesoundLowlevelDescriptors; source: var SourceBase;
                   pool: var Pool) {.importcpp: "createNetwork",
                                  header: "FreesoundLowlevelDescriptors.h".}
proc computeAverageLoudness*(this: var FreesoundLowlevelDescriptors; pool: var Pool) {.
    importcpp: "computeAverageLoudness", header: "FreesoundLowlevelDescriptors.h".}
##
##  Copyright (C) 2006-2021  Music Technology Group - Universitat Pompeu Fabra
##
##  This file is part of Essentia
##
##  Essentia is free software: you can redistribute it and/or modify it under
##  the terms of the GNU Affero General Public License as published by the Free
##  Software Foundation (FSF), either version 3 of the License, or (at your
##  option) any later version.
##
##  This program is distributed in the hope that it will be useful, but WITHOUT
##  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
##  FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
##  details.
##
##  You should have received a copy of the Affero GNU General Public License
##  version 3 along with this program.  If not, see http://www.gnu.org/licenses/
##

import
  freesoundDescriptorsSet

## using statement

## using statement

## using statement

type
  FreesoundRhythmDescriptors* {.importcpp: "FreesoundRhythmDescriptors",
                               header: "FreesoundRhythmDescriptors.h", bycopy.} = object of FreesoundDescriptorSet


proc constructFreesoundRhythmDescriptors*(options: var Pool): FreesoundRhythmDescriptors {.
    constructor, importcpp: "FreesoundRhythmDescriptors(@)",
    header: "FreesoundRhythmDescriptors.h".}
proc destroyFreesoundRhythmDescriptors*(this: var FreesoundRhythmDescriptors) {.
    importcpp: "#.~FreesoundRhythmDescriptors()",
    header: "FreesoundRhythmDescriptors.h".}
proc createNetwork*(this: var FreesoundRhythmDescriptors; source: var SourceBase;
                   pool: var Pool) {.importcpp: "createNetwork",
                                  header: "FreesoundRhythmDescriptors.h".}
proc createNetworkBeatsLoudness*(this: var FreesoundRhythmDescriptors;
                                source: var SourceBase; pool: var Pool) {.
    importcpp: "createNetworkBeatsLoudness",
    header: "FreesoundRhythmDescriptors.h".}
##
##  Copyright (C) 2006-2021  Music Technology Group - Universitat Pompeu Fabra
##
##  This file is part of Essentia
##
##  Essentia is free software: you can redistribute it and/or modify it under
##  the terms of the GNU Affero General Public License as published by the Free
##  Software Foundation (FSF), either version 3 of the License, or (at your
##  option) any later version.
##
##  This program is distributed in the hope that it will be useful, but WITHOUT
##  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
##  FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
##  details.
##
##  You should have received a copy of the Affero GNU General Public License
##  version 3 along with this program.  If not, see http://www.gnu.org/licenses/
##

import
  freesoundDescriptorsSet

type
  FreesoundSfxDescriptors* {.importcpp: "FreesoundSfxDescriptors",
                            header: "FreesoundSfxDescriptors.h", bycopy.} = object of FreesoundDescriptorSet


proc constructFreesoundSfxDescriptors*(options: var Pool): FreesoundSfxDescriptors {.
    constructor, importcpp: "FreesoundSfxDescriptors(@)",
    header: "FreesoundSfxDescriptors.h".}
proc destroyFreesoundSfxDescriptors*(this: var FreesoundSfxDescriptors) {.
    importcpp: "#.~FreesoundSfxDescriptors()", header: "FreesoundSfxDescriptors.h".}
proc createNetwork*(this: var FreesoundSfxDescriptors; source: var SourceBase;
                   pool: var Pool) {.importcpp: "createNetwork",
                                  header: "FreesoundSfxDescriptors.h".}
proc createPitchNetwork*(this: var FreesoundSfxDescriptors;
                        pitch: var VectorInput[Real]; pool: var Pool) {.
    importcpp: "createPitchNetwork", header: "FreesoundSfxDescriptors.h".}
proc createHarmonicityNetwork*(this: var FreesoundSfxDescriptors;
                              source: var SourceBase; pool: var Pool) {.
    importcpp: "createHarmonicityNetwork", header: "FreesoundSfxDescriptors.h".}
##
##  Copyright (C) 2006-2021  Music Technology Group - Universitat Pompeu Fabra
##
##  This file is part of Essentia
##
##  Essentia is free software: you can redistribute it and/or modify it under
##  the terms of the GNU Affero General Public License as published by the Free
##  Software Foundation (FSF), either version 3 of the License, or (at your
##  option) any later version.
##
##  This program is distributed in the hope that it will be useful, but WITHOUT
##  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
##  FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
##  details.
##
##  You should have received a copy of the Affero GNU General Public License
##  version 3 along with this program.  If not, see http://www.gnu.org/licenses/
##

import
  freesoundDescriptorsSet

type
  FreesoundTonalDescriptors* {.importcpp: "FreesoundTonalDescriptors",
                              header: "FreesoundTonalDescriptors.h", bycopy.} = object of FreesoundDescriptorSet


proc constructFreesoundTonalDescriptors*(options: var Pool): FreesoundTonalDescriptors {.
    constructor, importcpp: "FreesoundTonalDescriptors(@)",
    header: "FreesoundTonalDescriptors.h".}
proc destroyFreesoundTonalDescriptors*(this: var FreesoundTonalDescriptors) {.
    importcpp: "#.~FreesoundTonalDescriptors()",
    header: "FreesoundTonalDescriptors.h".}
proc createNetwork*(this: var FreesoundTonalDescriptors; source: var SourceBase;
                   pool: var Pool) {.importcpp: "createNetwork",
                                  header: "FreesoundTonalDescriptors.h".}
const
  FREESOUND_EXTRACTOR_VERSION* = "freesound 0.5"

##
##  Copyright (C) 2006-2021  Music Technology Group - Universitat Pompeu Fabra
##
##  This file is part of Essentia
##
##  Essentia is free software: you can redistribute it and/or modify it under
##  the terms of the GNU Affero General Public License as published by the Free
##  Software Foundation (FSF), either version 3 of the License, or (at your
##  option) any later version.
##
##  This program is distributed in the hope that it will be useful, but WITHOUT
##  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
##  FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
##  details.
##
##  You should have received a copy of the Affero GNU General Public License
##  version 3 along with this program.  If not, see http://www.gnu.org/licenses/
##

import
  essentia/streaming/sourcebase, essentia/pool, essentia/types,
  essentia/essentiamath, essentia/algorithm, essentia/scheduler/network,
  essentia/streaming/streamingalgorithm, essentia/algorithmfactory,
  essentia/streaming/algorithms/poolstorage,
  essentia/streaming/algorithms/vectorinput

## using statement

## using statement

## using statement

type
  MusicDescriptorSet* {.importcpp: "MusicDescriptorSet",
                       header: "MusicDescriptorsSet.h", bycopy.} = object


##
##  Copyright (C) 2006-2021  Music Technology Group - Universitat Pompeu Fabra
##
##  This file is part of Essentia
##
##  Essentia is free software: you can redistribute it and/or modify it under
##  the terms of the GNU Affero General Public License as published by the Free
##  Software Foundation (FSF), either version 3 of the License, or (at your
##  option) any later version.
##
##  This program is distributed in the hope that it will be useful, but WITHOUT
##  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
##  FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
##  details.
##
##  You should have received a copy of the Affero GNU General Public License
##  version 3 along with this program.  If not, see http://www.gnu.org/licenses/
##

import
  musicDescriptorsSet, essentia/essentiamath

## using statement

type
  MusicLowlevelDescriptors* {.importcpp: "MusicLowlevelDescriptors",
                             header: "MusicLowlevelDescriptors.h", bycopy.} = object of MusicDescriptorSet


proc constructMusicLowlevelDescriptors*(options: var Pool): MusicLowlevelDescriptors {.
    constructor, importcpp: "MusicLowlevelDescriptors(@)",
    header: "MusicLowlevelDescriptors.h".}
proc destroyMusicLowlevelDescriptors*(this: var MusicLowlevelDescriptors) {.
    importcpp: "#.~MusicLowlevelDescriptors()",
    header: "MusicLowlevelDescriptors.h".}
proc createNetworkNeqLoud*(this: var MusicLowlevelDescriptors;
                          source: var SourceBase; pool: var Pool) {.
    importcpp: "createNetworkNeqLoud", header: "MusicLowlevelDescriptors.h".}
proc createNetworkEqLoud*(this: var MusicLowlevelDescriptors;
                         source: var SourceBase; pool: var Pool) {.
    importcpp: "createNetworkEqLoud", header: "MusicLowlevelDescriptors.h".}
proc createNetworkLoudness*(this: var MusicLowlevelDescriptors;
                           source: var SourceBase; pool: var Pool) {.
    importcpp: "createNetworkLoudness", header: "MusicLowlevelDescriptors.h".}
proc computeAverageLoudness*(this: var MusicLowlevelDescriptors; pool: var Pool) {.
    importcpp: "computeAverageLoudness", header: "MusicLowlevelDescriptors.h".}
##
##  Copyright (C) 2006-2021  Music Technology Group - Universitat Pompeu Fabra
##
##  This file is part of Essentia
##
##  Essentia is free software: you can redistribute it and/or modify it under
##  the terms of the GNU Affero General Public License as published by the Free
##  Software Foundation (FSF), either version 3 of the License, or (at your
##  option) any later version.
##
##  This program is distributed in the hope that it will be useful, but WITHOUT
##  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
##  FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
##  details.
##
##  You should have received a copy of the Affero GNU General Public License
##  version 3 along with this program.  If not, see http://www.gnu.org/licenses/
##

import
  musicDescriptorsSet

## using statement

## using statement

## using statement

type
  MusicRhythmDescriptors* {.importcpp: "MusicRhythmDescriptors",
                           header: "MusicRhythmDescriptors.h", bycopy.} = object of MusicDescriptorSet


proc constructMusicRhythmDescriptors*(options: var Pool): MusicRhythmDescriptors {.
    constructor, importcpp: "MusicRhythmDescriptors(@)",
    header: "MusicRhythmDescriptors.h".}
proc destroyMusicRhythmDescriptors*(this: var MusicRhythmDescriptors) {.
    importcpp: "#.~MusicRhythmDescriptors()", header: "MusicRhythmDescriptors.h".}
proc createNetwork*(this: var MusicRhythmDescriptors; source: var SourceBase;
                   pool: var Pool) {.importcpp: "createNetwork",
                                  header: "MusicRhythmDescriptors.h".}
proc createNetworkBeatsLoudness*(this: var MusicRhythmDescriptors;
                                source: var SourceBase; pool: var Pool) {.
    importcpp: "createNetworkBeatsLoudness", header: "MusicRhythmDescriptors.h".}
##
##  Copyright (C) 2006-2021  Music Technology Group - Universitat Pompeu Fabra
##
##  This file is part of Essentia
##
##  Essentia is free software: you can redistribute it and/or modify it under
##  the terms of the GNU Affero General Public License as published by the Free
##  Software Foundation (FSF), either version 3 of the License, or (at your
##  option) any later version.
##
##  This program is distributed in the hope that it will be useful, but WITHOUT
##  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
##  FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
##  details.
##
##  You should have received a copy of the Affero GNU General Public License
##  version 3 along with this program.  If not, see http://www.gnu.org/licenses/
##

import
  musicDescriptorsSet

type
  MusicTonalDescriptors* {.importcpp: "MusicTonalDescriptors",
                          header: "MusicTonalDescriptors.h", bycopy.} = object of MusicDescriptorSet


proc constructMusicTonalDescriptors*(options: var Pool): MusicTonalDescriptors {.
    constructor, importcpp: "MusicTonalDescriptors(@)",
    header: "MusicTonalDescriptors.h".}
proc destroyMusicTonalDescriptors*(this: var MusicTonalDescriptors) {.
    importcpp: "#.~MusicTonalDescriptors()", header: "MusicTonalDescriptors.h".}
proc createNetworkTuningFrequency*(this: var MusicTonalDescriptors;
                                  source: var SourceBase; pool: var Pool) {.
    importcpp: "createNetworkTuningFrequency", header: "MusicTonalDescriptors.h".}
proc createNetwork*(this: var MusicTonalDescriptors; source: var SourceBase;
                   pool: var Pool) {.importcpp: "createNetwork",
                                  header: "MusicTonalDescriptors.h".}
proc computeTuningSystemFeatures*(this: var MusicTonalDescriptors; pool: var Pool) {.
    importcpp: "computeTuningSystemFeatures", header: "MusicTonalDescriptors.h".}
const
  MUSIC_EXTRACTOR_VERSION* = "music 2.0"
  MUSIC_EXTRACTOR_HL_VERSION* = "music_highlevel 1.0"

##  This whitelist is a combination of the taglib conversion matrices
##  and the picard tag matrix:
##  https://github.com/taglib/taglib/blob/072851869a2980dab4aefd471ad7dd707993d74f/taglib/toolkit/tpropertymap.h
##  https://github.com/taglib/taglib/blob/072851869a2980dab4aefd471ad7dd707993d74f/taglib/asf/asftag.cpp
##  https://github.com/taglib/taglib/blob/072851869a2980dab4aefd471ad7dd707993d74f/taglib/mp4/mp4tag.cpp
##  https://github.com/taglib/taglib/blob/072851869a2980dab4aefd471ad7dd707993d74f/taglib/mpeg/id3v2/id3v2frame.cpp
##  http://picard.musicbrainz.org/docs/mappings/

var tagWhitelist* {.importcpp: "tagWhitelist", header: "tagwhitelist.h".}: UncheckedArray[
    cstring]

import
  math

##  needed for sqrt() below.

## using statement

## *
##    <P>
##    For a symmetric, positive definite matrix A, this function
##    computes the Cholesky factorization, i.e. it computes a lower
##    triangular matrix L such that A = L*L'.
##    If the matrix is not symmetric or positive definite, the function
##    computes only a partial decomposition.  This can be tested with
##    the is_spd() flag.
##
##    <p>Typical usage looks like:
##    <pre>
## 	Array2D<double> A(n,n);
## 	Array2D<double> L;
##
## 	 ...
##
## 	Cholesky<double> chol(A);
##
## 	if (chol.is_spd())
## 		L = chol.getL();
## 		
##   	else
## 		cout << "factorization was not complete.\n";
##
## 	</pre>
##
##
##    <p>
## 	(Adapted from JAMA, a Java Matrix Library, developed by jointly
## 	by the Mathworks and NIST; see  http://math.nist.gov/javanumerics/jama).
##
##

type
  Cholesky*[Real] {.importcpp: "JAMA::Cholesky<\'0>", header: "jama_cholesky.h",
                   bycopy.} = object
    ##  lower triangular factor
    ##  1 if matrix to be factored was SPD


proc constructCholesky*[Real](): Cholesky[Real] {.constructor,
    importcpp: "JAMA::Cholesky<\'*0>(@)", header: "jama_cholesky.h".}
proc constructCholesky*[Real](a: Array2D[Real]): Cholesky[Real] {.constructor,
    importcpp: "JAMA::Cholesky<\'*0>(@)", header: "jama_cholesky.h".}
proc getL*[Real](this: Cholesky[Real]): Array2D[Real] {.noSideEffect,
    importcpp: "getL", header: "jama_cholesky.h".}
proc solve*[Real](this: var Cholesky[Real]; b: Array1D[Real]): Array1D[Real] {.
    importcpp: "solve", header: "jama_cholesky.h".}
proc solve*[Real](this: var Cholesky[Real]; b: Array2D[Real]): Array2D[Real] {.
    importcpp: "solve", header: "jama_cholesky.h".}
proc isSpd*[Real](this: Cholesky[Real]): cint {.noSideEffect, importcpp: "is_spd",
    header: "jama_cholesky.h".}
## !!!Ignored construct:  template < class Real > [end of template] Cholesky < Real > [end of template] :: Cholesky ( ) : L_ ( 0 , 0 ) , isspd ( 0 ) { } *
## 	@return 1, if original matrix to be factored was symmetric
## 		positive-definite (SPD).
##  template < class Real > int Cholesky < Real > :: is_spd ( ) const { return isspd ; } *
## 	@return the lower triangular factor, L, such that L*L'=A.
##  template < class Real > Array2D < Real > Cholesky < Real > :: getL ( ) const { return L_ ; } *
## 	Constructs a lower triangular matrix L, such that L*L'= A.
## 	If A is not symmetric positive-definite (SPD), only a
## 	partial factorization is performed.  If is_spd()
## 	evalutate true (1) then the factorizaiton was successful.
##  template < class Real > Cholesky < Real > :: Cholesky ( const Array2D < Real > & A ) { int m = A . dim1 ( ) ; int n = A . dim2 ( ) ; isspd = ( m == n ) ; if ( m != n ) { L_ = Array2D < Real > ( 0 , 0 ) ; return ; } L_ = Array2D < Real > ( n , n ) ;  Main loop. for ( int j = 0 ; j < n ; j ++ ) { double d = 0.0 ; for ( int k = 0 ; k < j ; k ++ ) { Real s = 0.0 ; for ( int i = 0 ; i < k ; i ++ ) { s += L_ [ k ] [ i ] * L_ [ j ] [ i ] ; } L_ [ j ] [ k ] = s = ( A [ j ] [ k ] - s ) / L_ [ k ] [ k ] ; d = d + s * s ; isspd = isspd && ( A [ k ] [ j ] == A [ j ] [ k ] ) ; } d = A [ j ] [ j ] - d ; isspd = isspd && ( d > 0.0 ) ; L_ [ j ] [ j ] = sqrt ( d > 0.0 ? d : 0.0 ) ; for ( int k = j + 1 ; k < n ; k ++ ) { L_ [ j ] [ k ] = 0.0 ; } } } *
##
## 	Solve a linear system A*x = b, using the previously computed
## 	cholesky factorization of A: L*L'.
##
##    @param  B   A Matrix with as many rows as A and any number of columns.
##    @return     x so that L*L'*x = b.  If b is nonconformat, or if A
##    				was not symmetric posidtive definite, a null (0x0)
##    						array is returned.
##  template < class Real > Array1D < Real > Cholesky < Real > :: solve ( const Array1D < Real > & b ) { int n = L_ . dim1 ( ) ; if ( b . dim1 ( ) != n ) return Array1D < Real > ( ) ; Array1D < Real > x = b . copy ( ) ;  Solve L*y = b; for ( int k = 0 ; k < n ; k ++ ) { for ( int i = 0 ; i < k ; i ++ ) x [ k ] -= x [ i ] * L_ [ k ] [ i ] ; x [ k ] /= L_ [ k ] [ k ] ; }  Solve L'*X = Y; for ( int k = n - 1 ; k >= 0 ; k -- ) { for ( int i = k + 1 ; i < n ; i ++ ) x [ k ] -= x [ i ] * L_ [ i ] [ k ] ; x [ k ] /= L_ [ k ] [ k ] ; } return x ; } *
##
## 	Solve a linear system A*X = B, using the previously computed
## 	cholesky factorization of A: L*L'.
##
##    @param  B   A Matrix with as many rows as A and any number of columns.
##    @return     X so that L*L'*X = B.  If B is nonconformat, or if A
##    				was not symmetric posidtive definite, a null (0x0)
##    						array is returned.
##  template < class Real > Array2D < Real > Cholesky < Real > :: solve ( const Array2D < Real > & B ) { int n = L_ . dim1 ( ) ; if ( B . dim1 ( ) != n ) return Array2D < Real > ( ) ; Array2D < Real > X = B . copy ( ) ; int nx = B . dim2 ( ) ;  Cleve's original code # 0 [NewLine]  Solve L*Y = B; for ( int k = 0 ; k < n ; k ++ ) { for ( int i = k + 1 ; i < n ; i ++ ) { for ( int j = 0 ; j < nx ; j ++ ) { X [ i ] [ j ] -= X [ k ] [ j ] * L_ [ k ] [ i ] ; } } for ( int j = 0 ; j < nx ; j ++ ) { X [ k ] [ j ] /= L_ [ k ] [ k ] ; } }  Solve L'*X = Y; for ( int k = n - 1 ; k >= 0 ; k -- ) { for ( int j = 0 ; j < nx ; j ++ ) { X [ k ] [ j ] /= L_ [ k ] [ k ] ; } for ( int i = 0 ; i < k ; i ++ ) { for ( int j = 0 ; j < nx ; j ++ ) { X [ i ] [ j ] -= X [ k ] [ j ] * L_ [ k ] [ i ] ; } } } # [NewLine]  Solve L*y = b; for ( int j = 0 ; j < nx ; j ++ ) { for ( int k = 0 ; k < n ; k ++ ) { for ( int i = 0 ; i < k ; i ++ ) X [ k ] [ j ] -= X [ i ] [ j ] * L_ [ k ] [ i ] ; X [ k ] [ j ] /= L_ [ k ] [ k ] ; } }  Solve L'*X = Y; for ( int j = 0 ; j < nx ; j ++ ) { for ( int k = n - 1 ; k >= 0 ; k -- ) { for ( int i = k + 1 ; i < n ; i ++ ) X [ k ] [ j ] -= X [ i ] [ j ] * L_ [ i ] [ k ] ; X [ k ] [ j ] /= L_ [ k ] [ k ] ; } } return X ; } }
## Error: identifier expected, but got: )!!!

##  namespace JAMA

##  JAMA_CHOLESKY_H

import
  tntArray1d, tntArray2d, tntMathUtils

##  for min(), max() below

##  for abs() below

## using statement

## using statement

## *
##
##     Computes eigenvalues and eigenvectors of a real (non-complex)
##     matrix.
## <P>
##     If A is symmetric, then A = V*D*V' where the eigenvalue matrix D is
##     diagonal and the eigenvector matrix V is orthogonal. That is,
## 	the diagonal values of D are the eigenvalues, and
##     V*V' = I, where I is the identity matrix.  The columns of V
##     represent the eigenvectors in the sense that A*V = V*D.
##
## <P>
##     If A is not symmetric, then the eigenvalue matrix D is block diagonal
##     with the real eigenvalues in 1-by-1 blocks and any complex eigenvalues,
##     a + i*b, in 2-by-2 blocks, [a, b; -b, a].  That is, if the complex
##     eigenvalues look like
## <pre>
##
##           u + iv     .        .          .      .    .
##             .      u - iv     .          .      .    .
##             .        .      a + ib       .      .    .
##             .        .        .        a - ib   .    .
##             .        .        .          .      x    .
##             .        .        .          .      .    y
## </pre>
##         then D looks like
## <pre>
##
##             u        v        .          .      .    .
##            -v        u        .          .      .    .
##             .        .        a          b      .    .
##             .        .       -b          a      .    .
##             .        .        .          .      x    .
##             .        .        .          .      .    y
## </pre>
##     This keeps V a real matrix in both symmetric and non-symmetric
##     cases, and A*V = V*D.
##
##
##
##     <p>
##     The matrix V may be badly
##     conditioned, or even singular, so the validity of the equation
##     A = V*D*inverse(V) depends upon the condition number of V.
##
##    <p>
## 	(Adapted from JAMA, a Java Matrix Library, developed by jointly
## 	by the Mathworks and NIST; see  http://math.nist.gov/javanumerics/jama).
##

type
  Eigenvalue*[Real] {.importcpp: "JAMA::Eigenvalue<\'0>", header: "jama_eig.h",
                     bycopy.} = object ## * Row and column dimension (square matrix).
                                    ## * Check for symmetry, then construct the eigenvalue decomposition
                                    ##    @param A    Square real (non-complex) matrix
                                    ##
    ##  boolean
    ## * Arrays for internal storage of eigenvalues.
    ##  real part
    ##  img part
    ## * Array for internal storage of eigenvectors.
    ## * Array for internal storage of nonsymmetric Hessenberg form.
    ##    @serial internal storage of nonsymmetric Hessenberg form.
    ##
    ## * Working storage for nonsymmetric algorithm.
    ##    @serial working storage for nonsymmetric algorithm.
    ##
    ##  Symmetric Householder reduction to tridiagonal form.


proc constructEigenvalue*[Real](a: Array2D[Real]): Eigenvalue[Real] {.constructor,
    importcpp: "JAMA::Eigenvalue<\'*0>(@)", header: "jama_eig.h".}
proc getV*[Real](this: var Eigenvalue[Real]; v: var Array2D[Real]) {.importcpp: "getV",
    header: "jama_eig.h".}
proc getRealEigenvalues*[Real](this: var Eigenvalue[Real]; d: var Array1D[Real]) {.
    importcpp: "getRealEigenvalues", header: "jama_eig.h".}
proc getImagEigenvalues*[Real](this: var Eigenvalue[Real]; e: var Array1D[Real]) {.
    importcpp: "getImagEigenvalues", header: "jama_eig.h".}
proc getD*[Real](this: var Eigenvalue[Real]; d: var Array2D[Real]) {.importcpp: "getD",
    header: "jama_eig.h".}
## namespace JAMA

##  JAMA_EIG_H

import
  tnt

## for min(), max() below

## using statement

## using statement

## * LU Decomposition.
##    <P>
##    For an m-by-n matrix A with m >= n, the LU decomposition is an m-by-n
##    unit lower triangular matrix L, an n-by-n upper triangular matrix U,
##    and a permutation vector piv of length m so that A(piv,:) = L*U.
##    If m < n, then L is m-by-m and U is m-by-n.
##    <P>
##    The LU decompostion with pivoting always exists, even if the matrix is
##    singular, so the constructor will never fail.  The primary use of the
##    LU decomposition is in the solution of square systems of simultaneous
##    linear equations.  This will fail if isNonsingular() returns false.
##

type
  Lu*[Real] {.importcpp: "JAMA::LU<\'0>", header: "jama_lu.h", bycopy.} = object ##  Array for internal storage of decomposition.
                                                                         ## * LU Decomposition
                                                                         ##    @param  A   Rectangular matrix
                                                                         ##    @return     LU Decomposition object to access L, U and piv.
                                                                         ##


proc constructLu*[Real](a: Array2D[Real]): Lu[Real] {.constructor,
    importcpp: "JAMA::LU<\'*0>(@)", header: "jama_lu.h".}
proc isNonsingular*[Real](this: var Lu[Real]): cint {.importcpp: "isNonsingular",
    header: "jama_lu.h".}
proc getL*[Real](this: var Lu[Real]): Array2D[Real] {.importcpp: "getL",
    header: "jama_lu.h".}
proc getU*[Real](this: var Lu[Real]): Array2D[Real] {.importcpp: "getU",
    header: "jama_lu.h".}
proc getPivot*[Real](this: var Lu[Real]): Array1D[cint] {.importcpp: "getPivot",
    header: "jama_lu.h".}
proc det*[Real](this: var Lu[Real]): Real {.importcpp: "det", header: "jama_lu.h".}
proc solve*[Real](this: var Lu[Real]; b: Array2D[Real]): Array2D[Real] {.
    importcpp: "solve", header: "jama_lu.h".}
proc solve*[Real](this: var Lu[Real]; b: Array1D[Real]): Array1D[Real] {.
    importcpp: "solve", header: "jama_lu.h".}
##  class LU

##  namespace JAMA

##  JAMA_LU_H

import
  tntArray1d, tntArray2d, tntMathUtils

## *
## <p>
## 	Classical QR Decompisition:
##    for an m-by-n matrix A with m >= n, the QR decomposition is an m-by-n
##    orthogonal matrix Q and an n-by-n upper triangular matrix R so that
##    A = Q*R.
## <P>
##    The QR decompostion always exists, even if the matrix does not have
##    full rank, so the constructor will never fail.  The primary use of the
##    QR decomposition is in the least squares solution of nonsquare systems
##    of simultaneous linear equations.  This will fail if isFullRank()
##    returns 0 (false).
##
## <p>
## 	The Q and R factors can be retrived via the getQ() and getR()
## 	methods. Furthermore, a solve() method is provided to find the
## 	least squares solution of Ax=b using the QR factors.
##
##    <p>
## 	(Adapted from JAMA, a Java Matrix Library, developed by jointly
## 	by the Mathworks and NIST; see  http://math.nist.gov/javanumerics/jama).
##

type
  Qr*[Real] {.importcpp: "JAMA::QR<\'0>", header: "jama_qr.h", bycopy.} = object ## * Array for
                                                                         ## internal storage of
                                                                         ## decomposition.
                                                                         ##    @serial
                                                                         ## internal array
                                                                         ## storage.
                                                                         ##
                                                                         ## *
                                                                         ## 	Create a QR
                                                                         ## factorization object for A.
                                                                         ##
                                                                         ## 	@param A
                                                                         ## rectangular (m>=n) matrix.
                                                                         ##
    ## * Row and column dimensions.
    ##    @serial column dimension.
    ##    @serial row dimension.
    ##
    ## * Array for internal storage of diagonal of R.
    ##    @serial diagonal of R.
    ##


proc constructQr*[Real](a: Array2D[Real]): Qr[Real] {.constructor,
    importcpp: "JAMA::QR<\'*0>(@)", header: "jama_qr.h".}
  ##  constructor
proc isFullRank*[Real](this: Qr[Real]): cint {.noSideEffect, importcpp: "isFullRank",
    header: "jama_qr.h".}
proc getHouseholder*[Real](this: Qr[Real]): Array2D[Real] {.noSideEffect,
    importcpp: "getHouseholder", header: "jama_qr.h".}
proc getR*[Real](this: Qr[Real]): Array2D[Real] {.noSideEffect, importcpp: "getR",
    header: "jama_qr.h".}
proc getQ*[Real](this: Qr[Real]): Array2D[Real] {.noSideEffect, importcpp: "getQ",
    header: "jama_qr.h".}
proc solve*[Real](this: Qr[Real]; b: Array1D[Real]): Array1D[Real] {.noSideEffect,
    importcpp: "solve", header: "jama_qr.h".}
proc solve*[Real](this: Qr[Real]; b: Array2D[Real]): Array2D[Real] {.noSideEffect,
    importcpp: "solve", header: "jama_qr.h".}
##  namespace JAMA

##  JAMA_QR__H

import
  tntArray1d, tntArray1dUtils, tntArray2d, tntArray2dUtils, tntMathUtils

##  for min(), max() below

##  for abs() below

## using statement

## using statement

## * Singular Value Decomposition.
##    <P>
##    For an m-by-n matrix A with m >= n, the singular value decomposition is
##    an m-by-n orthogonal matrix U, an n-by-n diagonal matrix S, and
##    an n-by-n orthogonal matrix V so that A = U*S*V'.
##    <P>
##    The singular values, sigma[k] = S[k][k], are ordered so that
##    sigma[0] >= sigma[1] >= ... >= sigma[n-1].
##    <P>
##    The singular value decompostion always exists, so the constructor will
##    never fail.  The matrix condition number and the effective numerical
##    rank can be computed from this decomposition.
##
##    <p>
## 	(Adapted from JAMA, a Java Matrix Library, developed by jointly
## 	by the Mathworks and NIST; see  http://math.nist.gov/javanumerics/jama).
##

type
  Svd*[Real] {.importcpp: "JAMA::SVD<\'0>", header: "jama_svd.h", bycopy.} = object


proc constructSvd*[Real](arg: Array2D[Real]): Svd[Real] {.constructor,
    importcpp: "JAMA::SVD<\'*0>(@)", header: "jama_svd.h".}
proc getU*[Real](this: var Svd[Real]; a: var Array2D[Real]) {.importcpp: "getU",
    header: "jama_svd.h".}
proc getV*[Real](this: var Svd[Real]; a: var Array2D[Real]) {.importcpp: "getV",
    header: "jama_svd.h".}
proc getSingularValues*[Real](this: var Svd[Real]; x: var Array1D[Real]) {.
    importcpp: "getSingularValues", header: "jama_svd.h".}
proc getS*[Real](this: var Svd[Real]; a: var Array2D[Real]) {.importcpp: "getS",
    header: "jama_svd.h".}
proc norm2*[Real](this: var Svd[Real]): cdouble {.importcpp: "norm2",
    header: "jama_svd.h".}
proc cond*[Real](this: var Svd[Real]): cdouble {.importcpp: "cond",
    header: "jama_svd.h".}
proc rank*[Real](this: var Svd[Real]): cint {.importcpp: "rank", header: "jama_svd.h".}
##  JAMA_SVD_H

##
##
##  Template Numerical Toolkit (TNT): Linear Algebra Module
##
##  Mathematical and Computational Sciences Division
##  National Institute of Technology,
##  Gaithersburg, MD USA
##
##
##  This software was developed at the National Institute of Standards and
##  Technology (NIST) by employees of the Federal Government in the course
##  of their official duties. Pursuant to title 17 Section 105 of the
##  United States Code, this software is not subject to copyright protection
##  and is in the public domain. NIST assumes no responsibility whatsoever for
##  its use by other parties, and makes no guarantees, expressed or implied,
##  about its quality, reliability, or any other characteristic.
##
##

## ---------------------------------------------------------------------
##  Define this macro if you want  TNT to track some of the out-of-bounds
##  indexing. This can encur a small run-time overhead, but is recommended
##  while developing code.  It can be turned off for production runs.
##
##        #define TNT_BOUNDS_CHECK
## ---------------------------------------------------------------------
##
## #define TNT_BOUNDS_CHECK

import
  tntVersion, tntMathUtils, tntArray1d, tntArray2d, tntArray3d, tntArray1dUtils,
  tntArray2dUtils, tntArray3dUtils, tntFortranArray1d, tntFortranArray2d,
  tntFortranArray3d, tntFortranArray1dUtils, tntFortranArray2dUtils,
  tntFortranArray3dUtils, tntSparseMatrixCsr, tntStopwatch, tntSubscript, tntVec,
  tntCmat

##  TNT_H

import
  tnt

proc `/=`*(a: var Array2D[T]; k: T) {.importcpp: "(# /= #)",
                                header: "tnt2essentiautils.h".}
proc `/`*(a: Array2D[T]; k: T): Array2D[T] {.importcpp: "(# / #)",
                                       header: "tnt2essentiautils.h".}
proc matinit*[T](a: var Array2D[T]): var Array2D[T] =
  discard

##  namespace essentia

##
##  Copyright (C) 2006-2021  Music Technology Group - Universitat Pompeu Fabra
##
##  This file is part of Essentia
##
##  Essentia is free software: you can redistribute it and/or modify it under
##  the terms of the GNU Affero General Public License as published by the Free
##  Software Foundation (FSF), either version 3 of the License, or (at your
##  option) any later version.
##
##  This program is distributed in the hope that it will be useful, but WITHOUT
##  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
##  FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
##  details.
##
##  You should have received a copy of the Affero GNU General Public License
##  version 3 along with this program.  If not, see http://www.gnu.org/licenses/
##

import
  tnt

proc vecvecToArray2D*(v: Vector[Vector[Real]]): Array2D[Real] =
  discard

proc array2DToVecvec*(v2D: Array2D[Real]): Vector[Vector[Real]] =
  discard

##  namespace essentia

##
##
##  Template Numerical Toolkit (TNT)
##
##  Mathematical and Computational Sciences Division
##  National Institute of Technology,
##  Gaithersburg, MD USA
##
##
##  This software was developed at the National Institute of Standards and
##  Technology (NIST) by employees of the Federal Government in the course
##  of their official duties. Pursuant to title 17 Section 105 of the
##  United States Code, this software is not subject to copyright protection
##  and is in the public domain. NIST assumes no responsibility whatsoever for
##  its use by other parties, and makes no guarantees, expressed or implied,
##  about its quality, reliability, or any other characteristic.
##
##

## #include <cstdlib>

when defined(TNT_BOUNDS_CHECK):
  discard
import
  tntIRefvec

type
  Array1D*[T] {.importcpp: "TNT::Array1D<\'0>", header: "tnt_array1d.h", bycopy.} = object ##  ...
    ##  this normally points to v_.begin(), but
    ##  could also point to a portion (subvector)
    ##  of v_.
    ##

  Array1DvalueType*[T] = T

proc constructArray1D*[T](): Array1D[T] {.constructor,
                                       importcpp: "TNT::Array1D<\'*0>(@)",
                                       header: "tnt_array1d.h".}
proc constructArray1D*[T](n: cint): Array1D[T] {.constructor,
    importcpp: "TNT::Array1D<\'*0>(@)", header: "tnt_array1d.h".}
proc constructArray1D*[T](n: cint; a: T): Array1D[T] {.constructor,
    importcpp: "TNT::Array1D<\'*0>(@)", header: "tnt_array1d.h".}
proc constructArray1D*[T](n: cint; a: ptr T): Array1D[T] {.constructor,
    importcpp: "TNT::Array1D<\'*0>(@)", header: "tnt_array1d.h".}
proc constructArray1D*[T](a: Array1D): Array1D[T] {.constructor,
    importcpp: "TNT::Array1D<\'*0>(@)", header: "tnt_array1d.h".}
converter `t*`*[T](this: var Array1D[T]): ptr T {.importcpp: "Array1D::operator T*",
    header: "tnt_array1d.h".}
converter `constT*`*[T](this: var Array1D[T]): ptr T {.
    importcpp: "Array1D::operator constT*", header: "tnt_array1d.h".}
proc `ref`*[T](this: var Array1D[T]; a: Array1D): var Array1D {.importcpp: "ref",
    header: "tnt_array1d.h".}
proc copy*[T](this: Array1D[T]): Array1D {.noSideEffect, importcpp: "copy",
                                       header: "tnt_array1d.h".}
proc inject*[T](this: var Array1D[T]; a: Array1D): var Array1D {.importcpp: "inject",
    header: "tnt_array1d.h".}
proc `[]`*[T](this: var Array1D[T]; i: cint): var T {.importcpp: "#[@]",
    header: "tnt_array1d.h".}
proc `[]`*[T](this: Array1D[T]; i: cint): T {.noSideEffect, importcpp: "#[@]",
                                        header: "tnt_array1d.h".}
proc dim1*[T](this: Array1D[T]): cint {.noSideEffect, importcpp: "dim1",
                                    header: "tnt_array1d.h".}
proc dim*[T](this: Array1D[T]): cint {.noSideEffect, importcpp: "dim",
                                   header: "tnt_array1d.h".}
proc destroyArray1D*[T](this: var Array1D[T]) {.importcpp: "#.~Array1D()",
    header: "tnt_array1d.h".}
proc refCount*[T](this: Array1D[T]): cint {.noSideEffect, importcpp: "ref_count",
                                        header: "tnt_array1d.h".}
proc subarray*[T](this: var Array1D[T]; i0: cint; i1: cint): Array1D[T] {.
    importcpp: "subarray", header: "tnt_array1d.h".}
## !!!Ignored construct:  template < class T > [end of template] Array1D < T > [end of template] :: Array1D ( ) : v_ ( ) , n_ ( 0 ) , data_ ( 0 ) { } template < class T > Array1D < T > :: Array1D ( const Array1D < T > & A ) : v_ ( A . v_ ) , n_ ( A . n_ ) , data_ ( A . data_ ) { # TNT_DEBUG [NewLine] std :: cout << Created Array1D(const Array1D<T> &A)
##  ; # [NewLine] } template < class T > Array1D < T > :: Array1D ( int n ) : v_ ( n ) , n_ ( n ) , data_ ( v_ . begin ( ) ) { # TNT_DEBUG [NewLine] std :: cout << Created Array1D(int n)
##  ; # [NewLine] } template < class T > Array1D < T > :: Array1D ( int n , const T & val ) : v_ ( n ) , n_ ( n ) , data_ ( v_ . begin ( ) ) { # TNT_DEBUG [NewLine] std :: cout << Created Array1D(int n, const T& val)
##  ; # [NewLine] set_ ( data_ , data_ + n , val ) ; } template < class T > Array1D < T > :: Array1D ( int n , T * a ) : v_ ( a ) , n_ ( n ) , data_ ( v_ . begin ( ) ) { # TNT_DEBUG [NewLine] std :: cout << Created Array1D(int n, T* a)
##  ; # [NewLine] } template < class T > inline Array1D < T > :: operator T * ( ) { return & ( v_ [ 0 ] ) ; } template < class T > inline Array1D < T > :: operator const T * ( ) { return & ( v_ [ 0 ] ) ; } template < class T > inline T & Array1D < T > :: operator [ ] ( int i ) { # TNT_BOUNDS_CHECK [NewLine] assert ( i >= 0 ) ; assert ( i < n_ ) ; # [NewLine] return data_ [ i ] ; } template < class T > inline const T & Array1D < T > :: operator [ ] ( int i ) const { # TNT_BOUNDS_CHECK [NewLine] assert ( i >= 0 ) ; assert ( i < n_ ) ; # [NewLine] return data_ [ i ] ; } template < class T > Array1D < T > & Array1D < T > :: operator = ( const T & a ) { set_ ( data_ , data_ + n_ , a ) ; return * this ; } template < class T > Array1D < T > Array1D < T > :: copy ( ) const { Array1D A ( n_ ) ; copy_ ( A . data_ , data_ , n_ ) ; return A ; } template < class T > Array1D < T > & Array1D < T > :: inject ( const Array1D & A ) { if ( A . n_ == n_ ) copy_ ( data_ , A . data_ , n_ ) ; return * this ; } template < class T > Array1D < T > & Array1D < T > :: ref ( const Array1D < T > & A ) { if ( this != & A ) { v_ = A . v_ ;  operator= handles the reference counting. n_ = A . n_ ; data_ = A . data_ ; } return * this ; } template < class T > Array1D < T > & Array1D < T > :: operator = ( const Array1D < T > & A ) { return ref ( A ) ; } template < class T > inline int Array1D < T > :: dim1 ( ) const { return n_ ; } template < class T > inline int Array1D < T > :: dim ( ) const { return n_ ; } template < class T > Array1D < T > :: ~ Array1D ( ) { }  ............................ exented interface ...................... template < class T > inline int Array1D < T > :: ref_count ( ) const { return v_ . ref_count ( ) ; } template < class T > inline Array1D < T > Array1D < T > :: subarray ( int i0 , int i1 ) { if ( ( ( i0 > 0 ) && ( i1 < n_ ) ) || ( i0 <= i1 ) ) { Array1D < T > X ( * this ) ;  create a new instance of this array. X . n_ = i1 - i0 + 1 ; X . data_ += i0 ; return X ; } else { return Array1D < T > ( ) ; } }  private internal functions template < class T > void Array1D < T > :: set_ ( T * begin , T * end , const T & a ) { for ( T * p = begin ; p < end ; p ++ ) * p = a ; } template < class T > void Array1D < T > :: copy_ ( T * p , const T * q , int len ) const { T * end = p + len ; while ( p < end ) * p ++ = * q ++ ; } }
## Error: identifier expected, but got: )!!!

##  namespace TNT

##  TNT_ARRAY1D_H

##
##
##  Template Numerical Toolkit (TNT)
##
##  Mathematical and Computational Sciences Division
##  National Institute of Technology,
##  Gaithersburg, MD USA
##
##
##  This software was developed at the National Institute of Standards and
##  Technology (NIST) by employees of the Federal Government in the course
##  of their official duties. Pursuant to title 17 Section 105 of the
##  United States Code, this software is not subject to copyright protection
##  and is in the public domain. NIST assumes no responsibility whatsoever for
##  its use by other parties, and makes no guarantees, expressed or implied,
##  about its quality, reliability, or any other characteristic.
##
##

proc `<<`*(s: var Ostream; a: Array1D[T]): var Ostream {.importcpp: "(# << #)",
    header: "tnt_array1d_utils.h".}
proc `>>`*(s: var Istream; a: var Array1D[T]): var Istream {.importcpp: "(# >> #)",
    header: "tnt_array1d_utils.h".}
proc `+`*(a: Array1D[T]; b: Array1D[T]): Array1D[T] {.importcpp: "(# + #)",
    header: "tnt_array1d_utils.h".}
proc `-`*(a: Array1D[T]; b: Array1D[T]): Array1D[T] {.importcpp: "(# - #)",
    header: "tnt_array1d_utils.h".}
proc `*`*(a: Array1D[T]; b: Array1D[T]): Array1D[T] {.importcpp: "(# * #)",
    header: "tnt_array1d_utils.h".}
proc `/`*(a: Array1D[T]; b: Array1D[T]): Array1D[T] {.importcpp: "(# / #)",
    header: "tnt_array1d_utils.h".}
proc `+=`*(a: var Array1D[T]; b: Array1D[T]) {.importcpp: "(# += #)",
    header: "tnt_array1d_utils.h".}
proc `-=`*(a: var Array1D[T]; b: Array1D[T]) {.importcpp: "(# -= #)",
    header: "tnt_array1d_utils.h".}
proc `*=`*(a: var Array1D[T]; b: Array1D[T]) {.importcpp: "(# *= #)",
    header: "tnt_array1d_utils.h".}
proc `/=`*(a: var Array1D[T]; b: Array1D[T]) {.importcpp: "(# /= #)",
    header: "tnt_array1d_utils.h".}
##  namespace TNT

##
##
##  Template Numerical Toolkit (TNT)
##
##  Mathematical and Computational Sciences Division
##  National Institute of Technology,
##  Gaithersburg, MD USA
##
##
##  This software was developed at the National Institute of Standards and
##  Technology (NIST) by employees of the Federal Government in the course
##  of their official duties. Pursuant to title 17 Section 105 of the
##  United States Code, this software is not subject to copyright protection
##  and is in the public domain. NIST assumes no responsibility whatsoever for
##  its use by other parties, and makes no guarantees, expressed or implied,
##  about its quality, reliability, or any other characteristic.
##
##

when defined(TNT_BOUNDS_CHECK):
  discard
import
  tntArray1d

type
  Array2D*[T] {.importcpp: "TNT::Array2D<\'0>", header: "tnt_array2d.h", bycopy.} = object

  Array2DvalueType*[T] = T

proc constructArray2D*[T](): Array2D[T] {.constructor,
                                       importcpp: "TNT::Array2D<\'*0>(@)",
                                       header: "tnt_array2d.h".}
proc constructArray2D*[T](m: cint; n: cint): Array2D[T] {.constructor,
    importcpp: "TNT::Array2D<\'*0>(@)", header: "tnt_array2d.h".}
proc constructArray2D*[T](m: cint; n: cint; a: ptr T): Array2D[T] {.constructor,
    importcpp: "TNT::Array2D<\'*0>(@)", header: "tnt_array2d.h".}
proc constructArray2D*[T](m: cint; n: cint; a: T): Array2D[T] {.constructor,
    importcpp: "TNT::Array2D<\'*0>(@)", header: "tnt_array2d.h".}
proc constructArray2D*[T](a: Array2D): Array2D[T] {.constructor,
    importcpp: "TNT::Array2D<\'*0>(@)", header: "tnt_array2d.h".}
converter `t**`*[T](this: var Array2D[T]): ptr ptr T {.
    importcpp: "Array2D::operator T**", header: "tnt_array2d.h".}
converter `constT**`*[T](this: var Array2D[T]): ptr ptr T {.
    importcpp: "Array2D::operator constT**", header: "tnt_array2d.h".}
proc `ref`*[T](this: var Array2D[T]; a: Array2D): var Array2D {.importcpp: "ref",
    header: "tnt_array2d.h".}
proc copy*[T](this: Array2D[T]): Array2D {.noSideEffect, importcpp: "copy",
                                       header: "tnt_array2d.h".}
proc inject*[T](this: var Array2D[T]; a: Array2D): var Array2D {.importcpp: "inject",
    header: "tnt_array2d.h".}
proc `[]`*[T](this: var Array2D[T]; i: cint): ptr T {.importcpp: "#[@]",
    header: "tnt_array2d.h".}
proc `[]`*[T](this: Array2D[T]; i: cint): ptr T {.noSideEffect, importcpp: "#[@]",
    header: "tnt_array2d.h".}
proc dim1*[T](this: Array2D[T]): cint {.noSideEffect, importcpp: "dim1",
                                    header: "tnt_array2d.h".}
proc dim2*[T](this: Array2D[T]): cint {.noSideEffect, importcpp: "dim2",
                                    header: "tnt_array2d.h".}
proc destroyArray2D*[T](this: var Array2D[T]) {.importcpp: "#.~Array2D()",
    header: "tnt_array2d.h".}
proc refCount*[T](this: var Array2D[T]): cint {.importcpp: "ref_count",
    header: "tnt_array2d.h".}
proc refCountData*[T](this: var Array2D[T]): cint {.importcpp: "ref_count_data",
    header: "tnt_array2d.h".}
proc refCountDim1*[T](this: var Array2D[T]): cint {.importcpp: "ref_count_dim1",
    header: "tnt_array2d.h".}
proc subarray*[T](this: var Array2D[T]; i0: cint; i1: cint; j0: cint; j1: cint): Array2D {.
    importcpp: "subarray", header: "tnt_array2d.h".}
## !!!Ignored construct:  template < class T > [end of template] Array2D < T > [end of template] :: Array2D ( ) : data_ ( ) , v_ ( ) , m_ ( 0 ) , n_ ( 0 ) { } template < class T > Array2D < T > :: Array2D ( const Array2D < T > & A ) : data_ ( A . data_ ) , v_ ( A . v_ ) , m_ ( A . m_ ) , n_ ( A . n_ ) { } template < class T > Array2D < T > :: Array2D ( int m , int n ) : data_ ( m * n ) , v_ ( m ) , m_ ( m ) , n_ ( n ) { if ( m > 0 && n > 0 ) { T * p = & ( data_ [ 0 ] ) ; for ( int i = 0 ; i < m ; i ++ ) { v_ [ i ] = p ; p += n ; } } } template < class T > Array2D < T > :: Array2D ( int m , int n , const T & val ) : data_ ( m * n ) , v_ ( m ) , m_ ( m ) , n_ ( n ) { if ( m > 0 && n > 0 ) { data_ = val ; T * p = & ( data_ [ 0 ] ) ; for ( int i = 0 ; i < m ; i ++ ) { v_ [ i ] = p ; p += n ; } } } template < class T > Array2D < T > :: Array2D ( int m , int n , T * a ) : data_ ( m * n , a ) , v_ ( m ) , m_ ( m ) , n_ ( n ) { if ( m > 0 && n > 0 ) { T * p = & ( data_ [ 0 ] ) ; for ( int i = 0 ; i < m ; i ++ ) { v_ [ i ] = p ; p += n ; } } } template < class T > inline T * Array2D < T > :: operator [ ] ( int i ) { # TNT_BOUNDS_CHECK [NewLine] assert ( i >= 0 ) ; assert ( i < m_ ) ; # [NewLine] return v_ [ i ] ; } template < class T > inline const T * Array2D < T > :: operator [ ] ( int i ) const { # TNT_BOUNDS_CHECK [NewLine] assert ( i >= 0 ) ; assert ( i < m_ ) ; # [NewLine] return v_ [ i ] ; } template < class T > Array2D < T > & Array2D < T > :: operator = ( const T & a ) {  non-optimzied, but will work with subarrays in future verions for ( int i = 0 ; i < m_ ; i ++ ) for ( int j = 0 ; j < n_ ; j ++ ) v_ [ i ] [ j ] = a ; return * this ; } template < class T > Array2D < T > Array2D < T > :: copy ( ) const { Array2D A ( m_ , n_ ) ; for ( int i = 0 ; i < m_ ; i ++ ) for ( int j = 0 ; j < n_ ; j ++ ) A [ i ] [ j ] = v_ [ i ] [ j ] ; return A ; } template < class T > Array2D < T > & Array2D < T > :: inject ( const Array2D & A ) { if ( A . m_ == m_ && A . n_ == n_ ) { for ( int i = 0 ; i < m_ ; i ++ ) for ( int j = 0 ; j < n_ ; j ++ ) v_ [ i ] [ j ] = A [ i ] [ j ] ; } return * this ; } template < class T > Array2D < T > & Array2D < T > :: ref ( const Array2D < T > & A ) { if ( this != & A ) { v_ = A . v_ ; data_ = A . data_ ; m_ = A . m_ ; n_ = A . n_ ; } return * this ; } template < class T > Array2D < T > & Array2D < T > :: operator = ( const Array2D < T > & A ) { return ref ( A ) ; } template < class T > inline int Array2D < T > :: dim1 ( ) const { return m_ ; } template < class T > inline int Array2D < T > :: dim2 ( ) const { return n_ ; } template < class T > Array2D < T > :: ~ Array2D ( ) { } template < class T > inline Array2D < T > :: operator T * * ( ) { return & ( v_ [ 0 ] ) ; } template < class T > inline Array2D < T > :: operator const T * * ( ) { return & ( v_ [ 0 ] ) ; }  ............... extended interface ............... *
## 	Create a new view to a subarray defined by the boundaries
## 	[i0][i0] and [i1][j1].  The size of the subarray is
## 	(i1-i0) by (j1-j0).  If either of these lengths are zero
## 	or negative, the subarray view is null.
##
##  template < class T > Array2D < T > Array2D < T > :: subarray ( int i0 , int i1 , int j0 , int j1 ) { Array2D < T > A ; int m = i1 - i0 + 1 ; int n = j1 - j0 + 1 ;  if either length is zero or negative, this is an invalide
## 		subarray. return a null view.
##  if ( m < 1 || n < 1 ) return A ; A . data_ = data_ ; A . m_ = m ; A . n_ = n ; A . v_ = Array1D < T * > ( m ) ; T * p = & ( data_ [ 0 ] ) + i0 * n_ + j0 ; for ( int i = 0 ; i < m ; i ++ ) { A . v_ [ i ] = p + i * n_ ; } return A ; } template < class T > inline int Array2D < T > :: ref_count ( ) { return ref_count_data ( ) ; } template < class T > inline int Array2D < T > :: ref_count_data ( ) { return data_ . ref_count ( ) ; } template < class T > inline int Array2D < T > :: ref_count_dim1 ( ) { return v_ . ref_count ( ) ; } }
## Error: identifier expected, but got: )!!!

##  namespace TNT

##  TNT_ARRAY2D_H

##
##
##  Template Numerical Toolkit (TNT)
##
##  Mathematical and Computational Sciences Division
##  National Institute of Technology,
##  Gaithersburg, MD USA
##
##
##  This software was developed at the National Institute of Standards and
##  Technology (NIST) by employees of the Federal Government in the course
##  of their official duties. Pursuant to title 17 Section 105 of the
##  United States Code, this software is not subject to copyright protection
##  and is in the public domain. NIST assumes no responsibility whatsoever for
##  its use by other parties, and makes no guarantees, expressed or implied,
##  about its quality, reliability, or any other characteristic.
##
##

proc `<<`*(s: var Ostream; a: Array2D[T]): var Ostream {.importcpp: "(# << #)",
    header: "tnt_array2d_utils.h".}
proc `>>`*(s: var Istream; a: var Array2D[T]): var Istream {.importcpp: "(# >> #)",
    header: "tnt_array2d_utils.h".}
proc `+`*(a: Array2D[T]; b: Array2D[T]): Array2D[T] {.importcpp: "(# + #)",
    header: "tnt_array2d_utils.h".}
proc `-`*(a: Array2D[T]; b: Array2D[T]): Array2D[T] {.importcpp: "(# - #)",
    header: "tnt_array2d_utils.h".}
proc `*`*(a: Array2D[T]; b: Array2D[T]): Array2D[T] {.importcpp: "(# * #)",
    header: "tnt_array2d_utils.h".}
proc `/`*(a: Array2D[T]; b: Array2D[T]): Array2D[T] {.importcpp: "(# / #)",
    header: "tnt_array2d_utils.h".}
proc `+=`*(a: var Array2D[T]; b: Array2D[T]) {.importcpp: "(# += #)",
    header: "tnt_array2d_utils.h".}
proc `-=`*(a: var Array2D[T]; b: Array2D[T]) {.importcpp: "(# -= #)",
    header: "tnt_array2d_utils.h".}
proc `*=`*(a: var Array2D[T]; b: Array2D[T]) {.importcpp: "(# *= #)",
    header: "tnt_array2d_utils.h".}
proc `/=`*(a: var Array2D[T]; b: Array2D[T]) {.importcpp: "(# /= #)",
    header: "tnt_array2d_utils.h".}
## *
##     Matrix Multiply:  compute C = A*B, where C[i][j]
##     is the dot-product of row i of A and column j of B.
##
##
##     @param A an (m x n) array
##     @param B an (n x k) array
##     @return the (m x k) array A*B, or a null array (0x0)
##         if the matrices are non-conformant (i.e. the number
##         of columns of A are different than the number of rows of B.)
##
##
##

proc matmult*[T](a: Array2D[T]; b: Array2D[T]): Array2D[T] =
  discard

##  namespace TNT

##
##
##  Template Numerical Toolkit (TNT)
##
##  Mathematical and Computational Sciences Division
##  National Institute of Technology,
##  Gaithersburg, MD USA
##
##
##  This software was developed at the National Institute of Standards and
##  Technology (NIST) by employees of the Federal Government in the course
##  of their official duties. Pursuant to title 17 Section 105 of the
##  United States Code, this software is not subject to copyright protection
##  and is in the public domain. NIST assumes no responsibility whatsoever for
##  its use by other parties, and makes no guarantees, expressed or implied,
##  about its quality, reliability, or any other characteristic.
##
##

when defined(TNT_BOUNDS_CHECK):
  discard
import
  tntArray1d, tntArray2d

type
  Array3D*[T] {.importcpp: "TNT::Array3D<\'0>", header: "tnt_array3d.h", bycopy.} = object

  Array3DvalueType*[T] = T

proc constructArray3D*[T](): Array3D[T] {.constructor,
                                       importcpp: "TNT::Array3D<\'*0>(@)",
                                       header: "tnt_array3d.h".}
proc constructArray3D*[T](m: cint; n: cint; g: cint): Array3D[T] {.constructor,
    importcpp: "TNT::Array3D<\'*0>(@)", header: "tnt_array3d.h".}
proc constructArray3D*[T](m: cint; n: cint; g: cint; val: T): Array3D[T] {.constructor,
    importcpp: "TNT::Array3D<\'*0>(@)", header: "tnt_array3d.h".}
proc constructArray3D*[T](m: cint; n: cint; g: cint; a: ptr T): Array3D[T] {.constructor,
    importcpp: "TNT::Array3D<\'*0>(@)", header: "tnt_array3d.h".}
converter `t***`*[T](this: var Array3D[T]): ptr ptr ptr T {.
    importcpp: "Array3D::operator T***", header: "tnt_array3d.h".}
converter `constT***`*[T](this: var Array3D[T]): ptr ptr ptr T {.
    importcpp: "Array3D::operator constT***", header: "tnt_array3d.h".}
proc constructArray3D*[T](a: Array3D): Array3D[T] {.constructor,
    importcpp: "TNT::Array3D<\'*0>(@)", header: "tnt_array3d.h".}
proc `ref`*[T](this: var Array3D[T]; a: Array3D): var Array3D {.importcpp: "ref",
    header: "tnt_array3d.h".}
proc copy*[T](this: Array3D[T]): Array3D {.noSideEffect, importcpp: "copy",
                                       header: "tnt_array3d.h".}
proc inject*[T](this: var Array3D[T]; a: Array3D): var Array3D {.importcpp: "inject",
    header: "tnt_array3d.h".}
proc `[]`*[T](this: var Array3D[T]; i: cint): ptr ptr T {.importcpp: "#[@]",
    header: "tnt_array3d.h".}
proc `[]`*[T](this: Array3D[T]; i: cint): ptr ptr T {.noSideEffect, importcpp: "#[@]",
    header: "tnt_array3d.h".}
proc dim1*[T](this: Array3D[T]): cint {.noSideEffect, importcpp: "dim1",
                                    header: "tnt_array3d.h".}
proc dim2*[T](this: Array3D[T]): cint {.noSideEffect, importcpp: "dim2",
                                    header: "tnt_array3d.h".}
proc dim3*[T](this: Array3D[T]): cint {.noSideEffect, importcpp: "dim3",
                                    header: "tnt_array3d.h".}
proc destroyArray3D*[T](this: var Array3D[T]) {.importcpp: "#.~Array3D()",
    header: "tnt_array3d.h".}
proc refCount*[T](this: var Array3D[T]): cint {.importcpp: "ref_count",
    header: "tnt_array3d.h".}
proc subarray*[T](this: var Array3D[T]; i0: cint; i1: cint; j0: cint; j1: cint; k0: cint;
                 k1: cint): Array3D {.importcpp: "subarray", header: "tnt_array3d.h".}
## !!!Ignored construct:  template < class T > [end of template] Array3D < T > [end of template] :: Array3D ( ) : data_ ( ) , v_ ( ) , m_ ( 0 ) , n_ ( 0 ) { } template < class T > Array3D < T > :: Array3D ( const Array3D < T > & A ) : data_ ( A . data_ ) , v_ ( A . v_ ) , m_ ( A . m_ ) , n_ ( A . n_ ) , g_ ( A . g_ ) { } template < class T > Array3D < T > :: Array3D ( int m , int n , int g ) : data_ ( m * n * g ) , v_ ( m , n ) , m_ ( m ) , n_ ( n ) , g_ ( g ) { if ( m > 0 && n > 0 && g > 0 ) { T * p = & ( data_ [ 0 ] ) ; int ng = n_ * g_ ; for ( int i = 0 ; i < m_ ; i ++ ) { T * ping = p + i * ng ; for ( int j = 0 ; j < n ; j ++ ) v_ [ i ] [ j ] = ping + j * g_ ; } } } template < class T > Array3D < T > :: Array3D ( int m , int n , int g , T val ) : data_ ( m * n * g , val ) , v_ ( m , n ) , m_ ( m ) , n_ ( n ) , g_ ( g ) { if ( m > 0 && n > 0 && g > 0 ) { T * p = & ( data_ [ 0 ] ) ; int ng = n_ * g_ ; for ( int i = 0 ; i < m_ ; i ++ ) { T * ping = p + i * ng ; for ( int j = 0 ; j < n ; j ++ ) v_ [ i ] [ j ] = ping + j * g_ ; } } } template < class T > Array3D < T > :: Array3D ( int m , int n , int g , T * a ) : data_ ( m * n * g , a ) , v_ ( m , n ) , m_ ( m ) , n_ ( n ) , g_ ( g ) { if ( m > 0 && n > 0 && g > 0 ) { T * p = & ( data_ [ 0 ] ) ; int ng = n_ * g_ ; for ( int i = 0 ; i < m_ ; i ++ ) { T * ping = p + i * ng ; for ( int j = 0 ; j < n ; j ++ ) v_ [ i ] [ j ] = ping + j * g_ ; } } } template < class T > inline T * * Array3D < T > :: operator [ ] ( int i ) { # TNT_BOUNDS_CHECK [NewLine] assert ( i >= 0 ) ; assert ( i < m_ ) ; # [NewLine] return v_ [ i ] ; } template < class T > inline const T * const * Array3D < T > :: operator [ ] ( int i ) const { return v_ [ i ] ; } template < class T > Array3D < T > & Array3D < T > :: operator = ( const T & a ) { for ( int i = 0 ; i < m_ ; i ++ ) for ( int j = 0 ; j < n_ ; j ++ ) for ( int k = 0 ; k < g_ ; k ++ ) v_ [ i ] [ j ] [ k ] = a ; return * this ; } template < class T > Array3D < T > Array3D < T > :: copy ( ) const { Array3D A ( m_ , n_ , g_ ) ; for ( int i = 0 ; i < m_ ; i ++ ) for ( int j = 0 ; j < n_ ; j ++ ) for ( int k = 0 ; k < g_ ; k ++ ) A . v_ [ i ] [ j ] [ k ] = v_ [ i ] [ j ] [ k ] ; return A ; } template < class T > Array3D < T > & Array3D < T > :: inject ( const Array3D & A ) { if ( A . m_ == m_ && A . n_ == n_ && A . g_ == g_ ) for ( int i = 0 ; i < m_ ; i ++ ) for ( int j = 0 ; j < n_ ; j ++ ) for ( int k = 0 ; k < g_ ; k ++ ) v_ [ i ] [ j ] [ k ] = A . v_ [ i ] [ j ] [ k ] ; return * this ; } template < class T > Array3D < T > & Array3D < T > :: ref ( const Array3D < T > & A ) { if ( this != & A ) { m_ = A . m_ ; n_ = A . n_ ; g_ = A . g_ ; v_ = A . v_ ; data_ = A . data_ ; } return * this ; } template < class T > Array3D < T > & Array3D < T > :: operator = ( const Array3D < T > & A ) { return ref ( A ) ; } template < class T > inline int Array3D < T > :: dim1 ( ) const { return m_ ; } template < class T > inline int Array3D < T > :: dim2 ( ) const { return n_ ; } template < class T > inline int Array3D < T > :: dim3 ( ) const { return g_ ; } template < class T > Array3D < T > :: ~ Array3D ( ) { } template < class T > inline Array3D < T > :: operator T * * * ( ) { return v_ ; } template < class T > inline Array3D < T > :: operator const T * * * ( ) { return v_ ; }  extended interface template < class T > Array3D < T > Array3D < T > :: subarray ( int i0 , int i1 , int j0 , int j1 , int k0 , int k1 ) {  check that ranges are valid. if ( ! ( 0 <= i0 && i0 <= i1 && i1 < m_ && 0 <= j0 && j0 <= j1 && j1 < n_ && 0 <= k0 && k0 <= k1 && k1 < g_ ) ) return Array3D < T > ( ) ;  null array Array3D < T > A ; A . data_ = data_ ; A . m_ = i1 - i0 + 1 ; A . n_ = j1 - j0 + 1 ; A . g_ = k1 - k0 + 1 ; A . v_ = Array2D < T * > ( A . m_ , A . n_ ) ; T * p = & ( data_ [ 0 ] ) + i0 * n_ * g_ + j0 * g_ + k0 ; for ( int i = 0 ; i < A . m_ ; i ++ ) { T * ping = p + i * n_ * g_ ; for ( int j = 0 ; j < A . n_ ; j ++ ) A . v_ [ i ] [ j ] = ping + j * g_ ; } return A ; } }
## Error: identifier expected, but got: )!!!

##  namespace TNT

##  TNT_ARRAY3D_H

proc `<<`*(s: var Ostream; a: Array3D[T]): var Ostream {.importcpp: "(# << #)",
    header: "tnt_array3d_utils.h".}
proc `>>`*(s: var Istream; a: var Array3D[T]): var Istream {.importcpp: "(# >> #)",
    header: "tnt_array3d_utils.h".}
proc `+`*(a: Array3D[T]; b: Array3D[T]): Array3D[T] {.importcpp: "(# + #)",
    header: "tnt_array3d_utils.h".}
proc `-`*(a: Array3D[T]; b: Array3D[T]): Array3D[T] {.importcpp: "(# - #)",
    header: "tnt_array3d_utils.h".}
proc `*`*(a: Array3D[T]; b: Array3D[T]): Array3D[T] {.importcpp: "(# * #)",
    header: "tnt_array3d_utils.h".}
proc `/`*(a: Array3D[T]; b: Array3D[T]): Array3D[T] {.importcpp: "(# / #)",
    header: "tnt_array3d_utils.h".}
proc `+=`*(a: var Array3D[T]; b: Array3D[T]) {.importcpp: "(# += #)",
    header: "tnt_array3d_utils.h".}
proc `-=`*(a: var Array3D[T]; b: Array3D[T]) {.importcpp: "(# -= #)",
    header: "tnt_array3d_utils.h".}
proc `*=`*(a: var Array3D[T]; b: Array3D[T]) {.importcpp: "(# *= #)",
    header: "tnt_array3d_utils.h".}
proc `/=`*(a: var Array3D[T]; b: Array3D[T]) {.importcpp: "(# /= #)",
    header: "tnt_array3d_utils.h".}
##  namespace TNT

##
##
##  Template Numerical Toolkit (TNT)
##
##  Mathematical and Computational Sciences Division
##  National Institute of Technology,
##  Gaithersburg, MD USA
##
##
##  This software was developed at the National Institute of Standards and
##  Technology (NIST) by employees of the Federal Government in the course
##  of their official duties. Pursuant to title 17 Section 105 of the
##  United States Code, this software is not subject to copyright protection
##  and is in the public domain. NIST assumes no responsibility whatsoever for
##  its use by other parties, and makes no guarantees, expressed or implied,
##  about its quality, reliability, or any other characteristic.
##
##
##  C compatible matrix: row-oriented, 0-based [i][j] and 1-based (i,j) indexing
##

import
  tntSubscript, tntVec

type
  Matrix*[T] {.importcpp: "TNT::Matrix<\'0>", header: "tnt_cmat.h", bycopy.} = object
    ##  total size
    ##  these point to the same data, but are 1-based
    ##  internal helper function to create the array
    ##  of row pointers

  MatrixsizeType* = Subscript
  MatrixvalueType*[T] = T
  MatrixelementType*[T] = T
  Matrixpointer*[T] = ptr T
  Matrixiterator*[T] = ptr T
  Matrixreference*[T] = var T
  MatrixconstIterator*[T] = ptr T
  MatrixconstReference*[T] = T

proc lbound*[T](this: Matrix[T]): Subscript {.noSideEffect, importcpp: "lbound",
    header: "tnt_cmat.h".}
converter `t**`*[T](this: var Matrix[T]): ptr ptr T {.
    importcpp: "Matrix::operator T**", header: "tnt_cmat.h".}
converter `t**`*[T](this: Matrix[T]): ptr ptr T {.noSideEffect,
    importcpp: "Matrix::operator T**", header: "tnt_cmat.h".}
proc size*[T](this: Matrix[T]): Subscript {.noSideEffect, importcpp: "size",
                                        header: "tnt_cmat.h".}
proc constructMatrix*[T](): Matrix[T] {.constructor,
                                     importcpp: "TNT::Matrix<\'*0>(@)",
                                     header: "tnt_cmat.h".}
proc constructMatrix*[T](a: Matrix[T]): Matrix[T] {.constructor,
    importcpp: "TNT::Matrix<\'*0>(@)", header: "tnt_cmat.h".}
proc constructMatrix*[T](m: Subscript; n: Subscript; value: T = t()): Matrix[T] {.
    constructor, importcpp: "TNT::Matrix<\'*0>(@)", header: "tnt_cmat.h".}
proc constructMatrix*[T](m: Subscript; n: Subscript; v: ptr T): Matrix[T] {.constructor,
    importcpp: "TNT::Matrix<\'*0>(@)", header: "tnt_cmat.h".}
proc constructMatrix*[T](m: Subscript; n: Subscript; s: cstring): Matrix[T] {.
    constructor, importcpp: "TNT::Matrix<\'*0>(@)", header: "tnt_cmat.h".}
proc destroyMatrix*[T](this: var Matrix[T]) {.importcpp: "#.~Matrix()",
    header: "tnt_cmat.h".}
proc newsize*[T](this: var Matrix[T]; m: Subscript; n: Subscript): var Matrix[T] {.
    importcpp: "newsize", header: "tnt_cmat.h".}
proc dim*[T](this: Matrix[T]; d: Subscript): Subscript {.noSideEffect,
    importcpp: "dim", header: "tnt_cmat.h".}
proc numRows*[T](this: Matrix[T]): Subscript {.noSideEffect, importcpp: "num_rows",
    header: "tnt_cmat.h".}
proc numCols*[T](this: Matrix[T]): Subscript {.noSideEffect, importcpp: "num_cols",
    header: "tnt_cmat.h".}
proc `[]`*[T](this: var Matrix[T]; i: Subscript): ptr T {.importcpp: "#[@]",
    header: "tnt_cmat.h".}
proc `[]`*[T](this: Matrix[T]; i: Subscript): ptr T {.noSideEffect, importcpp: "#[@]",
    header: "tnt_cmat.h".}
proc `()`*[T](this: var Matrix[T]; i: Subscript): Matrixreference {.importcpp: "#(@)",
    header: "tnt_cmat.h".}
proc `()`*[T](this: Matrix[T]; i: Subscript): MatrixconstReference {.noSideEffect,
    importcpp: "#(@)", header: "tnt_cmat.h".}
proc `()`*[T](this: var Matrix[T]; i: Subscript; j: Subscript): Matrixreference {.
    importcpp: "#(@)", header: "tnt_cmat.h".}
proc `()`*[T](this: Matrix[T]; i: Subscript; j: Subscript): MatrixconstReference {.
    noSideEffect, importcpp: "#(@)", header: "tnt_cmat.h".}
##  ***************************  I/O  *******************************

proc `<<`*(s: var Ostream; a: Matrix[T]): var Ostream {.importcpp: "(# << #)",
    header: "tnt_cmat.h".}
proc `>>`*(s: var Istream; a: var Matrix[T]): var Istream {.importcpp: "(# >> #)",
    header: "tnt_cmat.h".}
##  *******************[ basic matrix algorithms ]***************************

proc `+`*(a: Matrix[T]; b: Matrix[T]): Matrix[T] {.importcpp: "(# + #)",
    header: "tnt_cmat.h".}
proc `-`*(a: Matrix[T]; b: Matrix[T]): Matrix[T] {.importcpp: "(# - #)",
    header: "tnt_cmat.h".}
proc multElement*[T](a: Matrix[T]; b: Matrix[T]): Matrix[T] =
  discard

proc transpose*[T](a: Matrix[T]): Matrix[T] =
  discard

proc matmult*[T](a: Matrix[T]; b: Matrix[T]): Matrix[T] =
  discard

proc `*`*(a: Matrix[T]; b: Matrix[T]): Matrix[T] {.importcpp: "(# * #)",
    header: "tnt_cmat.h".}
proc matmult*[T](c: var Matrix[T]; a: Matrix[T]; b: Matrix[T]): cint =
  discard

proc matmult*[T](a: Matrix[T]; x: Vector[T]): Vector[T] =
  discard

proc `*`*(a: Matrix[T]; x: Vector[T]): Vector[T] {.importcpp: "(# * #)",
    header: "tnt_cmat.h".}
##  namespace TNT

##  CMAT_H

##
##
##  Template Numerical Toolkit (TNT)
##
##  Mathematical and Computational Sciences Division
##  National Institute of Technology,
##  Gaithersburg, MD USA
##
##
##  This software was developed at the National Institute of Standards and
##  Technology (NIST) by employees of the Federal Government in the course
##  of their official duties. Pursuant to title 17 Section 105 of the
##  United States Code, this software is not subject to copyright protection
##  and is in the public domain. NIST assumes no responsibility whatsoever for
##  its use by other parties, and makes no guarantees, expressed or implied,
##  about its quality, reliability, or any other characteristic.
##
##

when defined(TNT_BOUNDS_CHECK):
  discard
import
  tntIRefvec

type
  FortranArray1D*[T] {.importcpp: "TNT::Fortran_Array1D<\'0>",
                      header: "tnt_fortran_array1d.h", bycopy.} = object
    ##  this normally points to v_.begin(), but
    ##  could also point to a portion (subvector)
    ##  of v_.
    ##

  FortranArray1DvalueType*[T] = T

proc constructFortranArray1D*[T](): FortranArray1D[T] {.constructor,
    importcpp: "TNT::Fortran_Array1D<\'*0>(@)", header: "tnt_fortran_array1d.h".}
proc constructFortranArray1D*[T](n: cint): FortranArray1D[T] {.constructor,
    importcpp: "TNT::Fortran_Array1D<\'*0>(@)", header: "tnt_fortran_array1d.h".}
proc constructFortranArray1D*[T](n: cint; a: T): FortranArray1D[T] {.constructor,
    importcpp: "TNT::Fortran_Array1D<\'*0>(@)", header: "tnt_fortran_array1d.h".}
proc constructFortranArray1D*[T](n: cint; a: ptr T): FortranArray1D[T] {.constructor,
    importcpp: "TNT::Fortran_Array1D<\'*0>(@)", header: "tnt_fortran_array1d.h".}
proc constructFortranArray1D*[T](a: FortranArray1D): FortranArray1D[T] {.
    constructor, importcpp: "TNT::Fortran_Array1D<\'*0>(@)",
    header: "tnt_fortran_array1d.h".}
proc `ref`*[T](this: var FortranArray1D[T]; a: FortranArray1D): var FortranArray1D {.
    importcpp: "ref", header: "tnt_fortran_array1d.h".}
proc copy*[T](this: FortranArray1D[T]): FortranArray1D {.noSideEffect,
    importcpp: "copy", header: "tnt_fortran_array1d.h".}
proc inject*[T](this: var FortranArray1D[T]; a: FortranArray1D): var FortranArray1D {.
    importcpp: "inject", header: "tnt_fortran_array1d.h".}
proc `()`*[T](this: var FortranArray1D[T]; i: cint): var T {.importcpp: "#(@)",
    header: "tnt_fortran_array1d.h".}
proc `()`*[T](this: FortranArray1D[T]; i: cint): T {.noSideEffect, importcpp: "#(@)",
    header: "tnt_fortran_array1d.h".}
proc dim1*[T](this: FortranArray1D[T]): cint {.noSideEffect, importcpp: "dim1",
    header: "tnt_fortran_array1d.h".}
proc dim*[T](this: FortranArray1D[T]): cint {.noSideEffect, importcpp: "dim",
    header: "tnt_fortran_array1d.h".}
proc destroyFortranArray1D*[T](this: var FortranArray1D[T]) {.
    importcpp: "#.~Fortran_Array1D()", header: "tnt_fortran_array1d.h".}
proc refCount*[T](this: FortranArray1D[T]): cint {.noSideEffect,
    importcpp: "ref_count", header: "tnt_fortran_array1d.h".}
proc subarray*[T](this: var FortranArray1D[T]; i0: cint; i1: cint): FortranArray1D[T] {.
    importcpp: "subarray", header: "tnt_fortran_array1d.h".}
## !!!Ignored construct:  template < class T > [end of template] Fortran_Array1D < T > [end of template] :: Fortran_Array1D ( ) : v_ ( ) , n_ ( 0 ) , data_ ( 0 ) { } template < class T > Fortran_Array1D < T > :: Fortran_Array1D ( const Fortran_Array1D < T > & A ) : v_ ( A . v_ ) , n_ ( A . n_ ) , data_ ( A . data_ ) { # TNT_DEBUG [NewLine] std :: cout << Created Fortran_Array1D(const Fortran_Array1D<T> &A)
##  ; # [NewLine] } template < class T > Fortran_Array1D < T > :: Fortran_Array1D ( int n ) : v_ ( n ) , n_ ( n ) , data_ ( v_ . begin ( ) ) { # TNT_DEBUG [NewLine] std :: cout << Created Fortran_Array1D(int n)
##  ; # [NewLine] } template < class T > Fortran_Array1D < T > :: Fortran_Array1D ( int n , const T & val ) : v_ ( n ) , n_ ( n ) , data_ ( v_ . begin ( ) ) { # TNT_DEBUG [NewLine] std :: cout << Created Fortran_Array1D(int n, const T& val)
##  ; # [NewLine] set_ ( data_ , data_ + n , val ) ; } template < class T > Fortran_Array1D < T > :: Fortran_Array1D ( int n , T * a ) : v_ ( a ) , n_ ( n ) , data_ ( v_ . begin ( ) ) { # TNT_DEBUG [NewLine] std :: cout << Created Fortran_Array1D(int n, T* a)
##  ; # [NewLine] } template < class T > inline T & Fortran_Array1D < T > :: operator ( ) ( int i ) { # TNT_BOUNDS_CHECK [NewLine] assert ( i >= 1 ) ; assert ( i <= n_ ) ; # [NewLine] return data_ [ i - 1 ] ; } template < class T > inline const T & Fortran_Array1D < T > :: operator ( ) ( int i ) const { # TNT_BOUNDS_CHECK [NewLine] assert ( i >= 1 ) ; assert ( i <= n_ ) ; # [NewLine] return data_ [ i - 1 ] ; } template < class T > Fortran_Array1D < T > & Fortran_Array1D < T > :: operator = ( const T & a ) { set_ ( data_ , data_ + n_ , a ) ; return * this ; } template < class T > Fortran_Array1D < T > Fortran_Array1D < T > :: copy ( ) const { Fortran_Array1D A ( n_ ) ; copy_ ( A . data_ , data_ , n_ ) ; return A ; } template < class T > Fortran_Array1D < T > & Fortran_Array1D < T > :: inject ( const Fortran_Array1D & A ) { if ( A . n_ == n_ ) copy_ ( data_ , A . data_ , n_ ) ; return * this ; } template < class T > Fortran_Array1D < T > & Fortran_Array1D < T > :: ref ( const Fortran_Array1D < T > & A ) { if ( this != & A ) { v_ = A . v_ ;  operator= handles the reference counting. n_ = A . n_ ; data_ = A . data_ ; } return * this ; } template < class T > Fortran_Array1D < T > & Fortran_Array1D < T > :: operator = ( const Fortran_Array1D < T > & A ) { return ref ( A ) ; } template < class T > inline int Fortran_Array1D < T > :: dim1 ( ) const { return n_ ; } template < class T > inline int Fortran_Array1D < T > :: dim ( ) const { return n_ ; } template < class T > Fortran_Array1D < T > :: ~ Fortran_Array1D ( ) { }  ............................ exented interface ...................... template < class T > inline int Fortran_Array1D < T > :: ref_count ( ) const { return v_ . ref_count ( ) ; } template < class T > inline Fortran_Array1D < T > Fortran_Array1D < T > :: subarray ( int i0 , int i1 ) { # TNT_DEBUG [NewLine] std :: cout << entered subarray.
##  ; # [NewLine] if ( ( ( i0 > 0 ) && ( i1 < n_ ) ) || ( i0 <= i1 ) ) { Fortran_Array1D < T > X ( * this ) ;  create a new instance of this array. X . n_ = i1 - i0 + 1 ; X . data_ += i0 ; return X ; } else { # TNT_DEBUG [NewLine] std :: cout << subarray:  null return.
##  ; # [NewLine] return Fortran_Array1D < T > ( ) ; } }  private internal functions template < class T > void Fortran_Array1D < T > :: set_ ( T * begin , T * end , const T & a ) { for ( T * p = begin ; p < end ; p ++ ) * p = a ; } template < class T > void Fortran_Array1D < T > :: copy_ ( T * p , const T * q , int len ) const { T * end = p + len ; while ( p < end ) * p ++ = * q ++ ; } }
## Error: identifier expected, but got: )!!!

##  namespace TNT

##  TNT_FORTRAN_ARRAY1D_H

##
##
##  Template Numerical Toolkit (TNT)
##
##  Mathematical and Computational Sciences Division
##  National Institute of Technology,
##  Gaithersburg, MD USA
##
##
##  This software was developed at the National Institute of Standards and
##  Technology (NIST) by employees of the Federal Government in the course
##  of their official duties. Pursuant to title 17 Section 105 of the
##  United States Code, this software is not subject to copyright protection
##  and is in the public domain. NIST assumes no responsibility whatsoever for
##  its use by other parties, and makes no guarantees, expressed or implied,
##  about its quality, reliability, or any other characteristic.
##
##

## *
## 	Write an array to a character outstream.  Output format is one that can
## 	be read back in via the in-stream operator: one integer
## 	denoting the array dimension (n), followed by n elements,
## 	one per line.
##
##

proc `<<`*(s: var Ostream; a: FortranArray1D[T]): var Ostream {.importcpp: "(# << #)",
    header: "tnt_fortran_array1d_utils.h".}
## *
## 	Read an array from a character stream.  Input format
## 	is one integer, denoting the dimension (n), followed
## 	by n whitespace-separated elments.  Newlines are ignored
##
## 	<p>
## 	Note: the array being read into references new memory
## 	storage. If the intent is to fill an existing conformant
## 	array, use <code> cin >> B;  A.inject(B) ); </code>
## 	instead or read the elements in one-a-time by hand.
##
## 	@param s the charater to read from (typically <code>std::in</code>)
## 	@param A the array to read into.
##

proc `>>`*(s: var Istream; a: var FortranArray1D[T]): var Istream {.
    importcpp: "(# >> #)", header: "tnt_fortran_array1d_utils.h".}
proc `+`*(a: FortranArray1D[T]; b: FortranArray1D[T]): FortranArray1D[T] {.
    importcpp: "(# + #)", header: "tnt_fortran_array1d_utils.h".}
proc `-`*(a: FortranArray1D[T]; b: FortranArray1D[T]): FortranArray1D[T] {.
    importcpp: "(# - #)", header: "tnt_fortran_array1d_utils.h".}
proc `*`*(a: FortranArray1D[T]; b: FortranArray1D[T]): FortranArray1D[T] {.
    importcpp: "(# * #)", header: "tnt_fortran_array1d_utils.h".}
proc `/`*(a: FortranArray1D[T]; b: FortranArray1D[T]): FortranArray1D[T] {.
    importcpp: "(# / #)", header: "tnt_fortran_array1d_utils.h".}
proc `+=`*(a: var FortranArray1D[T]; b: FortranArray1D[T]) {.importcpp: "(# += #)",
    header: "tnt_fortran_array1d_utils.h".}
proc `-=`*(a: var FortranArray1D[T]; b: FortranArray1D[T]) {.importcpp: "(# -= #)",
    header: "tnt_fortran_array1d_utils.h".}
proc `*=`*(a: var FortranArray1D[T]; b: FortranArray1D[T]) {.importcpp: "(# *= #)",
    header: "tnt_fortran_array1d_utils.h".}
proc `/=`*(a: var FortranArray1D[T]; b: FortranArray1D[T]) {.importcpp: "(# /= #)",
    header: "tnt_fortran_array1d_utils.h".}
##  namespace TNT

##
##
##  Template Numerical Toolkit (TNT): Two-dimensional Fortran numerical array
##
##  Mathematical and Computational Sciences Division
##  National Institute of Technology,
##  Gaithersburg, MD USA
##
##
##  This software was developed at the National Institute of Standards and
##  Technology (NIST) by employees of the Federal Government in the course
##  of their official duties. Pursuant to title 17 Section 105 of the
##  United States Code, this software is not subject to copyright protection
##  and is in the public domain. NIST assumes no responsibility whatsoever for
##  its use by other parties, and makes no guarantees, expressed or implied,
##  about its quality, reliability, or any other characteristic.
##
##

when defined(TNT_BOUNDS_CHECK):
  discard
import
  tntIRefvec

type
  FortranArray2D*[T] {.importcpp: "TNT::Fortran_Array2D<\'0>",
                      header: "tnt_fortran_array2d.h", bycopy.} = object

  FortranArray2DvalueType*[T] = T

proc constructFortranArray2D*[T](): FortranArray2D[T] {.constructor,
    importcpp: "TNT::Fortran_Array2D<\'*0>(@)", header: "tnt_fortran_array2d.h".}
proc constructFortranArray2D*[T](m: cint; n: cint): FortranArray2D[T] {.constructor,
    importcpp: "TNT::Fortran_Array2D<\'*0>(@)", header: "tnt_fortran_array2d.h".}
proc constructFortranArray2D*[T](m: cint; n: cint; a: ptr T): FortranArray2D[T] {.
    constructor, importcpp: "TNT::Fortran_Array2D<\'*0>(@)",
    header: "tnt_fortran_array2d.h".}
proc constructFortranArray2D*[T](m: cint; n: cint; a: T): FortranArray2D[T] {.
    constructor, importcpp: "TNT::Fortran_Array2D<\'*0>(@)",
    header: "tnt_fortran_array2d.h".}
proc constructFortranArray2D*[T](a: FortranArray2D): FortranArray2D[T] {.
    constructor, importcpp: "TNT::Fortran_Array2D<\'*0>(@)",
    header: "tnt_fortran_array2d.h".}
proc `ref`*[T](this: var FortranArray2D[T]; a: FortranArray2D): var FortranArray2D {.
    importcpp: "ref", header: "tnt_fortran_array2d.h".}
proc copy*[T](this: FortranArray2D[T]): FortranArray2D {.noSideEffect,
    importcpp: "copy", header: "tnt_fortran_array2d.h".}
proc inject*[T](this: var FortranArray2D[T]; a: FortranArray2D): var FortranArray2D {.
    importcpp: "inject", header: "tnt_fortran_array2d.h".}
proc `()`*[T](this: var FortranArray2D[T]; i: cint; j: cint): var T {.importcpp: "#(@)",
    header: "tnt_fortran_array2d.h".}
proc `()`*[T](this: FortranArray2D[T]; i: cint; j: cint): T {.noSideEffect,
    importcpp: "#(@)", header: "tnt_fortran_array2d.h".}
proc dim1*[T](this: FortranArray2D[T]): cint {.noSideEffect, importcpp: "dim1",
    header: "tnt_fortran_array2d.h".}
proc dim2*[T](this: FortranArray2D[T]): cint {.noSideEffect, importcpp: "dim2",
    header: "tnt_fortran_array2d.h".}
proc destroyFortranArray2D*[T](this: var FortranArray2D[T]) {.
    importcpp: "#.~Fortran_Array2D()", header: "tnt_fortran_array2d.h".}
proc refCount*[T](this: FortranArray2D[T]): cint {.noSideEffect,
    importcpp: "ref_count", header: "tnt_fortran_array2d.h".}
## !!!Ignored construct:  template < class T > [end of template] Fortran_Array2D < T > [end of template] :: Fortran_Array2D ( ) : v_ ( ) , m_ ( 0 ) , n_ ( 0 ) , data_ ( 0 ) { } template < class T > Fortran_Array2D < T > :: Fortran_Array2D ( const Fortran_Array2D < T > & A ) : v_ ( A . v_ ) , m_ ( A . m_ ) , n_ ( A . n_ ) , data_ ( A . data_ ) { } template < class T > Fortran_Array2D < T > :: Fortran_Array2D ( int m , int n ) : v_ ( m * n ) , m_ ( m ) , n_ ( n ) , data_ ( v_ . begin ( ) ) { } template < class T > Fortran_Array2D < T > :: Fortran_Array2D ( int m , int n , const T & val ) : v_ ( m * n ) , m_ ( m ) , n_ ( n ) , data_ ( v_ . begin ( ) ) { set_ ( data_ , data_ + m * n , val ) ; } template < class T > Fortran_Array2D < T > :: Fortran_Array2D ( int m , int n , T * a ) : v_ ( a ) , m_ ( m ) , n_ ( n ) , data_ ( v_ . begin ( ) ) { } template < class T > inline T & Fortran_Array2D < T > :: operator ( ) ( int i , int j ) { # TNT_BOUNDS_CHECK [NewLine] assert ( i >= 1 ) ; assert ( i <= m_ ) ; assert ( j >= 1 ) ; assert ( j <= n_ ) ; # [NewLine] return v_ [ ( j - 1 ) * m_ + ( i - 1 ) ] ; } template < class T > inline const T & Fortran_Array2D < T > :: operator ( ) ( int i , int j ) const { # TNT_BOUNDS_CHECK [NewLine] assert ( i >= 1 ) ; assert ( i <= m_ ) ; assert ( j >= 1 ) ; assert ( j <= n_ ) ; # [NewLine] return v_ [ ( j - 1 ) * m_ + ( i - 1 ) ] ; } template < class T > Fortran_Array2D < T > & Fortran_Array2D < T > :: operator = ( const T & a ) { set_ ( data_ , data_ + m_ * n_ , a ) ; return * this ; } template < class T > Fortran_Array2D < T > Fortran_Array2D < T > :: copy ( ) const { Fortran_Array2D B ( m_ , n_ ) ; B . inject ( * this ) ; return B ; } template < class T > Fortran_Array2D < T > & Fortran_Array2D < T > :: inject ( const Fortran_Array2D & A ) { if ( m_ == A . m_ && n_ == A . n_ ) copy_ ( data_ , A . data_ , m_ * n_ ) ; return * this ; } template < class T > Fortran_Array2D < T > & Fortran_Array2D < T > :: ref ( const Fortran_Array2D < T > & A ) { if ( this != & A ) { v_ = A . v_ ; m_ = A . m_ ; n_ = A . n_ ; data_ = A . data_ ; } return * this ; } template < class T > Fortran_Array2D < T > & Fortran_Array2D < T > :: operator = ( const Fortran_Array2D < T > & A ) { return ref ( A ) ; } template < class T > inline int Fortran_Array2D < T > :: dim1 ( ) const { return m_ ; } template < class T > inline int Fortran_Array2D < T > :: dim2 ( ) const { return n_ ; } template < class T > Fortran_Array2D < T > :: ~ Fortran_Array2D ( ) { } template < class T > inline int Fortran_Array2D < T > :: ref_count ( ) const { return v_ . ref_count ( ) ; } template < class T > void Fortran_Array2D < T > :: set_ ( T * begin , T * end , const T & a ) { for ( T * p = begin ; p < end ; p ++ ) * p = a ; } template < class T > void Fortran_Array2D < T > :: copy_ ( T * p , const T * q , int len ) { T * end = p + len ; while ( p < end ) * p ++ = * q ++ ; } }
## Error: identifier expected, but got: )!!!

##  namespace TNT

##  TNT_FORTRAN_ARRAY2D_H

##
##
##  Template Numerical Toolkit (TNT)
##
##  Mathematical and Computational Sciences Division
##  National Institute of Technology,
##  Gaithersburg, MD USA
##
##
##  This software was developed at the National Institute of Standards and
##  Technology (NIST) by employees of the Federal Government in the course
##  of their official duties. Pursuant to title 17 Section 105 of the
##  United States Code, this software is not subject to copyright protection
##  and is in the public domain. NIST assumes no responsibility whatsoever for
##  its use by other parties, and makes no guarantees, expressed or implied,
##  about its quality, reliability, or any other characteristic.
##
##

proc `<<`*(s: var Ostream; a: FortranArray2D[T]): var Ostream {.importcpp: "(# << #)",
    header: "tnt_fortran_array2d_utils.h".}
proc `>>`*(s: var Istream; a: var FortranArray2D[T]): var Istream {.
    importcpp: "(# >> #)", header: "tnt_fortran_array2d_utils.h".}
proc `+`*(a: FortranArray2D[T]; b: FortranArray2D[T]): FortranArray2D[T] {.
    importcpp: "(# + #)", header: "tnt_fortran_array2d_utils.h".}
proc `-`*(a: FortranArray2D[T]; b: FortranArray2D[T]): FortranArray2D[T] {.
    importcpp: "(# - #)", header: "tnt_fortran_array2d_utils.h".}
proc `*`*(a: FortranArray2D[T]; b: FortranArray2D[T]): FortranArray2D[T] {.
    importcpp: "(# * #)", header: "tnt_fortran_array2d_utils.h".}
proc `/`*(a: FortranArray2D[T]; b: FortranArray2D[T]): FortranArray2D[T] {.
    importcpp: "(# / #)", header: "tnt_fortran_array2d_utils.h".}
proc `+=`*(a: var FortranArray2D[T]; b: FortranArray2D[T]) {.importcpp: "(# += #)",
    header: "tnt_fortran_array2d_utils.h".}
proc `-=`*(a: var FortranArray2D[T]; b: FortranArray2D[T]) {.importcpp: "(# -= #)",
    header: "tnt_fortran_array2d_utils.h".}
proc `*=`*(a: var FortranArray2D[T]; b: FortranArray2D[T]) {.importcpp: "(# *= #)",
    header: "tnt_fortran_array2d_utils.h".}
proc `/=`*(a: var FortranArray2D[T]; b: FortranArray2D[T]) {.importcpp: "(# /= #)",
    header: "tnt_fortran_array2d_utils.h".}
##  namespace TNT

##
##
##  Template Numerical Toolkit (TNT): Three-dimensional Fortran numerical array
##
##  Mathematical and Computational Sciences Division
##  National Institute of Technology,
##  Gaithersburg, MD USA
##
##
##  This software was developed at the National Institute of Standards and
##  Technology (NIST) by employees of the Federal Government in the course
##  of their official duties. Pursuant to title 17 Section 105 of the
##  United States Code, this software is not subject to copyright protection
##  and is in the public domain. NIST assumes no responsibility whatsoever for
##  its use by other parties, and makes no guarantees, expressed or implied,
##  about its quality, reliability, or any other characteristic.
##
##

when defined(TNT_BOUNDS_CHECK):
  discard
import
  tntIRefvec

type
  FortranArray3D*[T] {.importcpp: "TNT::Fortran_Array3D<\'0>",
                      header: "tnt_fortran_array3d.h", bycopy.} = object

  FortranArray3DvalueType*[T] = T

proc constructFortranArray3D*[T](): FortranArray3D[T] {.constructor,
    importcpp: "TNT::Fortran_Array3D<\'*0>(@)", header: "tnt_fortran_array3d.h".}
proc constructFortranArray3D*[T](m: cint; n: cint; k: cint): FortranArray3D[T] {.
    constructor, importcpp: "TNT::Fortran_Array3D<\'*0>(@)",
    header: "tnt_fortran_array3d.h".}
proc constructFortranArray3D*[T](m: cint; n: cint; k: cint; a: ptr T): FortranArray3D[T] {.
    constructor, importcpp: "TNT::Fortran_Array3D<\'*0>(@)",
    header: "tnt_fortran_array3d.h".}
proc constructFortranArray3D*[T](m: cint; n: cint; k: cint; a: T): FortranArray3D[T] {.
    constructor, importcpp: "TNT::Fortran_Array3D<\'*0>(@)",
    header: "tnt_fortran_array3d.h".}
proc constructFortranArray3D*[T](a: FortranArray3D): FortranArray3D[T] {.
    constructor, importcpp: "TNT::Fortran_Array3D<\'*0>(@)",
    header: "tnt_fortran_array3d.h".}
proc `ref`*[T](this: var FortranArray3D[T]; a: FortranArray3D): var FortranArray3D {.
    importcpp: "ref", header: "tnt_fortran_array3d.h".}
proc copy*[T](this: FortranArray3D[T]): FortranArray3D {.noSideEffect,
    importcpp: "copy", header: "tnt_fortran_array3d.h".}
proc inject*[T](this: var FortranArray3D[T]; a: FortranArray3D): var FortranArray3D {.
    importcpp: "inject", header: "tnt_fortran_array3d.h".}
proc `()`*[T](this: var FortranArray3D[T]; i: cint; j: cint; k: cint): var T {.
    importcpp: "#(@)", header: "tnt_fortran_array3d.h".}
proc `()`*[T](this: FortranArray3D[T]; i: cint; j: cint; k: cint): T {.noSideEffect,
    importcpp: "#(@)", header: "tnt_fortran_array3d.h".}
proc dim1*[T](this: FortranArray3D[T]): cint {.noSideEffect, importcpp: "dim1",
    header: "tnt_fortran_array3d.h".}
proc dim2*[T](this: FortranArray3D[T]): cint {.noSideEffect, importcpp: "dim2",
    header: "tnt_fortran_array3d.h".}
proc dim3*[T](this: FortranArray3D[T]): cint {.noSideEffect, importcpp: "dim3",
    header: "tnt_fortran_array3d.h".}
proc refCount*[T](this: FortranArray3D[T]): cint {.noSideEffect,
    importcpp: "ref_count", header: "tnt_fortran_array3d.h".}
proc destroyFortranArray3D*[T](this: var FortranArray3D[T]) {.
    importcpp: "#.~Fortran_Array3D()", header: "tnt_fortran_array3d.h".}
## !!!Ignored construct:  template < class T > [end of template] Fortran_Array3D < T > [end of template] :: Fortran_Array3D ( ) : v_ ( ) , m_ ( 0 ) , n_ ( 0 ) , k_ ( 0 ) , data_ ( 0 ) { } template < class T > Fortran_Array3D < T > :: Fortran_Array3D ( const Fortran_Array3D < T > & A ) : v_ ( A . v_ ) , m_ ( A . m_ ) , n_ ( A . n_ ) , k_ ( A . k_ ) , data_ ( A . data_ ) { } template < class T > Fortran_Array3D < T > :: Fortran_Array3D ( int m , int n , int k ) : v_ ( m * n * k ) , m_ ( m ) , n_ ( n ) , k_ ( k ) , data_ ( v_ . begin ( ) ) { } template < class T > Fortran_Array3D < T > :: Fortran_Array3D ( int m , int n , int k , const T & val ) : v_ ( m * n * k ) , m_ ( m ) , n_ ( n ) , k_ ( k ) , data_ ( v_ . begin ( ) ) { for ( T * p = data_ ; p < data_ + m * n * k ; p ++ ) * p = val ; } template < class T > Fortran_Array3D < T > :: Fortran_Array3D ( int m , int n , int k , T * a ) : v_ ( a ) , m_ ( m ) , n_ ( n ) , k_ ( k ) , data_ ( v_ . begin ( ) ) { } template < class T > inline T & Fortran_Array3D < T > :: operator ( ) ( int i , int j , int k ) { # TNT_BOUNDS_CHECK [NewLine] assert ( i >= 1 ) ; assert ( i <= m_ ) ; assert ( j >= 1 ) ; assert ( j <= n_ ) ; assert ( k >= 1 ) ; assert ( k <= k_ ) ; # [NewLine] return data_ [ ( k - 1 ) * m_ * n_ + ( j - 1 ) * m_ + i - 1 ] ; } template < class T > inline const T & Fortran_Array3D < T > :: operator ( ) ( int i , int j , int k ) const { # TNT_BOUNDS_CHECK [NewLine] assert ( i >= 1 ) ; assert ( i <= m_ ) ; assert ( j >= 1 ) ; assert ( j <= n_ ) ; assert ( k >= 1 ) ; assert ( k <= k_ ) ; # [NewLine] return data_ [ ( k - 1 ) * m_ * n_ + ( j - 1 ) * m_ + i - 1 ] ; } template < class T > Fortran_Array3D < T > & Fortran_Array3D < T > :: operator = ( const T & a ) { T * end = data_ + m_ * n_ * k_ ; for ( T * p = data_ ; p != end ; * p ++ = a ) ; return * this ; } template < class T > Fortran_Array3D < T > Fortran_Array3D < T > :: copy ( ) const { Fortran_Array3D B ( m_ , n_ , k_ ) ; B . inject ( * this ) ; return B ; } template < class T > Fortran_Array3D < T > & Fortran_Array3D < T > :: inject ( const Fortran_Array3D & A ) { if ( m_ == A . m_ && n_ == A . n_ && k_ == A . k_ ) { T * p = data_ ; T * end = data_ + m_ * n_ * k_ ; const T * q = A . data_ ; for ( ; p < end ; * p ++ = * q ++ ) ; } return * this ; } template < class T > Fortran_Array3D < T > & Fortran_Array3D < T > :: ref ( const Fortran_Array3D < T > & A ) { if ( this != & A ) { v_ = A . v_ ; m_ = A . m_ ; n_ = A . n_ ; k_ = A . k_ ; data_ = A . data_ ; } return * this ; } template < class T > Fortran_Array3D < T > & Fortran_Array3D < T > :: operator = ( const Fortran_Array3D < T > & A ) { return ref ( A ) ; } template < class T > inline int Fortran_Array3D < T > :: dim1 ( ) const { return m_ ; } template < class T > inline int Fortran_Array3D < T > :: dim2 ( ) const { return n_ ; } template < class T > inline int Fortran_Array3D < T > :: dim3 ( ) const { return k_ ; } template < class T > inline int Fortran_Array3D < T > :: ref_count ( ) const { return v_ . ref_count ( ) ; } template < class T > Fortran_Array3D < T > :: ~ Fortran_Array3D ( ) { } }
## Error: identifier expected, but got: )!!!

##  namespace TNT

##  TNT_FORTRAN_ARRAY3D_H

##
##
##  Template Numerical Toolkit (TNT)
##
##  Mathematical and Computational Sciences Division
##  National Institute of Technology,
##  Gaithersburg, MD USA
##
##
##  This software was developed at the National Institute of Standards and
##  Technology (NIST) by employees of the Federal Government in the course
##  of their official duties. Pursuant to title 17 Section 105 of the
##  United States Code, this software is not subject to copyright protection
##  and is in the public domain. NIST assumes no responsibility whatsoever for
##  its use by other parties, and makes no guarantees, expressed or implied,
##  about its quality, reliability, or any other characteristic.
##
##

proc `<<`*(s: var Ostream; a: FortranArray3D[T]): var Ostream {.importcpp: "(# << #)",
    header: "tnt_fortran_array3d_utils.h".}
proc `>>`*(s: var Istream; a: var FortranArray3D[T]): var Istream {.
    importcpp: "(# >> #)", header: "tnt_fortran_array3d_utils.h".}
proc `+`*(a: FortranArray3D[T]; b: FortranArray3D[T]): FortranArray3D[T] {.
    importcpp: "(# + #)", header: "tnt_fortran_array3d_utils.h".}
proc `-`*(a: FortranArray3D[T]; b: FortranArray3D[T]): FortranArray3D[T] {.
    importcpp: "(# - #)", header: "tnt_fortran_array3d_utils.h".}
proc `*`*(a: FortranArray3D[T]; b: FortranArray3D[T]): FortranArray3D[T] {.
    importcpp: "(# * #)", header: "tnt_fortran_array3d_utils.h".}
proc `/`*(a: FortranArray3D[T]; b: FortranArray3D[T]): FortranArray3D[T] {.
    importcpp: "(# / #)", header: "tnt_fortran_array3d_utils.h".}
proc `+=`*(a: var FortranArray3D[T]; b: FortranArray3D[T]) {.importcpp: "(# += #)",
    header: "tnt_fortran_array3d_utils.h".}
proc `-=`*(a: var FortranArray3D[T]; b: FortranArray3D[T]) {.importcpp: "(# -= #)",
    header: "tnt_fortran_array3d_utils.h".}
proc `*=`*(a: var FortranArray3D[T]; b: FortranArray3D[T]) {.importcpp: "(# *= #)",
    header: "tnt_fortran_array3d_utils.h".}
proc `/=`*(a: var FortranArray3D[T]; b: FortranArray3D[T]) {.importcpp: "(# /= #)",
    header: "tnt_fortran_array3d_utils.h".}
##  namespace TNT

##
##
##  Template Numerical Toolkit (TNT)
##
##  Mathematical and Computational Sciences Division
##  National Institute of Technology,
##  Gaithersburg, MD USA
##
##
##  This software was developed at the National Institute of Standards and
##  Technology (NIST) by employees of the Federal Government in the course
##  of their official duties. Pursuant to title 17 Section 105 of the
##  United States Code, this software is not subject to copyright protection
##  and is in the public domain. NIST assumes no responsibility whatsoever for
##  its use by other parties, and makes no guarantees, expressed or implied,
##  about its quality, reliability, or any other characteristic.
##
##

when defined(TNT_BOUNDS_CHECK):
  discard
when not defined(NULL):
  const
    NULL* = 0
##
## 	Internal representation of ref-counted array.  The TNT
## 	arrays all use this building block.
##
## 	<p>
## 	If an array block is created by TNT, then every time
## 	an assignment is made, the left-hand-side reference
## 	is decreased by one, and the right-hand-side refernce
## 	count is increased by one.  If the array block was
## 	external to TNT, the refernce count is a NULL pointer
## 	regardless of how many references are made, since the
## 	memory is not freed by TNT.
##
##
## 	
##

type
  IRefvec*[T] {.importcpp: "TNT::i_refvec<\'0>", header: "tnt_i_refvec.h", bycopy.} = object


proc constructIRefvec*[T](): IRefvec[T] {.constructor,
                                       importcpp: "TNT::i_refvec<\'*0>(@)",
                                       header: "tnt_i_refvec.h".}
proc constructIRefvec*[T](n: cint): IRefvec[T] {.constructor,
    importcpp: "TNT::i_refvec<\'*0>(@)", header: "tnt_i_refvec.h".}
proc constructIRefvec*[T](data: ptr T): IRefvec[T] {.constructor,
    importcpp: "TNT::i_refvec<\'*0>(@)", header: "tnt_i_refvec.h".}
proc constructIRefvec*[T](v: IRefvec): IRefvec[T] {.constructor,
    importcpp: "TNT::i_refvec<\'*0>(@)", header: "tnt_i_refvec.h".}
proc begin*[T](this: var IRefvec[T]): ptr T {.importcpp: "begin",
                                        header: "tnt_i_refvec.h".}
proc begin*[T](this: IRefvec[T]): ptr T {.noSideEffect, importcpp: "begin",
                                     header: "tnt_i_refvec.h".}
proc `[]`*[T](this: var IRefvec[T]; i: cint): var T {.importcpp: "#[@]",
    header: "tnt_i_refvec.h".}
proc `[]`*[T](this: IRefvec[T]; i: cint): T {.noSideEffect, importcpp: "#[@]",
                                        header: "tnt_i_refvec.h".}
proc copy*[T](this: var IRefvec[T]; p: ptr T; q: ptr T; e: ptr T) {.importcpp: "copy_",
    header: "tnt_i_refvec.h".}
proc set*[T](this: var IRefvec[T]; p: ptr T; b: ptr T; e: ptr T) {.importcpp: "set_",
    header: "tnt_i_refvec.h".}
proc refCount*[T](this: IRefvec[T]): cint {.noSideEffect, importcpp: "ref_count",
                                        header: "tnt_i_refvec.h".}
proc isNull*[T](this: IRefvec[T]): cint {.noSideEffect, importcpp: "is_null",
                                      header: "tnt_i_refvec.h".}
proc destroy*[T](this: var IRefvec[T]) {.importcpp: "destroy",
                                     header: "tnt_i_refvec.h".}
proc destroyIRefvec*[T](this: var IRefvec[T]) {.importcpp: "#.~i_refvec()",
    header: "tnt_i_refvec.h".}
## !!!Ignored construct:  template < class T > [end of template] void i_refvec < T > :: copy_ ( T * p , const T * q , const T * e ) { for ( T * t = p ; q < e ; t ++ , q ++ ) * t = * q ; } template < class T > i_refvec < T > :: i_refvec ( ) : data_ ( NULL ) , ref_count_ ( NULL ) { } *
## 	In case n is 0 or negative, it does NOT call new.
##  template < class T > i_refvec < T > :: i_refvec ( int n ) : data_ ( NULL ) , ref_count_ ( NULL ) { if ( n >= 1 ) { # TNT_DEBUG [NewLine] std :: cout << new data storage.
##  ; # [NewLine] data_ = new T [ n ] ; ref_count_ = new int ; * ref_count_ = 1 ; } } template < class T > inline i_refvec < T > :: i_refvec ( const i_refvec < T > & V ) : data_ ( V . data_ ) , ref_count_ ( V . ref_count_ ) { if ( V . ref_count_ != NULL ) ( * ( V . ref_count_ ) ) ++ ; } template < class T > i_refvec < T > :: i_refvec ( T * data ) : data_ ( data ) , ref_count_ ( NULL ) { } template < class T > inline T * i_refvec < T > :: begin ( ) { return data_ ; } template < class T > inline const T & i_refvec < T > :: operator [ ] ( int i ) const { return data_ [ i ] ; } template < class T > inline T & i_refvec < T > :: operator [ ] ( int i ) { return data_ [ i ] ; } template < class T > inline const T * i_refvec < T > :: begin ( ) const { return data_ ; } template < class T > i_refvec < T > & i_refvec < T > :: operator = ( const i_refvec < T > & V ) { if ( this == & V ) return * this ; if ( ref_count_ != NULL ) { ( * ref_count_ ) -- ; if ( ( * ref_count_ ) == 0 ) destroy ( ) ; } data_ = V . data_ ; ref_count_ = V . ref_count_ ; if ( V . ref_count_ != NULL ) ( * ( V . ref_count_ ) ) ++ ; return * this ; } template < class T > void i_refvec < T > :: destroy ( ) { if ( ref_count_ != NULL ) { # TNT_DEBUG [NewLine] std :: cout << destorying data...
##  ; # [NewLine] delete ref_count_ ; # TNT_DEBUG [NewLine] std :: cout << deleted ref_count_ ...
##  ; # [NewLine] if ( data_ != NULL ) delete [ ] data_ ; # TNT_DEBUG [NewLine] std :: cout << deleted data_[] ...
##  ; # [NewLine] data_ = NULL ; } }
##  return 1 is vector is empty, 0 otherwise
##
##  if is_null() is false and ref_count() is 0, then
##
##  template < class T > int i_refvec < T > :: is_null ( ) const { return ( data_ == NULL ? 1 : 0 ) ; }
##   returns -1 if data is external,
##   returns 0 if a is NULL array,
##   otherwise returns the positive number of vectors sharing
##   		this data space.
##  template < class T > int i_refvec < T > :: ref_count ( ) const { if ( data_ == NULL ) return 0 ; else return ( ref_count_ != NULL ? * ref_count_ : - 1 ) ; } template < class T > i_refvec < T > :: ~ i_refvec ( ) { if ( ref_count_ != NULL ) { ( * ref_count_ ) -- ; if ( * ref_count_ == 0 ) destroy ( ) ; } } }
## Error: token expected: ; but got: <!!!

##  namespace TNT

##  TNT_I_REFVEC_H

##  needed for fabs, sqrt() below

## *
## 	@returns hypotenuse of real (non-complex) scalars a and b by
## 	avoiding underflow/overflow
## 	using (a * sqrt( 1 + (b/a) * (b/a))), rather than
## 	sqrt(a*a + b*b).
##

proc hypot*[Real](a: Real; b: Real): Real =
  discard

##  TNT namespace

##  MATH_UTILS_H

##
##
##  Template Numerical Toolkit (TNT)
##
##  Mathematical and Computational Sciences Division
##  National Institute of Technology,
##  Gaithersburg, MD USA
##
##
##  This software was developed at the National Institute of Standards and
##  Technology (NIST) by employees of the Federal Government in the course
##  of their official duties. Pursuant to title 17 Section 105 of the
##  United States Code, this software is not subject to copyright protection
##  and is in the public domain. NIST assumes no responsibility whatsoever for
##  its use by other parties, and makes no guarantees, expressed or implied,
##  about its quality, reliability, or any other characteristic.
##
##

import
  tntArray1d

## *
## 	Read-only view of a sparse matrix in compressed-row storage
## 	format.  Neither array elements (nonzeros) nor sparsity
## 	structure can be modified.  If modifications are required,
## 	create a new view.
##
## 	<p>
## 	Index values begin at 0.
##
## 	<p>
## 	<b>Storage requirements:</b> An (m x n) matrix with
## 	nz nonzeros requires no more than  ((T+I)*nz + M*I)
## 	bytes, where T is the size of data elements and
## 	I is the size of integers.
##
##
##

type
  SparseMatrixCompRow*[T] {.importcpp: "TNT::Sparse_Matrix_CompRow<\'0>",
                           header: "tnt_sparse_matrix_csr.h", bycopy.} = object
    ##  data values (nz_ elements)
    ##  row_ptr (dim_[0]+1 elements)
    ##  col_ind  (nz_ elements)
    ##  number of rows
    ##  number of cols


proc constructSparseMatrixCompRow*[T](s: SparseMatrixCompRow): SparseMatrixCompRow[
    T] {.constructor, importcpp: "TNT::Sparse_Matrix_CompRow<\'*0>(@)",
        header: "tnt_sparse_matrix_csr.h".}
proc constructSparseMatrixCompRow*[T](m: cint; n: cint; nz: cint; val: ptr T; r: ptr cint;
                                     c: ptr cint): SparseMatrixCompRow[T] {.
    constructor, importcpp: "TNT::Sparse_Matrix_CompRow<\'*0>(@)",
    header: "tnt_sparse_matrix_csr.h".}
proc val*[T](this: SparseMatrixCompRow[T]; i: cint): T {.noSideEffect,
    importcpp: "val", header: "tnt_sparse_matrix_csr.h".}
proc rowPtr*[T](this: SparseMatrixCompRow[T]; i: cint): cint {.noSideEffect,
    importcpp: "row_ptr", header: "tnt_sparse_matrix_csr.h".}
proc colInd*[T](this: SparseMatrixCompRow[T]; i: cint): cint {.noSideEffect,
    importcpp: "col_ind", header: "tnt_sparse_matrix_csr.h".}
proc dim1*[T](this: SparseMatrixCompRow[T]): cint {.noSideEffect, importcpp: "dim1",
    header: "tnt_sparse_matrix_csr.h".}
proc dim2*[T](this: SparseMatrixCompRow[T]): cint {.noSideEffect, importcpp: "dim2",
    header: "tnt_sparse_matrix_csr.h".}
proc numNonzeros*[T](this: SparseMatrixCompRow[T]): cint {.noSideEffect,
    importcpp: "NumNonzeros", header: "tnt_sparse_matrix_csr.h".}
## *
## 	Construct a read-only view of existing sparse matrix in
## 	compressed-row storage format.
##
## 	@param M the number of rows of sparse matrix
## 	@param N the  number of columns of sparse matrix
## 	@param nz the number of nonzeros
## 	@param val a contiguous list of nonzero values
## 	@param r row-pointers: r[i] denotes the begining position of row i
## 		(i.e. the ith row begins at val[row[i]]).
## 	@param c column-indices: c[i] denotes the column location of val[i]
##

## !!!Ignored construct:  template < class T > [end of template] Sparse_Matrix_CompRow < T > [end of template] :: Sparse_Matrix_CompRow ( int M , int N , int nz , const T * val , const int * r , const int * c ) : val_ ( nz , val ) , rowptr_ ( M , ( int * ) r ) , colind_ ( nz , ( int * ) c ) , dim1_ ( M ) , dim2_ ( N ) { } }
## Error: token expected: :: but got: [identifier]!!!

##  namespace TNT

##
##
##  Mathematical and Computational Sciences Division
##  National Institute of Technology,
##  Gaithersburg, MD USA
##
##
##  This software was developed at the National Institute of Standards and
##  Technology (NIST) by employees of the Federal Government in the course
##  of their official duties. Pursuant to title 17 Section 105 of the
##  United States Code, this software is not subject to copyright protection
##  and is in the public domain.  NIST assumes no responsibility whatsoever for
##  its use by other parties, and makes no guarantees, expressed or implied,
##  about its quality, reliability, or any other characteristic.
##
##

##  for clock() and CLOCKS_PER_SEC

proc seconds*(): cdouble =
  discard

type
  Stopwatch* {.importcpp: "TNT::Stopwatch", header: "tnt_stopwatch.h", bycopy.} = object


proc constructStopwatch*(): Stopwatch {.constructor, importcpp: "TNT::Stopwatch(@)",
                                     header: "tnt_stopwatch.h".}
proc start*(this: var Stopwatch) {.importcpp: "start", header: "tnt_stopwatch.h".}
proc stop*(this: var Stopwatch): cdouble {.importcpp: "stop", header: "tnt_stopwatch.h".}
proc read*(this: var Stopwatch): cdouble {.importcpp: "read", header: "tnt_stopwatch.h".}
proc resume*(this: var Stopwatch) {.importcpp: "resume", header: "tnt_stopwatch.h".}
proc running*(this: var Stopwatch): cint {.importcpp: "running",
                                      header: "tnt_stopwatch.h".}
## !!!Ignored construct:  inline Stopwatch :: Stopwatch ( ) : running_ ( 0 ) , start_time_ ( 0.0 ) , total_ ( 0.0 ) { } void Stopwatch :: start ( ) { running_ = 1 ; total_ = 0.0 ; start_time_ = seconds ( ) ; } double Stopwatch :: stop ( ) { if ( running_ ) { total_ += ( seconds ( ) - start_time_ ) ; running_ = 0 ; } return total_ ; } inline void Stopwatch :: resume ( ) { if ( ! running_ ) { start_time_ = seconds ( ) ; running_ = 1 ; } } inline double Stopwatch :: read ( ) { if ( running_ ) { stop ( ) ; resume ( ) ; } return total_ ; } }
## Error: identifier expected, but got: )!!!

##  TNT namespace

##
##
##  Template Numerical Toolkit (TNT)
##
##  Mathematical and Computational Sciences Division
##  National Institute of Technology,
##  Gaithersburg, MD USA
##
##
##  This software was developed at the National Institute of Standards and
##  Technology (NIST) by employees of the Federal Government in the course
##  of their official duties. Pursuant to title 17 Section 105 of the
##  United States Code, this software is not subject to copyright protection
##  and is in the public domain. NIST assumes no responsibility whatsoever for
##  its use by other parties, and makes no guarantees, expressed or implied,
##  about its quality, reliability, or any other characteristic.
##
##

## ---------------------------------------------------------------------
##  This definition describes the default TNT data type used for
##  indexing into TNT matrices and vectors.  The data type should
##  be wide enough to index into large arrays.  It defaults to an
##  "int", but can be overriden at compile time redefining TNT_SUBSCRIPT_TYPE,
##  e.g.
##
##       c++ -DTNT_SUBSCRIPT_TYPE='unsigned int'  ...
##
## ---------------------------------------------------------------------
##

when not defined(TNT_SUBSCRIPT_TYPE):
  const
    TNT_SUBSCRIPT_TYPE* = int
type
  Subscript* = Tnt_Subscript_Type

##  namespace TNT
##  () indexing in TNT means 1-offset, i.e. x(1) and A(1,1) are the
##  first elements.  This offset is left as a macro for future
##  purposes, but should not be changed in the current release.
##
##

const
  TNT_BASE_OFFSET* = (1)

##
##
##  Template Numerical Toolkit (TNT)
##
##  Mathematical and Computational Sciences Division
##  National Institute of Technology,
##  Gaithersburg, MD USA
##
##
##  This software was developed at the National Institute of Standards and
##  Technology (NIST) by employees of the Federal Government in the course
##  of their official duties. Pursuant to title 17 Section 105 of the
##  United States Code, this software is not subject to copyright protection
##  and is in the public domain. NIST assumes no responsibility whatsoever for
##  its use by other parties, and makes no guarantees, expressed or implied,
##  about its quality, reliability, or any other characteristic.
##
##

import
  tntSubscript

## *
##  <b>[Deprecatred]</b>  Value-based vector class from pre-1.0
##  	TNT version.  Kept here for backward compatiblity, but should
## 	use the newer TNT::Array1D classes instead.
##
##

type
  Vector*[T] {.importcpp: "TNT::Vector<\'0>", header: "tnt_vec.h", bycopy.} = object ##  access
    ##  pointer adjustment for optimzied 1-offset indexing
    ##  internal helper function to create the array
    ##  of row pointers

  VectorsizeType* = Subscript
  VectorvalueType*[T] = T
  VectorelementType*[T] = T
  Vectorpointer*[T] = ptr T
  Vectoriterator*[T] = ptr T
  Vectorreference*[T] = var T
  VectorconstIterator*[T] = ptr T
  VectorconstReference*[T] = T

proc lbound*[T](this: Vector[T]): Subscript {.noSideEffect, importcpp: "lbound",
    header: "tnt_vec.h".}
proc begin*[T](this: var Vector[T]): Vectoriterator {.importcpp: "begin",
    header: "tnt_vec.h".}
proc `end`*[T](this: var Vector[T]): Vectoriterator {.importcpp: "end",
    header: "tnt_vec.h".}
proc begin*[T](this: Vector[T]): Vectoriterator {.noSideEffect, importcpp: "begin",
    header: "tnt_vec.h".}
proc `end`*[T](this: Vector[T]): Vectoriterator {.noSideEffect, importcpp: "end",
    header: "tnt_vec.h".}
proc destroyVector*[T](this: var Vector[T]) {.importcpp: "#.~Vector()",
    header: "tnt_vec.h".}
proc constructVector*[T](): Vector[T] {.constructor,
                                     importcpp: "TNT::Vector<\'*0>(@)",
                                     header: "tnt_vec.h".}
proc constructVector*[T](a: Vector[T]): Vector[T] {.constructor,
    importcpp: "TNT::Vector<\'*0>(@)", header: "tnt_vec.h".}
proc constructVector*[T](n: Subscript; value: T = t()): Vector[T] {.constructor,
    importcpp: "TNT::Vector<\'*0>(@)", header: "tnt_vec.h".}
proc constructVector*[T](n: Subscript; v: ptr T): Vector[T] {.constructor,
    importcpp: "TNT::Vector<\'*0>(@)", header: "tnt_vec.h".}
proc constructVector*[T](n: Subscript; s: cstring): Vector[T] {.constructor,
    importcpp: "TNT::Vector<\'*0>(@)", header: "tnt_vec.h".}
proc newsize*[T](this: var Vector[T]; n: Subscript): var Vector[T] {.
    importcpp: "newsize", header: "tnt_vec.h".}
proc dim*[T](this: Vector[T]): Subscript {.noSideEffect, importcpp: "dim",
                                       header: "tnt_vec.h".}
proc size*[T](this: Vector[T]): Subscript {.noSideEffect, importcpp: "size",
                                        header: "tnt_vec.h".}
proc `()`*[T](this: var Vector[T]; i: Subscript): Vectorreference {.importcpp: "#(@)",
    header: "tnt_vec.h".}
proc `()`*[T](this: Vector[T]; i: Subscript): VectorconstReference {.noSideEffect,
    importcpp: "#(@)", header: "tnt_vec.h".}
proc `[]`*[T](this: var Vector[T]; i: Subscript): Vectorreference {.importcpp: "#[@]",
    header: "tnt_vec.h".}
proc `[]`*[T](this: Vector[T]; i: Subscript): VectorconstReference {.noSideEffect,
    importcpp: "#[@]", header: "tnt_vec.h".}
##  ***************************  I/O  *******************************

proc `<<`*(s: var Ostream; a: Vector[T]): var Ostream {.importcpp: "(# << #)",
    header: "tnt_vec.h".}
proc `>>`*(s: var Istream; a: var Vector[T]): var Istream {.importcpp: "(# >> #)",
    header: "tnt_vec.h".}
##  *******************[ basic matrix algorithms ]***************************

proc `+`*(a: Vector[T]; b: Vector[T]): Vector[T] {.importcpp: "(# + #)",
    header: "tnt_vec.h".}
proc `-`*(a: Vector[T]; b: Vector[T]): Vector[T] {.importcpp: "(# - #)",
    header: "tnt_vec.h".}
proc `*`*(a: Vector[T]; b: Vector[T]): Vector[T] {.importcpp: "(# * #)",
    header: "tnt_vec.h".}
proc dotProd*[T](a: Vector[T]; b: Vector[T]): T =
  discard

##  namespace TNT

##  TNT_VEC_H

##
##
##  Template Numerical Toolkit (TNT)
##
##  Mathematical and Computational Sciences Division
##  National Institute of Technology,
##  Gaithersburg, MD USA
##
##
##  This software was developed at the National Institute of Standards and
##  Technology (NIST) by employees of the Federal Government in the course
##  of their official duties. Pursuant to title 17 Section 105 of the
##  United States Code, this software is not subject to copyright protection
##  and is in the public domain. NIST assumes no responsibility whatsoever for
##  its use by other parties, and makes no guarantees, expressed or implied,
##  about its quality, reliability, or any other characteristic.
##
##

## ---------------------------------------------------------------------
##   current version
## ---------------------------------------------------------------------

const
  TNT_MAJOR_VERSION* = '1'
  TNT_MINOR_VERSION* = '2'
  TNT_SUBMINOR_VERSION* = '6'
  TNT_VERSION_STRING* = "1.2.6"

##  TNT_VERSION_H
