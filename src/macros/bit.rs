macro_rules! BinRec [
    ( [ $acc:ty ] [  ] ) => ( Ph!($acc) );
    ( [ $acc:ty ] [ 0 $(,$bs:tt)* ] ) => ( BinRec!([ ($acc, $crate::bit::_0) ] [ $($bs),* ]) );
    ( [ $acc:ty ] [ 1 $(,$bs:tt)* ] ) => ( BinRec!([ ($acc, $crate::bit::_1) ] [ $($bs),* ]) );
];

macro_rules! B [
    ( 0 $(,$bs:tt)* ) => ( BinRec!([ $crate::bit::_0 ] [ $($bs),* ]) );
    ( 1 $(,$bs:tt)* ) => ( BinRec!([ $crate::bit::_1 ] [ $($bs),* ]) );
];

macro_rules! bin_rec [
    ( [ $acc:ty ] [  ] ) => ( ph!($acc) );
    ( [ $acc:ty ] [ 0 $(,$bs:tt)* ] ) => ( bin_rec!([ ($acc, $crate::bit::_0) ] [ $($bs),* ]) );
    ( [ $acc:ty ] [ 1 $(,$bs:tt)* ] ) => ( bin_rec!([ ($acc, $crate::bit::_1) ] [ $($bs),* ]) );
];

macro_rules! b [
    ( .. ) => ( ph!() );
    ( 0 $(,$bs:tt)* ) => ( bin_rec!([ $crate::bit::_0 ] [ $($bs),* ]) );
    ( 1 $(,$bs:tt)* ) => ( bin_rec!([ $crate::bit::_1 ] [ $($bs),* ]) );
];
