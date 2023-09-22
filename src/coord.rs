use crate::avx2_coord;
use crate::cube::{Corner, Edge};
use crate::cubie::{CornerCubieCube, CubieCube, EdgeCubieCube};

pub trait Coord<const SIZE: usize>: Into<usize> + Copy + Clone + Eq + PartialEq{
    fn size() -> usize {
        SIZE
    }
    fn val(&self) -> usize;
}

//Edge orientation on the respective axis
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct EOCoordAll(pub EOCoordUD, pub EOCoordFB, pub EOCoordLR);
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct EOCoordUD(pub(crate) u16);
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct EOCoordFB(pub(crate) u16);
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct EOCoordLR(pub(crate) u16);

//EO without considering edges in the UD slice (because they are already oriented)
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct EOCoordNoUDSlice(pub(crate) u8);

//UD corner orientation
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct COUDCoord(pub(crate) u16);

//Corner permutation
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CPCoord(pub(crate) u16);

//Edge permutation
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct EPCoord(pub(crate) u32);

//Coordinate representing the position of edges that belong into the UD slice.
//0 if they are in the slice, they don't have to be in the correct position
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct UDSliceUnsortedCoord(pub(crate) u16);

//Assuming we already have FB-EO, represents the combination of UDSliceUnsortedCoord and COUDCoord
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct DRUDEOFBCoord(pub(crate) u32);

//Coordinate representing the position of edges that belong into the FB slice, assuming the UD slice is already correct.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FBSliceUnsortedCoord(pub(crate) u8);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CPOrbitUnsortedCoord(pub(crate) u8);

//Coordinate representing the twist state of HTR corner orbits
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CPOrbitTwistParityCoord(pub(crate) u8);

//Coordinate representing the parity state
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ParityCoord(pub(crate) bool);

//Assuming we already have UD-DR, represents the combination of UDSliceUnsortedCoord and COUDCoord
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct HTRDRUDCoord(pub(crate) u32);

impl Coord<2048> for EOCoordUD {
    fn val(&self) -> usize {
        self.0 as usize
    }
}

impl Into<usize> for EOCoordUD {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl Coord<2048> for EOCoordFB {
    fn val(&self) -> usize {
        self.0 as usize
    }
}

impl Into<usize> for EOCoordFB {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl Coord<2048> for EOCoordLR {
    fn val(&self) -> usize {
        self.0 as usize
    }
}

impl Into<usize> for EOCoordLR {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl Coord<128> for EOCoordNoUDSlice {
    fn val(&self) -> usize {
        self.0 as usize
    }
}

impl Into<usize> for EOCoordNoUDSlice {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl Coord<2187> for COUDCoord {
    fn val(&self) -> usize {
        self.0 as usize
    }
}

impl Into<usize> for COUDCoord {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl Coord<40320> for CPCoord {
    fn val(&self) -> usize {
        self.0 as usize
    }
}

impl Into<usize> for CPCoord {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl Coord<479001600> for EPCoord {
    fn val(&self) -> usize {
        self.0 as usize
    }
}

impl Into<usize> for EPCoord {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl Coord<495> for UDSliceUnsortedCoord {
    fn val(&self) -> usize {
        self.0 as usize
    }
}

impl Into<usize> for UDSliceUnsortedCoord {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl Coord<70> for FBSliceUnsortedCoord {
    fn val(&self) -> usize {
        self.0 as usize
    }
}

impl Into<usize> for FBSliceUnsortedCoord {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl Coord<70> for CPOrbitUnsortedCoord {
    fn val(&self) -> usize {
        self.0 as usize
    }
}

impl Into<usize> for CPOrbitUnsortedCoord {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl Coord<3> for CPOrbitTwistParityCoord {
    fn val(&self) -> usize {
        self.0 as usize
    }
}

impl Into<usize> for CPOrbitTwistParityCoord {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl Coord<2> for ParityCoord {
    fn val(&self) -> usize {
        self.0 as usize
    }
}

impl Into<usize> for ParityCoord {
    fn into(self) -> usize {
        self.0 as usize
    }
}

//TODO this should use 'impl const' once it's stable
pub const DRUDEOFB_SIZE: usize = 495 * 2187;
impl Coord<DRUDEOFB_SIZE> for DRUDEOFBCoord {
    fn val(&self) -> usize {
        self.0 as usize
    }
}

impl Into<usize> for DRUDEOFBCoord {
    fn into(self) -> usize {
        self.val()
    }
}

//TODO this should use 'impl const' once it's stable
pub const HTRDRUD_SIZE: usize = 70 * 70 * 6;
impl Coord<HTRDRUD_SIZE> for HTRDRUDCoord {
    fn val(&self) -> usize {
        self.0 as usize
    }
}

impl Into<usize> for HTRDRUDCoord {
    fn into(self) -> usize {
        self.val()
    }
}

impl From<&EdgeCubieCube> for EOCoordAll {

