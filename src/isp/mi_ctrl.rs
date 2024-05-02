#[doc = "Register `MI_CTRL` reader"]
pub type R = crate::R<MiCtrlSpec>;
#[doc = "Register `MI_CTRL` writer"]
pub type W = crate::W<MiCtrlSpec>;
#[doc = "Enables data pathes of MI according to the following\n\ntable:\n\n\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PathEnable {
    #[doc = "0: disabled, no data is transferred"]
    B0000 = 0,
    #[doc = "1: YUV data output, mainpath only (mp_enable only) 0010: self-path only, output data format depending on other settings (sp_enable only)"]
    B0001 = 1,
    #[doc = "3: YUV data output in mainpath and self-path image data active"]
    B0011 = 3,
    #[doc = "4: JPEG data output, mainpath only (jpeg_enable only) 0101: not allowed"]
    B0100 = 4,
    #[doc = "6: JPEG data output in mainpath and self-path image data active"]
    B0110 = 6,
    #[doc = "7: not allowed"]
    B0111 = 7,
    #[doc = "8: RAW data output, mainpath only (raw_enable only) 1001: defect pixel data on self-path, image data on mainpath 1010: defect pixel data on mainpath, image data on self-path 1011: not allowed"]
    B1000 = 8,
    #[doc = "12: defect pixel data on self-path, JPEG data on mainpath 1101: defect pixel data on mainpath only"]
    B1100 = 12,
    #[doc = "14: defect pixel data on self-path only"]
    B1110 = 14,
    #[doc = "15: defect pixel data on self-path, RAW data on mainpath Programmed value becomes effective (visible in shadow register) after a soft reset, a forced software update or an automatic config update. Affects MI_IN and MI_OUT module."]
    B1111 = 15,
}
impl From<PathEnable> for u8 {
    #[inline(always)]
    fn from(variant: PathEnable) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PathEnable {
    type Ux = u8;
}
#[doc = "Field `path_enable` reader - Enables data pathes of MI according to the following\n\ntable:\n\n"]
pub type PathEnableR = crate::FieldReader<PathEnable>;
impl PathEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PathEnable> {
        match self.bits {
            0 => Some(PathEnable::B0000),
            1 => Some(PathEnable::B0001),
            3 => Some(PathEnable::B0011),
            4 => Some(PathEnable::B0100),
            6 => Some(PathEnable::B0110),
            7 => Some(PathEnable::B0111),
            8 => Some(PathEnable::B1000),
            12 => Some(PathEnable::B1100),
            14 => Some(PathEnable::B1110),
            15 => Some(PathEnable::B1111),
            _ => None,
        }
    }
    #[doc = "disabled, no data is transferred"]
    #[inline(always)]
    pub fn is_b0000(&self) -> bool {
        *self == PathEnable::B0000
    }
    #[doc = "YUV data output, mainpath only (mp_enable only) 0010: self-path only, output data format depending on other settings (sp_enable only)"]
    #[inline(always)]
    pub fn is_b0001(&self) -> bool {
        *self == PathEnable::B0001
    }
    #[doc = "YUV data output in mainpath and self-path image data active"]
    #[inline(always)]
    pub fn is_b0011(&self) -> bool {
        *self == PathEnable::B0011
    }
    #[doc = "JPEG data output, mainpath only (jpeg_enable only) 0101: not allowed"]
    #[inline(always)]
    pub fn is_b0100(&self) -> bool {
        *self == PathEnable::B0100
    }
    #[doc = "JPEG data output in mainpath and self-path image data active"]
    #[inline(always)]
    pub fn is_b0110(&self) -> bool {
        *self == PathEnable::B0110
    }
    #[doc = "not allowed"]
    #[inline(always)]
    pub fn is_b0111(&self) -> bool {
        *self == PathEnable::B0111
    }
    #[doc = "RAW data output, mainpath only (raw_enable only) 1001: defect pixel data on self-path, image data on mainpath 1010: defect pixel data on mainpath, image data on self-path 1011: not allowed"]
    #[inline(always)]
    pub fn is_b1000(&self) -> bool {
        *self == PathEnable::B1000
    }
    #[doc = "defect pixel data on self-path, JPEG data on mainpath 1101: defect pixel data on mainpath only"]
    #[inline(always)]
    pub fn is_b1100(&self) -> bool {
        *self == PathEnable::B1100
    }
    #[doc = "defect pixel data on self-path only"]
    #[inline(always)]
    pub fn is_b1110(&self) -> bool {
        *self == PathEnable::B1110
    }
    #[doc = "defect pixel data on self-path, RAW data on mainpath Programmed value becomes effective (visible in shadow register) after a soft reset, a forced software update or an automatic config update. Affects MI_IN and MI_OUT module."]
    #[inline(always)]
    pub fn is_b1111(&self) -> bool {
        *self == PathEnable::B1111
    }
}
#[doc = "Field `path_enable` writer - Enables data pathes of MI according to the following\n\ntable:\n\n"]
pub type PathEnableW<'a, REG> = crate::FieldWriter<'a, REG, 4, PathEnable>;
impl<'a, REG> PathEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disabled, no data is transferred"]
    #[inline(always)]
    pub fn b0000(self) -> &'a mut crate::W<REG> {
        self.variant(PathEnable::B0000)
    }
    #[doc = "YUV data output, mainpath only (mp_enable only) 0010: self-path only, output data format depending on other settings (sp_enable only)"]
    #[inline(always)]
    pub fn b0001(self) -> &'a mut crate::W<REG> {
        self.variant(PathEnable::B0001)
    }
    #[doc = "YUV data output in mainpath and self-path image data active"]
    #[inline(always)]
    pub fn b0011(self) -> &'a mut crate::W<REG> {
        self.variant(PathEnable::B0011)
    }
    #[doc = "JPEG data output, mainpath only (jpeg_enable only) 0101: not allowed"]
    #[inline(always)]
    pub fn b0100(self) -> &'a mut crate::W<REG> {
        self.variant(PathEnable::B0100)
    }
    #[doc = "JPEG data output in mainpath and self-path image data active"]
    #[inline(always)]
    pub fn b0110(self) -> &'a mut crate::W<REG> {
        self.variant(PathEnable::B0110)
    }
    #[doc = "not allowed"]
    #[inline(always)]
    pub fn b0111(self) -> &'a mut crate::W<REG> {
        self.variant(PathEnable::B0111)
    }
    #[doc = "RAW data output, mainpath only (raw_enable only) 1001: defect pixel data on self-path, image data on mainpath 1010: defect pixel data on mainpath, image data on self-path 1011: not allowed"]
    #[inline(always)]
    pub fn b1000(self) -> &'a mut crate::W<REG> {
        self.variant(PathEnable::B1000)
    }
    #[doc = "defect pixel data on self-path, JPEG data on mainpath 1101: defect pixel data on mainpath only"]
    #[inline(always)]
    pub fn b1100(self) -> &'a mut crate::W<REG> {
        self.variant(PathEnable::B1100)
    }
    #[doc = "defect pixel data on self-path only"]
    #[inline(always)]
    pub fn b1110(self) -> &'a mut crate::W<REG> {
        self.variant(PathEnable::B1110)
    }
    #[doc = "defect pixel data on self-path, RAW data on mainpath Programmed value becomes effective (visible in shadow register) after a soft reset, a forced software update or an automatic config update. Affects MI_IN and MI_OUT module."]
    #[inline(always)]
    pub fn b1111(self) -> &'a mut crate::W<REG> {
        self.variant(PathEnable::B1111)
    }
}
#[doc = "Horizontal flipping of self picture. For picture\n\norientation and operation modes see sub-chapter 'Picture\n\nOrientation' in chapter 'Self Path Output Programming'.\n\nFor Y component the line length in 4:2:x planar mode\n\nmust be a multiple of 8, for all other component modes a\n\nmultiple of 4 and for RGB 565 a multiple of 2. There are no\n\nrestrictions for RGB 888/666.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HFlip {
    #[doc = "1: enabled"]
    B1 = 1,
    #[doc = "0: disabled note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the self path."]
    B0 = 0,
}
impl From<HFlip> for bool {
    #[inline(always)]
    fn from(variant: HFlip) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `h_flip` reader - Horizontal flipping of self picture. For picture\n\norientation and operation modes see sub-chapter 'Picture\n\nOrientation' in chapter 'Self Path Output Programming'.\n\nFor Y component the line length in 4:2:x planar mode\n\nmust be a multiple of 8, for all other component modes a\n\nmultiple of 4 and for RGB 565 a multiple of 2. There are no\n\nrestrictions for RGB 888/666."]
pub type HFlipR = crate::BitReader<HFlip>;
impl HFlipR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HFlip {
        match self.bits {
            true => HFlip::B1,
            false => HFlip::B0,
        }
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HFlip::B1
    }
    #[doc = "disabled note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the self path."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HFlip::B0
    }
}
#[doc = "Field `h_flip` writer - Horizontal flipping of self picture. For picture\n\norientation and operation modes see sub-chapter 'Picture\n\nOrientation' in chapter 'Self Path Output Programming'.\n\nFor Y component the line length in 4:2:x planar mode\n\nmust be a multiple of 8, for all other component modes a\n\nmultiple of 4 and for RGB 565 a multiple of 2. There are no\n\nrestrictions for RGB 888/666."]
pub type HFlipW<'a, REG> = crate::BitWriter<'a, REG, HFlip>;
impl<'a, REG> HFlipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HFlip::B1)
    }
    #[doc = "disabled note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the self path."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HFlip::B0)
    }
}
#[doc = "Vertical flipping of self picture. For picture orientation\n\nand operation modes see sub-chapter 'Picture\n\nOrientation' in chapter 'Self Path Output Programming'.\n\nFor Y component the line length in 4:2:x planar mode\n\nmust be a multiple of 8, for all other component modes a\n\nmultiple of 4 and for RGB 565 a multiple of 2. There are no\n\nrestrictions for RGB 888/666.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VFlip {
    #[doc = "1: enabled"]
    B1 = 1,
    #[doc = "0: disabled note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the self path."]
    B0 = 0,
}
impl From<VFlip> for bool {
    #[inline(always)]
    fn from(variant: VFlip) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `v_flip` reader - Vertical flipping of self picture. For picture orientation\n\nand operation modes see sub-chapter 'Picture\n\nOrientation' in chapter 'Self Path Output Programming'.\n\nFor Y component the line length in 4:2:x planar mode\n\nmust be a multiple of 8, for all other component modes a\n\nmultiple of 4 and for RGB 565 a multiple of 2. There are no\n\nrestrictions for RGB 888/666."]
pub type VFlipR = crate::BitReader<VFlip>;
impl VFlipR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VFlip {
        match self.bits {
            true => VFlip::B1,
            false => VFlip::B0,
        }
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VFlip::B1
    }
    #[doc = "disabled note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the self path."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VFlip::B0
    }
}
#[doc = "Field `v_flip` writer - Vertical flipping of self picture. For picture orientation\n\nand operation modes see sub-chapter 'Picture\n\nOrientation' in chapter 'Self Path Output Programming'.\n\nFor Y component the line length in 4:2:x planar mode\n\nmust be a multiple of 8, for all other component modes a\n\nmultiple of 4 and for RGB 565 a multiple of 2. There are no\n\nrestrictions for RGB 888/666."]
pub type VFlipW<'a, REG> = crate::BitWriter<'a, REG, VFlip>;
impl<'a, REG> VFlipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VFlip::B1)
    }
    #[doc = "disabled note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the self path."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VFlip::B0)
    }
}
#[doc = "Rotation 90 degree counter clockwise of self picture,\n\nonly in RGB mode. For picture orientation and operation\n\nmodes see sub-chapter 'Picture Orientation' in chapter\n\n'Self Path Output Programming'.\n\nFor RGB 565 format the line length must be a multiple\n\nof 2. There are no restrictions for RGB 888/666.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rot {
    #[doc = "1: enabled"]
    B1 = 1,
    #[doc = "0: disabled note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the self path. In rotation mode only 4-beat bursts are supported for self- path."]
    B0 = 0,
}
impl From<Rot> for bool {
    #[inline(always)]
    fn from(variant: Rot) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `rot` reader - Rotation 90 degree counter clockwise of self picture,\n\nonly in RGB mode. For picture orientation and operation\n\nmodes see sub-chapter 'Picture Orientation' in chapter\n\n'Self Path Output Programming'.\n\nFor RGB 565 format the line length must be a multiple\n\nof 2. There are no restrictions for RGB 888/666."]
pub type RotR = crate::BitReader<Rot>;
impl RotR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rot {
        match self.bits {
            true => Rot::B1,
            false => Rot::B0,
        }
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rot::B1
    }
    #[doc = "disabled note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the self path. In rotation mode only 4-beat bursts are supported for self- path."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rot::B0
    }
}
#[doc = "Field `rot` writer - Rotation 90 degree counter clockwise of self picture,\n\nonly in RGB mode. For picture orientation and operation\n\nmodes see sub-chapter 'Picture Orientation' in chapter\n\n'Self Path Output Programming'.\n\nFor RGB 565 format the line length must be a multiple\n\nof 2. There are no restrictions for RGB 888/666."]
pub type RotW<'a, REG> = crate::BitWriter<'a, REG, Rot>;
impl<'a, REG> RotW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rot::B1)
    }
    #[doc = "disabled note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the self path. In rotation mode only 4-beat bursts are supported for self- path."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rot::B0)
    }
}
#[doc = "Enables change of byte order of the 32 bit output word\n\nat write port\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ByteSwap {
    #[doc = "1: byte order is mirrored but the bit order within one byte doesn‟t change"]
    B1 = 1,
    #[doc = "0: no byte mirroring note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the main and self path."]
    B0 = 0,
}
impl From<ByteSwap> for bool {
    #[inline(always)]
    fn from(variant: ByteSwap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `byte_swap` reader - Enables change of byte order of the 32 bit output word\n\nat write port"]
pub type ByteSwapR = crate::BitReader<ByteSwap>;
impl ByteSwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ByteSwap {
        match self.bits {
            true => ByteSwap::B1,
            false => ByteSwap::B0,
        }
    }
    #[doc = "byte order is mirrored but the bit order within one byte doesn‟t change"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ByteSwap::B1
    }
    #[doc = "no byte mirroring note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the main and self path."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ByteSwap::B0
    }
}
#[doc = "Field `byte_swap` writer - Enables change of byte order of the 32 bit output word\n\nat write port"]
pub type ByteSwapW<'a, REG> = crate::BitWriter<'a, REG, ByteSwap>;
impl<'a, REG> ByteSwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "byte order is mirrored but the bit order within one byte doesn‟t change"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ByteSwap::B1)
    }
    #[doc = "no byte mirroring note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the main and self path."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ByteSwap::B0)
    }
}
#[doc = "Enables Y full range for self path YCbCr -> RGB\n\nconversion\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum YFullRange {
    #[doc = "1: Y has full range (0..255)"]
    B1 = 1,
    #[doc = "0: Y has compressed range (16..235) note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the self path."]
    B0 = 0,
}
impl From<YFullRange> for bool {
    #[inline(always)]
    fn from(variant: YFullRange) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `y_full_range` reader - Enables Y full range for self path YCbCr -> RGB\n\nconversion"]
pub type YFullRangeR = crate::BitReader<YFullRange>;
impl YFullRangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> YFullRange {
        match self.bits {
            true => YFullRange::B1,
            false => YFullRange::B0,
        }
    }
    #[doc = "Y has full range (0..255)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == YFullRange::B1
    }
    #[doc = "Y has compressed range (16..235) note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the self path."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == YFullRange::B0
    }
}
#[doc = "Field `y_full_range` writer - Enables Y full range for self path YCbCr -> RGB\n\nconversion"]
pub type YFullRangeW<'a, REG> = crate::BitWriter<'a, REG, YFullRange>;
impl<'a, REG> YFullRangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Y has full range (0..255)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(YFullRange::B1)
    }
    #[doc = "Y has compressed range (16..235) note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the self path."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(YFullRange::B0)
    }
}
#[doc = "Enables CbCr full range for self path YCbCr -> RGB\n\nconversion\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CbcrFullRange {
    #[doc = "1: CbCr have full range (0..255)"]
    B1 = 1,
    #[doc = "0: CbCr have compressed range range (16..240) note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the self path."]
    B0 = 0,
}
impl From<CbcrFullRange> for bool {
    #[inline(always)]
    fn from(variant: CbcrFullRange) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `cbcr_full_range` reader - Enables CbCr full range for self path YCbCr -> RGB\n\nconversion"]
pub type CbcrFullRangeR = crate::BitReader<CbcrFullRange>;
impl CbcrFullRangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CbcrFullRange {
        match self.bits {
            true => CbcrFullRange::B1,
            false => CbcrFullRange::B0,
        }
    }
    #[doc = "CbCr have full range (0..255)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CbcrFullRange::B1
    }
    #[doc = "CbCr have compressed range range (16..240) note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the self path."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CbcrFullRange::B0
    }
}
#[doc = "Field `cbcr_full_range` writer - Enables CbCr full range for self path YCbCr -> RGB\n\nconversion"]
pub type CbcrFullRangeW<'a, REG> = crate::BitWriter<'a, REG, CbcrFullRange>;
impl<'a, REG> CbcrFullRangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CbCr have full range (0..255)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CbcrFullRange::B1)
    }
    #[doc = "CbCr have compressed range range (16..240) note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the self path."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CbcrFullRange::B0)
    }
}
#[doc = "Enables self path YCbCr422non-co-sited -> YCbCr444\n\ninterpolation\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum _422noncosited {
    #[doc = "1: YCbCr422 data are non_co-sited (Cb and Cr samples are centered between Y samples) so modified interpolation is activated"]
    B1 = 1,
    #[doc = "0: YCbCr422 data are co-sited (Y0 Cb0 and Cr0 are sampled at the same position) note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the self path."]
    B0 = 0,
}
impl From<_422noncosited> for bool {
    #[inline(always)]
    fn from(variant: _422noncosited) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `422noncosited` reader - Enables self path YCbCr422non-co-sited -> YCbCr444\n\ninterpolation"]
pub type _422noncositedR = crate::BitReader<_422noncosited>;
impl _422noncositedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> _422noncosited {
        match self.bits {
            true => _422noncosited::B1,
            false => _422noncosited::B0,
        }
    }
    #[doc = "YCbCr422 data are non_co-sited (Cb and Cr samples are centered between Y samples) so modified interpolation is activated"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == _422noncosited::B1
    }
    #[doc = "YCbCr422 data are co-sited (Y0 Cb0 and Cr0 are sampled at the same position) note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the self path."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == _422noncosited::B0
    }
}
#[doc = "Field `422noncosited` writer - Enables self path YCbCr422non-co-sited -> YCbCr444\n\ninterpolation"]
pub type _422noncositedW<'a, REG> = crate::BitWriter<'a, REG, _422noncosited>;
impl<'a, REG> _422noncositedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "YCbCr422 data are non_co-sited (Cb and Cr samples are centered between Y samples) so modified interpolation is activated"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(_422noncosited::B1)
    }
    #[doc = "YCbCr422 data are co-sited (Y0 Cb0 and Cr0 are sampled at the same position) note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the self path."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(_422noncosited::B0)
    }
}
#[doc = "pingpong mode of configuration registers for main\n\npath at frame end.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MpPingpongEnable {
    #[doc = "1: enabled"]
    B1 = 1,
    #[doc = "0: disabled"]
    B0 = 0,
}
impl From<MpPingpongEnable> for bool {
    #[inline(always)]
    fn from(variant: MpPingpongEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `mp_pingpong_enable` reader - pingpong mode of configuration registers for main\n\npath at frame end."]
pub type MpPingpongEnableR = crate::BitReader<MpPingpongEnable>;
impl MpPingpongEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MpPingpongEnable {
        match self.bits {
            true => MpPingpongEnable::B1,
            false => MpPingpongEnable::B0,
        }
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == MpPingpongEnable::B1
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == MpPingpongEnable::B0
    }
}
#[doc = "Field `mp_pingpong_enable` writer - pingpong mode of configuration registers for main\n\npath at frame end."]
pub type MpPingpongEnableW<'a, REG> = crate::BitWriter<'a, REG, MpPingpongEnable>;
impl<'a, REG> MpPingpongEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(MpPingpongEnable::B1)
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(MpPingpongEnable::B0)
    }
}
#[doc = "pingpong mode of configuration registers for self path\n\nat frame end.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpPingpongEnable {
    #[doc = "1: enabled"]
    B1 = 1,
    #[doc = "0: disabled"]
    B0 = 0,
}
impl From<SpPingpongEnable> for bool {
    #[inline(always)]
    fn from(variant: SpPingpongEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `sp_pingpong_enable` reader - pingpong mode of configuration registers for self path\n\nat frame end."]
pub type SpPingpongEnableR = crate::BitReader<SpPingpongEnable>;
impl SpPingpongEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SpPingpongEnable {
        match self.bits {
            true => SpPingpongEnable::B1,
            false => SpPingpongEnable::B0,
        }
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SpPingpongEnable::B1
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SpPingpongEnable::B0
    }
}
#[doc = "Field `sp_pingpong_enable` writer - pingpong mode of configuration registers for self path\n\nat frame end."]
pub type SpPingpongEnableW<'a, REG> = crate::BitWriter<'a, REG, SpPingpongEnable>;
impl<'a, REG> SpPingpongEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SpPingpongEnable::B1)
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SpPingpongEnable::B0)
    }
}
#[doc = "automatic update of configuration registers for main\n\npath at frame end.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MpAutoUpdate {
    #[doc = "1: enabled"]
    B1 = 1,
    #[doc = "0: disabled"]
    B0 = 0,
}
impl From<MpAutoUpdate> for bool {
    #[inline(always)]
    fn from(variant: MpAutoUpdate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `mp_auto_update` reader - automatic update of configuration registers for main\n\npath at frame end."]
pub type MpAutoUpdateR = crate::BitReader<MpAutoUpdate>;
impl MpAutoUpdateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MpAutoUpdate {
        match self.bits {
            true => MpAutoUpdate::B1,
            false => MpAutoUpdate::B0,
        }
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == MpAutoUpdate::B1
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == MpAutoUpdate::B0
    }
}
#[doc = "Field `mp_auto_update` writer - automatic update of configuration registers for main\n\npath at frame end."]
pub type MpAutoUpdateW<'a, REG> = crate::BitWriter<'a, REG, MpAutoUpdate>;
impl<'a, REG> MpAutoUpdateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(MpAutoUpdate::B1)
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(MpAutoUpdate::B0)
    }
}
#[doc = "automatic update of configuration registers for self\n\npath at frame end.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpAutoUpdate {
    #[doc = "1: enabled"]
    B1 = 1,
    #[doc = "0: disabled"]
    B0 = 0,
}
impl From<SpAutoUpdate> for bool {
    #[inline(always)]
    fn from(variant: SpAutoUpdate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `sp_auto_update` reader - automatic update of configuration registers for self\n\npath at frame end."]
pub type SpAutoUpdateR = crate::BitReader<SpAutoUpdate>;
impl SpAutoUpdateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SpAutoUpdate {
        match self.bits {
            true => SpAutoUpdate::B1,
            false => SpAutoUpdate::B0,
        }
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SpAutoUpdate::B1
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SpAutoUpdate::B0
    }
}
#[doc = "Field `sp_auto_update` writer - automatic update of configuration registers for self\n\npath at frame end."]
pub type SpAutoUpdateW<'a, REG> = crate::BitWriter<'a, REG, SpAutoUpdate>;
impl<'a, REG> SpAutoUpdateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SpAutoUpdate::B1)
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SpAutoUpdate::B0)
    }
}
#[doc = "enables the last pixel signalization\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LastPixelSigEn {
    #[doc = "1: enabled"]
    B1 = 1,
    #[doc = "0: disabled"]
    B0 = 0,
}
impl From<LastPixelSigEn> for bool {
    #[inline(always)]
    fn from(variant: LastPixelSigEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `last_pixel_sig_en` reader - enables the last pixel signalization"]
pub type LastPixelSigEnR = crate::BitReader<LastPixelSigEn>;
impl LastPixelSigEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LastPixelSigEn {
        match self.bits {
            true => LastPixelSigEn::B1,
            false => LastPixelSigEn::B0,
        }
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == LastPixelSigEn::B1
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == LastPixelSigEn::B0
    }
}
#[doc = "Field `last_pixel_sig_en` writer - enables the last pixel signalization"]
pub type LastPixelSigEnW<'a, REG> = crate::BitWriter<'a, REG, LastPixelSigEn>;
impl<'a, REG> LastPixelSigEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(LastPixelSigEn::B1)
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(LastPixelSigEn::B0)
    }
}
#[doc = "Burst length for Y, JPEG, or raw data affecting write\n\nport. 00: 4-beat bursts\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BurstLenLum {
    #[doc = "1: 8-beat bursts"]
    B01 = 1,
    #[doc = "2: 16-beat bursts"]
    B10 = 2,
    #[doc = "3: reserved Ignored if 8- or 16-beat bursts are not supported. note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the main and self path."]
    B11 = 3,
}
impl From<BurstLenLum> for u8 {
    #[inline(always)]
    fn from(variant: BurstLenLum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BurstLenLum {
    type Ux = u8;
}
#[doc = "Field `burst_len_lum` reader - Burst length for Y, JPEG, or raw data affecting write\n\nport. 00: 4-beat bursts"]
pub type BurstLenLumR = crate::FieldReader<BurstLenLum>;
impl BurstLenLumR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BurstLenLum> {
        match self.bits {
            1 => Some(BurstLenLum::B01),
            2 => Some(BurstLenLum::B10),
            3 => Some(BurstLenLum::B11),
            _ => None,
        }
    }
    #[doc = "8-beat bursts"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == BurstLenLum::B01
    }
    #[doc = "16-beat bursts"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == BurstLenLum::B10
    }
    #[doc = "reserved Ignored if 8- or 16-beat bursts are not supported. note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the main and self path."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == BurstLenLum::B11
    }
}
#[doc = "Field `burst_len_lum` writer - Burst length for Y, JPEG, or raw data affecting write\n\nport. 00: 4-beat bursts"]
pub type BurstLenLumW<'a, REG> = crate::FieldWriter<'a, REG, 2, BurstLenLum>;
impl<'a, REG> BurstLenLumW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-beat bursts"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(BurstLenLum::B01)
    }
    #[doc = "16-beat bursts"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(BurstLenLum::B10)
    }
    #[doc = "reserved Ignored if 8- or 16-beat bursts are not supported. note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the main and self path."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(BurstLenLum::B11)
    }
}
#[doc = "Burst length for Cb or Cr data affecting write port.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BurstLenChrom {
    #[doc = "0: 4-beat bursts"]
    B00 = 0,
    #[doc = "1: 8-beat bursts"]
    B01 = 1,
    #[doc = "2: 16-beat bursts"]
    B10 = 2,
    #[doc = "3: reserved Ignored if 8- or 16-beat bursts are not supported. If rotation is active, then only 4-beat bursts will be generated in self path, regardless of the setting here. note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the main and self path."]
    B11 = 3,
}
impl From<BurstLenChrom> for u8 {
    #[inline(always)]
    fn from(variant: BurstLenChrom) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BurstLenChrom {
    type Ux = u8;
}
#[doc = "Field `burst_len_chrom` reader - Burst length for Cb or Cr data affecting write port."]
pub type BurstLenChromR = crate::FieldReader<BurstLenChrom>;
impl BurstLenChromR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BurstLenChrom {
        match self.bits {
            0 => BurstLenChrom::B00,
            1 => BurstLenChrom::B01,
            2 => BurstLenChrom::B10,
            3 => BurstLenChrom::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "4-beat bursts"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == BurstLenChrom::B00
    }
    #[doc = "8-beat bursts"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == BurstLenChrom::B01
    }
    #[doc = "16-beat bursts"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == BurstLenChrom::B10
    }
    #[doc = "reserved Ignored if 8- or 16-beat bursts are not supported. If rotation is active, then only 4-beat bursts will be generated in self path, regardless of the setting here. note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the main and self path."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == BurstLenChrom::B11
    }
}
#[doc = "Field `burst_len_chrom` writer - Burst length for Cb or Cr data affecting write port."]
pub type BurstLenChromW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, BurstLenChrom>;
impl<'a, REG> BurstLenChromW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4-beat bursts"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(BurstLenChrom::B00)
    }
    #[doc = "8-beat bursts"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(BurstLenChrom::B01)
    }
    #[doc = "16-beat bursts"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(BurstLenChrom::B10)
    }
    #[doc = "reserved Ignored if 8- or 16-beat bursts are not supported. If rotation is active, then only 4-beat bursts will be generated in self path, regardless of the setting here. note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the main and self path."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(BurstLenChrom::B11)
    }
}
#[doc = "Field `init_base_en` reader - Enables updating of the base address and buffer size\n\nshadow registers for main and self picture to the\n\nprogrammed register init values. MI_MP/SP_Y/CB/CR_BASE_AD_INIT\n\n-> MI_MP/SP_Y/CB/CR_BASE_AD_SHD MI_MP/SP_Y/CB/CR_SIZE_INIT\n\n-> MI_MP/SP_Y/CB/CR_SIZE_SHD\n\nThe update will be executed either when a forced\n\nsoftware update occurs (in register MI_INIT bit cfg_upd =\n\n1) or when an automatic config update signal arrives at\n\nthe MI input port. The latter is split into main and self\n\npicture. So only the corresponding main/self shadow\n\nregisters are affected."]
pub type InitBaseEnR = crate::BitReader;
#[doc = "Field `init_base_en` writer - Enables updating of the base address and buffer size\n\nshadow registers for main and self picture to the\n\nprogrammed register init values. MI_MP/SP_Y/CB/CR_BASE_AD_INIT\n\n-> MI_MP/SP_Y/CB/CR_BASE_AD_SHD MI_MP/SP_Y/CB/CR_SIZE_INIT\n\n-> MI_MP/SP_Y/CB/CR_SIZE_SHD\n\nThe update will be executed either when a forced\n\nsoftware update occurs (in register MI_INIT bit cfg_upd =\n\n1) or when an automatic config update signal arrives at\n\nthe MI input port. The latter is split into main and self\n\npicture. So only the corresponding main/self shadow\n\nregisters are affected."]
pub type InitBaseEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `init_offset_en` reader - Enables updating of the offset counters shadow\n\nregisters for main and self picture to the programmed\n\nregister init values. MI_MP/SP_Y/CB/CR_OFFS_CNT_INIT\n\n-> MI_MP/SP_Y/CB/CR_OFFS_CNT_SHD\n\nThe update will be executed either when a forced\n\nsoftware update occurs (in register MI_INIT bit cfg_upd =\n\n1) or when an automatic config update signal arrives at\n\nthe MI input port. The latter is split into main and self\n\npicture. So only the corresponding main/self shadow\n\nregisters are affected.\n\nAfter a picture skip has been performed init_offset_en\n\nselects between skip restart and skip init mode (see bit\n\nskip in register MI_INIT)."]
pub type InitOffsetEnR = crate::BitReader;
#[doc = "Field `init_offset_en` writer - Enables updating of the offset counters shadow\n\nregisters for main and self picture to the programmed\n\nregister init values. MI_MP/SP_Y/CB/CR_OFFS_CNT_INIT\n\n-> MI_MP/SP_Y/CB/CR_OFFS_CNT_SHD\n\nThe update will be executed either when a forced\n\nsoftware update occurs (in register MI_INIT bit cfg_upd =\n\n1) or when an automatic config update signal arrives at\n\nthe MI input port. The latter is split into main and self\n\npicture. So only the corresponding main/self shadow\n\nregisters are affected.\n\nAfter a picture skip has been performed init_offset_en\n\nselects between skip restart and skip init mode (see bit\n\nskip in register MI_INIT)."]
pub type InitOffsetEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mp_write_format` reader - Defines how YCbCr main picture data is written to\n\nmemory. Ignored if JPEG data is chosen.\n\nIn YCbCr mode the following meaning is applicable 00:\n\nplanar\n\n01: semi planar, for YCbCr 4:2:x\n\n10: interleaved (combined), for YCbCr 4:2:2 only 11:\n\nreserved\n\nIn RAW data mode the following meaning is applicable\n\n00: RAW 8 bit\n\n01: reserved\n\n10: RAW 12 bit\n\n11: reserved\n\nnote: Programmed value becomes effective\n\nimmediately. So write to the register only if no picture data\n\nis sent to the main path.\n\n"]
pub type MpWriteFormatR = crate::FieldReader;
#[doc = "Field `mp_write_format` writer - Defines how YCbCr main picture data is written to\n\nmemory. Ignored if JPEG data is chosen.\n\nIn YCbCr mode the following meaning is applicable 00:\n\nplanar\n\n01: semi planar, for YCbCr 4:2:x\n\n10: interleaved (combined), for YCbCr 4:2:2 only 11:\n\nreserved\n\nIn RAW data mode the following meaning is applicable\n\n00: RAW 8 bit\n\n01: reserved\n\n10: RAW 12 bit\n\n11: reserved\n\nnote: Programmed value becomes effective\n\nimmediately. So write to the register only if no picture data\n\nis sent to the main path.\n\n"]
pub type MpWriteFormatW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Defines how YCbCr self picture data is written to\n\nmemory. Must be set to 00 if RGB conversion is active.\n\nNote that with RGB conversion active the output format is\n\nalways interleaved.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SpWriteFormat {
    #[doc = "0: planar"]
    B00 = 0,
    #[doc = "1: semi planar, for YCbCr 4:2:x"]
    B01 = 1,
    #[doc = "2: interleaved (combined), for YCbCr 4:2:2 only 11: reserved note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the self path."]
    B10 = 2,
}
impl From<SpWriteFormat> for u8 {
    #[inline(always)]
    fn from(variant: SpWriteFormat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SpWriteFormat {
    type Ux = u8;
}
#[doc = "Field `sp_write_format` reader - Defines how YCbCr self picture data is written to\n\nmemory. Must be set to 00 if RGB conversion is active.\n\nNote that with RGB conversion active the output format is\n\nalways interleaved."]
pub type SpWriteFormatR = crate::FieldReader<SpWriteFormat>;
impl SpWriteFormatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SpWriteFormat> {
        match self.bits {
            0 => Some(SpWriteFormat::B00),
            1 => Some(SpWriteFormat::B01),
            2 => Some(SpWriteFormat::B10),
            _ => None,
        }
    }
    #[doc = "planar"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == SpWriteFormat::B00
    }
    #[doc = "semi planar, for YCbCr 4:2:x"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == SpWriteFormat::B01
    }
    #[doc = "interleaved (combined), for YCbCr 4:2:2 only 11: reserved note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the self path."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == SpWriteFormat::B10
    }
}
#[doc = "Field `sp_write_format` writer - Defines how YCbCr self picture data is written to\n\nmemory. Must be set to 00 if RGB conversion is active.\n\nNote that with RGB conversion active the output format is\n\nalways interleaved."]
pub type SpWriteFormatW<'a, REG> = crate::FieldWriter<'a, REG, 2, SpWriteFormat>;
impl<'a, REG> SpWriteFormatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "planar"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(SpWriteFormat::B00)
    }
    #[doc = "semi planar, for YCbCr 4:2:x"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(SpWriteFormat::B01)
    }
    #[doc = "interleaved (combined), for YCbCr 4:2:2 only 11: reserved note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the self path."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(SpWriteFormat::B10)
    }
}
#[doc = "Selects input format of self picture. For possible\n\nrestrictions see sub-chapter 'Picture Orientation' in\n\nchapter 'Self Path Output Programming'.\n\n\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SpInputFormat {
    #[doc = "3: YCbCr 4:4:4"]
    B11 = 3,
    #[doc = "2: YCbCr 4:2:2"]
    B10 = 2,
    #[doc = "1: YCbCr 4:2:0"]
    B01 = 1,
    #[doc = "0: YCbCr 4:0:0 note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the self path."]
    B00 = 0,
}
impl From<SpInputFormat> for u8 {
    #[inline(always)]
    fn from(variant: SpInputFormat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SpInputFormat {
    type Ux = u8;
}
#[doc = "Field `sp_input_format` reader - Selects input format of self picture. For possible\n\nrestrictions see sub-chapter 'Picture Orientation' in\n\nchapter 'Self Path Output Programming'.\n\n"]
pub type SpInputFormatR = crate::FieldReader<SpInputFormat>;
impl SpInputFormatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SpInputFormat {
        match self.bits {
            3 => SpInputFormat::B11,
            2 => SpInputFormat::B10,
            1 => SpInputFormat::B01,
            0 => SpInputFormat::B00,
            _ => unreachable!(),
        }
    }
    #[doc = "YCbCr 4:4:4"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == SpInputFormat::B11
    }
    #[doc = "YCbCr 4:2:2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == SpInputFormat::B10
    }
    #[doc = "YCbCr 4:2:0"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == SpInputFormat::B01
    }
    #[doc = "YCbCr 4:0:0 note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the self path."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == SpInputFormat::B00
    }
}
#[doc = "Field `sp_input_format` writer - Selects input format of self picture. For possible\n\nrestrictions see sub-chapter 'Picture Orientation' in\n\nchapter 'Self Path Output Programming'.\n\n"]
pub type SpInputFormatW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SpInputFormat>;
impl<'a, REG> SpInputFormatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "YCbCr 4:4:4"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(SpInputFormat::B11)
    }
    #[doc = "YCbCr 4:2:2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(SpInputFormat::B10)
    }
    #[doc = "YCbCr 4:2:0"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(SpInputFormat::B01)
    }
    #[doc = "YCbCr 4:0:0 note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the self path."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(SpInputFormat::B00)
    }
}
#[doc = "Selects output format of self picture. For possible\n\nrestrictions see sub-chapter 'Picture Orientation' in\n\nchapter 'Self Path Output Programming'.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SpOutputFormat {
    #[doc = "6: RGB 888"]
    B110 = 6,
    #[doc = "5: RGB 666"]
    B101 = 5,
    #[doc = "4: RGB 565"]
    B100 = 4,
    #[doc = "3: YCbCr 4:4:4"]
    B011 = 3,
    #[doc = "2: YCbCr 4:2:2"]
    B010 = 2,
    #[doc = "1: YCbCr 4:2:0"]
    B001 = 1,
    #[doc = "0: YCbCr 4:0:0 note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the self path. for RGB output format the SP input format must be YCbCr 4:2:2"]
    B000 = 0,
}
impl From<SpOutputFormat> for u8 {
    #[inline(always)]
    fn from(variant: SpOutputFormat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SpOutputFormat {
    type Ux = u8;
}
#[doc = "Field `sp_output_format` reader - Selects output format of self picture. For possible\n\nrestrictions see sub-chapter 'Picture Orientation' in\n\nchapter 'Self Path Output Programming'."]
pub type SpOutputFormatR = crate::FieldReader<SpOutputFormat>;
impl SpOutputFormatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SpOutputFormat> {
        match self.bits {
            6 => Some(SpOutputFormat::B110),
            5 => Some(SpOutputFormat::B101),
            4 => Some(SpOutputFormat::B100),
            3 => Some(SpOutputFormat::B011),
            2 => Some(SpOutputFormat::B010),
            1 => Some(SpOutputFormat::B001),
            0 => Some(SpOutputFormat::B000),
            _ => None,
        }
    }
    #[doc = "RGB 888"]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == SpOutputFormat::B110
    }
    #[doc = "RGB 666"]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == SpOutputFormat::B101
    }
    #[doc = "RGB 565"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == SpOutputFormat::B100
    }
    #[doc = "YCbCr 4:4:4"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == SpOutputFormat::B011
    }
    #[doc = "YCbCr 4:2:2"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == SpOutputFormat::B010
    }
    #[doc = "YCbCr 4:2:0"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == SpOutputFormat::B001
    }
    #[doc = "YCbCr 4:0:0 note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the self path. for RGB output format the SP input format must be YCbCr 4:2:2"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == SpOutputFormat::B000
    }
}
#[doc = "Field `sp_output_format` writer - Selects output format of self picture. For possible\n\nrestrictions see sub-chapter 'Picture Orientation' in\n\nchapter 'Self Path Output Programming'."]
pub type SpOutputFormatW<'a, REG> = crate::FieldWriter<'a, REG, 3, SpOutputFormat>;
impl<'a, REG> SpOutputFormatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RGB 888"]
    #[inline(always)]
    pub fn b110(self) -> &'a mut crate::W<REG> {
        self.variant(SpOutputFormat::B110)
    }
    #[doc = "RGB 666"]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(SpOutputFormat::B101)
    }
    #[doc = "RGB 565"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(SpOutputFormat::B100)
    }
    #[doc = "YCbCr 4:4:4"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(SpOutputFormat::B011)
    }
    #[doc = "YCbCr 4:2:2"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(SpOutputFormat::B010)
    }
    #[doc = "YCbCr 4:2:0"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(SpOutputFormat::B001)
    }
    #[doc = "YCbCr 4:0:0 note: Programmed value becomes effective immediately. So write to the register only if no picture data is sent to the self path. for RGB output format the SP input format must be YCbCr 4:2:2"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(SpOutputFormat::B000)
    }
}
impl R {
    #[doc = "Bits 0:3 - Enables data pathes of MI according to the following\n\ntable:\n\n"]
    #[inline(always)]
    pub fn path_enable(&self) -> PathEnableR {
        PathEnableR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Horizontal flipping of self picture. For picture\n\norientation and operation modes see sub-chapter 'Picture\n\nOrientation' in chapter 'Self Path Output Programming'.\n\nFor Y component the line length in 4:2:x planar mode\n\nmust be a multiple of 8, for all other component modes a\n\nmultiple of 4 and for RGB 565 a multiple of 2. There are no\n\nrestrictions for RGB 888/666."]
    #[inline(always)]
    pub fn h_flip(&self) -> HFlipR {
        HFlipR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Vertical flipping of self picture. For picture orientation\n\nand operation modes see sub-chapter 'Picture\n\nOrientation' in chapter 'Self Path Output Programming'.\n\nFor Y component the line length in 4:2:x planar mode\n\nmust be a multiple of 8, for all other component modes a\n\nmultiple of 4 and for RGB 565 a multiple of 2. There are no\n\nrestrictions for RGB 888/666."]
    #[inline(always)]
    pub fn v_flip(&self) -> VFlipR {
        VFlipR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rotation 90 degree counter clockwise of self picture,\n\nonly in RGB mode. For picture orientation and operation\n\nmodes see sub-chapter 'Picture Orientation' in chapter\n\n'Self Path Output Programming'.\n\nFor RGB 565 format the line length must be a multiple\n\nof 2. There are no restrictions for RGB 888/666."]
    #[inline(always)]
    pub fn rot(&self) -> RotR {
        RotR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables change of byte order of the 32 bit output word\n\nat write port"]
    #[inline(always)]
    pub fn byte_swap(&self) -> ByteSwapR {
        ByteSwapR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enables Y full range for self path YCbCr -> RGB\n\nconversion"]
    #[inline(always)]
    pub fn y_full_range(&self) -> YFullRangeR {
        YFullRangeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enables CbCr full range for self path YCbCr -> RGB\n\nconversion"]
    #[inline(always)]
    pub fn cbcr_full_range(&self) -> CbcrFullRangeR {
        CbcrFullRangeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enables self path YCbCr422non-co-sited -> YCbCr444\n\ninterpolation"]
    #[inline(always)]
    pub fn _422noncosited(&self) -> _422noncositedR {
        _422noncositedR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - pingpong mode of configuration registers for main\n\npath at frame end."]
    #[inline(always)]
    pub fn mp_pingpong_enable(&self) -> MpPingpongEnableR {
        MpPingpongEnableR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - pingpong mode of configuration registers for self path\n\nat frame end."]
    #[inline(always)]
    pub fn sp_pingpong_enable(&self) -> SpPingpongEnableR {
        SpPingpongEnableR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - automatic update of configuration registers for main\n\npath at frame end."]
    #[inline(always)]
    pub fn mp_auto_update(&self) -> MpAutoUpdateR {
        MpAutoUpdateR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - automatic update of configuration registers for self\n\npath at frame end."]
    #[inline(always)]
    pub fn sp_auto_update(&self) -> SpAutoUpdateR {
        SpAutoUpdateR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - enables the last pixel signalization"]
    #[inline(always)]
    pub fn last_pixel_sig_en(&self) -> LastPixelSigEnR {
        LastPixelSigEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Burst length for Y, JPEG, or raw data affecting write\n\nport. 00: 4-beat bursts"]
    #[inline(always)]
    pub fn burst_len_lum(&self) -> BurstLenLumR {
        BurstLenLumR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Burst length for Cb or Cr data affecting write port."]
    #[inline(always)]
    pub fn burst_len_chrom(&self) -> BurstLenChromR {
        BurstLenChromR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Enables updating of the base address and buffer size\n\nshadow registers for main and self picture to the\n\nprogrammed register init values. MI_MP/SP_Y/CB/CR_BASE_AD_INIT\n\n-> MI_MP/SP_Y/CB/CR_BASE_AD_SHD MI_MP/SP_Y/CB/CR_SIZE_INIT\n\n-> MI_MP/SP_Y/CB/CR_SIZE_SHD\n\nThe update will be executed either when a forced\n\nsoftware update occurs (in register MI_INIT bit cfg_upd =\n\n1) or when an automatic config update signal arrives at\n\nthe MI input port. The latter is split into main and self\n\npicture. So only the corresponding main/self shadow\n\nregisters are affected."]
    #[inline(always)]
    pub fn init_base_en(&self) -> InitBaseEnR {
        InitBaseEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enables updating of the offset counters shadow\n\nregisters for main and self picture to the programmed\n\nregister init values. MI_MP/SP_Y/CB/CR_OFFS_CNT_INIT\n\n-> MI_MP/SP_Y/CB/CR_OFFS_CNT_SHD\n\nThe update will be executed either when a forced\n\nsoftware update occurs (in register MI_INIT bit cfg_upd =\n\n1) or when an automatic config update signal arrives at\n\nthe MI input port. The latter is split into main and self\n\npicture. So only the corresponding main/self shadow\n\nregisters are affected.\n\nAfter a picture skip has been performed init_offset_en\n\nselects between skip restart and skip init mode (see bit\n\nskip in register MI_INIT)."]
    #[inline(always)]
    pub fn init_offset_en(&self) -> InitOffsetEnR {
        InitOffsetEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Defines how YCbCr main picture data is written to\n\nmemory. Ignored if JPEG data is chosen.\n\nIn YCbCr mode the following meaning is applicable 00:\n\nplanar\n\n01: semi planar, for YCbCr 4:2:x\n\n10: interleaved (combined), for YCbCr 4:2:2 only 11:\n\nreserved\n\nIn RAW data mode the following meaning is applicable\n\n00: RAW 8 bit\n\n01: reserved\n\n10: RAW 12 bit\n\n11: reserved\n\nnote: Programmed value becomes effective\n\nimmediately. So write to the register only if no picture data\n\nis sent to the main path.\n\n"]
    #[inline(always)]
    pub fn mp_write_format(&self) -> MpWriteFormatR {
        MpWriteFormatR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Defines how YCbCr self picture data is written to\n\nmemory. Must be set to 00 if RGB conversion is active.\n\nNote that with RGB conversion active the output format is\n\nalways interleaved."]
    #[inline(always)]
    pub fn sp_write_format(&self) -> SpWriteFormatR {
        SpWriteFormatR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Selects input format of self picture. For possible\n\nrestrictions see sub-chapter 'Picture Orientation' in\n\nchapter 'Self Path Output Programming'.\n\n"]
    #[inline(always)]
    pub fn sp_input_format(&self) -> SpInputFormatR {
        SpInputFormatR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:30 - Selects output format of self picture. For possible\n\nrestrictions see sub-chapter 'Picture Orientation' in\n\nchapter 'Self Path Output Programming'."]
    #[inline(always)]
    pub fn sp_output_format(&self) -> SpOutputFormatR {
        SpOutputFormatR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Enables data pathes of MI according to the following\n\ntable:\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn path_enable(&mut self) -> PathEnableW<MiCtrlSpec> {
        PathEnableW::new(self, 0)
    }
    #[doc = "Bit 4 - Horizontal flipping of self picture. For picture\n\norientation and operation modes see sub-chapter 'Picture\n\nOrientation' in chapter 'Self Path Output Programming'.\n\nFor Y component the line length in 4:2:x planar mode\n\nmust be a multiple of 8, for all other component modes a\n\nmultiple of 4 and for RGB 565 a multiple of 2. There are no\n\nrestrictions for RGB 888/666."]
    #[inline(always)]
    #[must_use]
    pub fn h_flip(&mut self) -> HFlipW<MiCtrlSpec> {
        HFlipW::new(self, 4)
    }
    #[doc = "Bit 5 - Vertical flipping of self picture. For picture orientation\n\nand operation modes see sub-chapter 'Picture\n\nOrientation' in chapter 'Self Path Output Programming'.\n\nFor Y component the line length in 4:2:x planar mode\n\nmust be a multiple of 8, for all other component modes a\n\nmultiple of 4 and for RGB 565 a multiple of 2. There are no\n\nrestrictions for RGB 888/666."]
    #[inline(always)]
    #[must_use]
    pub fn v_flip(&mut self) -> VFlipW<MiCtrlSpec> {
        VFlipW::new(self, 5)
    }
    #[doc = "Bit 6 - Rotation 90 degree counter clockwise of self picture,\n\nonly in RGB mode. For picture orientation and operation\n\nmodes see sub-chapter 'Picture Orientation' in chapter\n\n'Self Path Output Programming'.\n\nFor RGB 565 format the line length must be a multiple\n\nof 2. There are no restrictions for RGB 888/666."]
    #[inline(always)]
    #[must_use]
    pub fn rot(&mut self) -> RotW<MiCtrlSpec> {
        RotW::new(self, 6)
    }
    #[doc = "Bit 7 - Enables change of byte order of the 32 bit output word\n\nat write port"]
    #[inline(always)]
    #[must_use]
    pub fn byte_swap(&mut self) -> ByteSwapW<MiCtrlSpec> {
        ByteSwapW::new(self, 7)
    }
    #[doc = "Bit 8 - Enables Y full range for self path YCbCr -> RGB\n\nconversion"]
    #[inline(always)]
    #[must_use]
    pub fn y_full_range(&mut self) -> YFullRangeW<MiCtrlSpec> {
        YFullRangeW::new(self, 8)
    }
    #[doc = "Bit 9 - Enables CbCr full range for self path YCbCr -> RGB\n\nconversion"]
    #[inline(always)]
    #[must_use]
    pub fn cbcr_full_range(&mut self) -> CbcrFullRangeW<MiCtrlSpec> {
        CbcrFullRangeW::new(self, 9)
    }
    #[doc = "Bit 10 - Enables self path YCbCr422non-co-sited -> YCbCr444\n\ninterpolation"]
    #[inline(always)]
    #[must_use]
    pub fn _422noncosited(&mut self) -> _422noncositedW<MiCtrlSpec> {
        _422noncositedW::new(self, 10)
    }
    #[doc = "Bit 11 - pingpong mode of configuration registers for main\n\npath at frame end."]
    #[inline(always)]
    #[must_use]
    pub fn mp_pingpong_enable(&mut self) -> MpPingpongEnableW<MiCtrlSpec> {
        MpPingpongEnableW::new(self, 11)
    }
    #[doc = "Bit 12 - pingpong mode of configuration registers for self path\n\nat frame end."]
    #[inline(always)]
    #[must_use]
    pub fn sp_pingpong_enable(&mut self) -> SpPingpongEnableW<MiCtrlSpec> {
        SpPingpongEnableW::new(self, 12)
    }
    #[doc = "Bit 13 - automatic update of configuration registers for main\n\npath at frame end."]
    #[inline(always)]
    #[must_use]
    pub fn mp_auto_update(&mut self) -> MpAutoUpdateW<MiCtrlSpec> {
        MpAutoUpdateW::new(self, 13)
    }
    #[doc = "Bit 14 - automatic update of configuration registers for self\n\npath at frame end."]
    #[inline(always)]
    #[must_use]
    pub fn sp_auto_update(&mut self) -> SpAutoUpdateW<MiCtrlSpec> {
        SpAutoUpdateW::new(self, 14)
    }
    #[doc = "Bit 15 - enables the last pixel signalization"]
    #[inline(always)]
    #[must_use]
    pub fn last_pixel_sig_en(&mut self) -> LastPixelSigEnW<MiCtrlSpec> {
        LastPixelSigEnW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Burst length for Y, JPEG, or raw data affecting write\n\nport. 00: 4-beat bursts"]
    #[inline(always)]
    #[must_use]
    pub fn burst_len_lum(&mut self) -> BurstLenLumW<MiCtrlSpec> {
        BurstLenLumW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Burst length for Cb or Cr data affecting write port."]
    #[inline(always)]
    #[must_use]
    pub fn burst_len_chrom(&mut self) -> BurstLenChromW<MiCtrlSpec> {
        BurstLenChromW::new(self, 18)
    }
    #[doc = "Bit 20 - Enables updating of the base address and buffer size\n\nshadow registers for main and self picture to the\n\nprogrammed register init values. MI_MP/SP_Y/CB/CR_BASE_AD_INIT\n\n-> MI_MP/SP_Y/CB/CR_BASE_AD_SHD MI_MP/SP_Y/CB/CR_SIZE_INIT\n\n-> MI_MP/SP_Y/CB/CR_SIZE_SHD\n\nThe update will be executed either when a forced\n\nsoftware update occurs (in register MI_INIT bit cfg_upd =\n\n1) or when an automatic config update signal arrives at\n\nthe MI input port. The latter is split into main and self\n\npicture. So only the corresponding main/self shadow\n\nregisters are affected."]
    #[inline(always)]
    #[must_use]
    pub fn init_base_en(&mut self) -> InitBaseEnW<MiCtrlSpec> {
        InitBaseEnW::new(self, 20)
    }
    #[doc = "Bit 21 - Enables updating of the offset counters shadow\n\nregisters for main and self picture to the programmed\n\nregister init values. MI_MP/SP_Y/CB/CR_OFFS_CNT_INIT\n\n-> MI_MP/SP_Y/CB/CR_OFFS_CNT_SHD\n\nThe update will be executed either when a forced\n\nsoftware update occurs (in register MI_INIT bit cfg_upd =\n\n1) or when an automatic config update signal arrives at\n\nthe MI input port. The latter is split into main and self\n\npicture. So only the corresponding main/self shadow\n\nregisters are affected.\n\nAfter a picture skip has been performed init_offset_en\n\nselects between skip restart and skip init mode (see bit\n\nskip in register MI_INIT)."]
    #[inline(always)]
    #[must_use]
    pub fn init_offset_en(&mut self) -> InitOffsetEnW<MiCtrlSpec> {
        InitOffsetEnW::new(self, 21)
    }
    #[doc = "Bits 22:23 - Defines how YCbCr main picture data is written to\n\nmemory. Ignored if JPEG data is chosen.\n\nIn YCbCr mode the following meaning is applicable 00:\n\nplanar\n\n01: semi planar, for YCbCr 4:2:x\n\n10: interleaved (combined), for YCbCr 4:2:2 only 11:\n\nreserved\n\nIn RAW data mode the following meaning is applicable\n\n00: RAW 8 bit\n\n01: reserved\n\n10: RAW 12 bit\n\n11: reserved\n\nnote: Programmed value becomes effective\n\nimmediately. So write to the register only if no picture data\n\nis sent to the main path.\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn mp_write_format(&mut self) -> MpWriteFormatW<MiCtrlSpec> {
        MpWriteFormatW::new(self, 22)
    }
    #[doc = "Bits 24:25 - Defines how YCbCr self picture data is written to\n\nmemory. Must be set to 00 if RGB conversion is active.\n\nNote that with RGB conversion active the output format is\n\nalways interleaved."]
    #[inline(always)]
    #[must_use]
    pub fn sp_write_format(&mut self) -> SpWriteFormatW<MiCtrlSpec> {
        SpWriteFormatW::new(self, 24)
    }
    #[doc = "Bits 26:27 - Selects input format of self picture. For possible\n\nrestrictions see sub-chapter 'Picture Orientation' in\n\nchapter 'Self Path Output Programming'.\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn sp_input_format(&mut self) -> SpInputFormatW<MiCtrlSpec> {
        SpInputFormatW::new(self, 26)
    }
    #[doc = "Bits 28:30 - Selects output format of self picture. For possible\n\nrestrictions see sub-chapter 'Picture Orientation' in\n\nchapter 'Self Path Output Programming'."]
    #[inline(always)]
    #[must_use]
    pub fn sp_output_format(&mut self) -> SpOutputFormatW<MiCtrlSpec> {
        SpOutputFormatW::new(self, 28)
    }
}
#[doc = "Global control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiCtrlSpec;
impl crate::RegisterSpec for MiCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_ctrl::R`](R) reader structure"]
impl crate::Readable for MiCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mi_ctrl::W`](W) writer structure"]
impl crate::Writable for MiCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MI_CTRL to value 0"]
impl crate::Resettable for MiCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