    #[inline]
    #[cfg(target_feature = "avx2")]
    fn from(value: &EdgeCubieCube) -> Self {
        unsafe {
            avx2_coord::avx2_coord::unsafe_from_eocoord_all(value)
        }
    }
}

impl From<&EdgeCubieCube> for EOCoordUD {

    #[inline]
    #[cfg(target_feature = "avx2")]
    fn from(value: &EdgeCubieCube) -> Self {
        unsafe {
            avx2_coord::avx2_coord::unsafe_from_eocoord_ud(value)
        }
    }
}

impl From<&CubieCube> for EOCoordUD {
    fn from(value: &CubieCube) -> Self {
        EOCoordUD::from(&value.edges)
    }
}

impl From<&EdgeCubieCube> for EOCoordFB {

    #[inline]
    #[cfg(target_feature = "avx2")]
    fn from(value: &EdgeCubieCube) -> Self {
        unsafe {
            avx2_coord::avx2_coord::unsafe_from_eocoord_fb(value)
        }
    }
}

impl From<&CubieCube> for EOCoordFB {
    fn from(value: &CubieCube) -> Self {
        EOCoordFB::from(&value.edges)
    }
}

impl From<&EdgeCubieCube> for EOCoordLR {

    #[inline]
    #[cfg(target_feature = "avx2")]
    fn from(value: &EdgeCubieCube) -> Self {
        unsafe {
            avx2_coord::avx2_coord::unsafe_from_eocoord_lr(value)
        }
    }
}

impl From<&CubieCube> for EOCoordLR {
    fn from(value: &CubieCube) -> Self {
        EOCoordLR::from(&value.edges)
    }
}

impl From<&EdgeCubieCube> for EOCoordNoUDSlice {

    #[inline]
    #[cfg(target_feature = "avx2")]
    fn from(value: &EdgeCubieCube) -> Self {
        unsafe {
            avx2_coord::avx2_coord::unsafe_from_eocoord_no_ud_slice(value)
        }
    }
}

impl From<&[Edge; 12]> for EOCoordAll {
    fn from(value: &[Edge; 12]) -> Self {
        let mut ud = 0_u16;
        let mut fb = 0_u16;
        let mut lr = 0_u16;

        for i in (0..11).rev() {
            let edge = value[i];
            ud = (ud << 1) | (!edge.oriented_ud as u16);
            fb = (fb << 1) | (!edge.oriented_fb as u16);
            lr = (lr << 1) | (!edge.oriented_rl as u16);
        }

        EOCoordAll(EOCoordUD(ud), EOCoordFB(fb), EOCoordLR(lr))
    }
}

impl From<&CornerCubieCube> for COUDCoord {

    #[inline]
    #[cfg(target_feature = "avx2")]
    fn from(value: &CornerCubieCube) -> Self {
        unsafe {
            avx2_coord::avx2_coord::unsafe_from_cocoord(value)
        }
    }
}

impl From<&[Corner; 8]> for COUDCoord {
    fn from(value: &[Corner; 8]) -> Self {
        let mut co = 0_u16;

        for i in (0..7).rev() {
            co = co * 3 + value[i].orientation as u16;
        }

        COUDCoord(co)
    }
}

impl From<&CornerCubieCube> for CPCoord {

    #[inline]
    #[cfg(target_feature = "avx2")]
    fn from(value: &CornerCubieCube) -> Self {
        unsafe {
            avx2_coord::avx2_coord::unsafe_from_cpcoord(value)
        }
    }
}

impl From<&[Corner; 8]> for CPCoord {
    fn from(value: &[Corner; 8]) -> Self {
        let mut cp = 0_u16;
        let factorial = [1, 2, 6, 24, 120, 720, 5040];

        for i in 1..8 {
            let mut higher = 0;
            for j in 0..i {
                if value[i].id < value[j].id {
                    higher += 1;
                }
            }
            cp += factorial[i - 1] * higher;
        }
        CPCoord(cp)
    }
}

impl From<&EdgeCubieCube> for EPCoord {

    #[inline]
    #[cfg(target_feature = "avx2")]
    fn from(value: &EdgeCubieCube) -> Self {
        unsafe {
            avx2_coord::avx2_coord::unsafe_from_epcoord(value)
        }
    }
}

impl From<&[Edge; 12]> for EPCoord {
    fn from(value: &[Edge; 12]) -> Self {
        let mut ep = 0_u32;
        let factorial = [1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800, 39916800];

        for i in 1..12 {
            let mut higher = 0;
            for j in 0..i {
                if value[i].id < value[j].id {
                    higher += 1;
                }
            }
            ep += factorial[i - 1] * higher;
        }
        EPCoord(ep)
    }
}

impl From<&EdgeCubieCube> for UDSliceUnsortedCoord {

    #[inline]
    #[cfg(target_feature = "avx2")]
    fn from(value: &EdgeCubieCube) -> Self {
        unsafe {
            avx2_coord::avx2_coord::unsafe_from_udslice_unsorted_coord(value)
        }
    }
}

impl From<&CubieCube> for DRUDEOFBCoord {
    #[inline]
    fn from(value: &CubieCube) -> Self {
        let ud_slice = UDSliceUnsortedCoord::from(&value.edges).val();
        let co = COUDCoord::from(&value.corners).val();
        let index =
            co * UDSliceUnsortedCoord::size() +
            ud_slice;
        DRUDEOFBCoord(index as u32)
    }
}

impl From<&EdgeCubieCube> for FBSliceUnsortedCoord {

    #[inline]
    #[cfg(target_feature = "avx2")]
    fn from(value: &EdgeCubieCube) -> Self {
        unsafe {
            avx2_coord::avx2_coord::unsafe_from_fbslice_unsorted_coord(value)
        }
    }
}

impl From<&CornerCubieCube> for CPOrbitUnsortedCoord {

    #[inline]
    #[cfg(target_feature = "avx2")]
    fn from(value: &CornerCubieCube) -> Self {
        unsafe {
            avx2_coord::avx2_coord::unsafe_from_cp_orbit_unsorted_coord(value)
        }
    }
}

impl From<&CornerCubieCube> for CPOrbitTwistParityCoord {

    #[inline]
    #[cfg(target_feature = "avx2")]
    fn from(value: &CornerCubieCube) -> Self {
        unsafe {
            avx2_coord::avx2_coord::unsafe_from_cp_orbit_twist_parity_coord(value)
        }
    }
}

impl From<&CornerCubieCube> for ParityCoord {

    #[inline]
    #[cfg(target_feature = "avx2")]
    fn from(value: &CornerCubieCube) -> Self {
        unsafe {
            avx2_coord::avx2_coord::unsafe_from_parity_coord(value)
        }
    }
}

impl From<&CubieCube> for HTRDRUDCoord {
    fn from(value: &CubieCube) -> Self {
        let ep_fbslice_coord = FBSliceUnsortedCoord::from(&value.edges).val();
        let cp_orbit_coord = CPOrbitUnsortedCoord::from(&value.corners).val();
        let cp_orbit_twist = CPOrbitTwistParityCoord::from(&value.corners).val();
        let parity = ParityCoord::from(&value.corners).val();

        let val =
            parity +
            cp_orbit_twist * ParityCoord::size() +
            cp_orbit_coord * ParityCoord::size() * CPOrbitTwistParityCoord::size() +
            ep_fbslice_coord * ParityCoord::size() * CPOrbitTwistParityCoord::size() * CPOrbitUnsortedCoord::size();
        HTRDRUDCoord(val as u32)
    }
}