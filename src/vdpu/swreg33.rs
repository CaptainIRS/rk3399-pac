#[doc = "Register `SWREG33` reader"]
pub type R = crate::R<Swreg33Spec>;
#[doc = "Field `PPD_MAX_OUTW` reader - the max pixels width allow for pp output"]
pub type PpdMaxOutwR = crate::FieldReader<u16>;
#[doc = "PD input tiled mode\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PpTileInMode {
    #[doc = "0: unsupport"]
    B0 = 0,
    #[doc = "1: 8x4 tile be used"]
    B1 = 1,
}
impl From<PpTileInMode> for u8 {
    #[inline(always)]
    fn from(variant: PpTileInMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PpTileInMode {
    type Ux = u8;
}
#[doc = "Field `PP_TILE_IN_MODE` reader - PD input tiled mode"]
pub type PpTileInModeR = crate::FieldReader<PpTileInMode>;
impl PpTileInModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PpTileInMode> {
        match self.bits {
            0 => Some(PpTileInMode::B0),
            1 => Some(PpTileInMode::B1),
            _ => None,
        }
    }
    #[doc = "unsupport"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PpTileInMode::B0
    }
    #[doc = "8x4 tile be used"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PpTileInMode::B1
    }
}
#[doc = "PPD exists flag\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PpdExistFlag {
    #[doc = "0: no exist"]
    B0 = 0,
    #[doc = "1: exist"]
    B1 = 1,
}
impl From<PpdExistFlag> for bool {
    #[inline(always)]
    fn from(variant: PpdExistFlag) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPD_EXIST_FLAG` reader - PPD exists flag"]
pub type PpdExistFlagR = crate::BitReader<PpdExistFlag>;
impl PpdExistFlagR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PpdExistFlag {
        match self.bits {
            false => PpdExistFlag::B0,
            true => PpdExistFlag::B1,
        }
    }
    #[doc = "no exist"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PpdExistFlag::B0
    }
    #[doc = "exist"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PpdExistFlag::B1
    }
}
#[doc = "PP output buffering select\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PpOutBufSel {
    #[doc = "0: output buffering is 1 unit"]
    B0 = 0,
    #[doc = "1: output buffering is 4 unit"]
    B1 = 1,
}
impl From<PpOutBufSel> for bool {
    #[inline(always)]
    fn from(variant: PpOutBufSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PP_OUT_BUF_SEL` reader - PP output buffering select"]
pub type PpOutBufSelR = crate::BitReader<PpOutBufSel>;
impl PpOutBufSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PpOutBufSel {
        match self.bits {
            false => PpOutBufSel::B0,
            true => PpOutBufSel::B1,
        }
    }
    #[doc = "output buffering is 1 unit"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PpOutBufSel::B0
    }
    #[doc = "output buffering is 4 unit"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PpOutBufSel::B1
    }
}
#[doc = "PP output endian mode select\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PpEndianMode {
    #[doc = "0: Endian mode supported except RGB"]
    B0 = 0,
    #[doc = "1: Endian mode supported for all format"]
    B1 = 1,
}
impl From<PpEndianMode> for bool {
    #[inline(always)]
    fn from(variant: PpEndianMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PP_ENDIAN_MODE` reader - PP output endian mode select"]
pub type PpEndianModeR = crate::BitReader<PpEndianMode>;
impl PpEndianModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PpEndianMode {
        match self.bits {
            false => PpEndianMode::B0,
            true => PpEndianMode::B1,
        }
    }
    #[doc = "Endian mode supported except RGB"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PpEndianMode::B0
    }
    #[doc = "Endian mode supported for all format"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PpEndianMode::B1
    }
}
#[doc = "PP input buffering select\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PpInBufSel {
    #[doc = "0: output buffering is 1 MB"]
    B0 = 0,
    #[doc = "1: output buffering is 4 MB"]
    B1 = 1,
}
impl From<PpInBufSel> for bool {
    #[inline(always)]
    fn from(variant: PpInBufSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PP_IN_BUF_SEL` reader - PP input buffering select"]
pub type PpInBufSelR = crate::BitReader<PpInBufSel>;
impl PpInBufSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PpInBufSel {
        match self.bits {
            false => PpInBufSel::B0,
            true => PpInBufSel::B1,
        }
    }
    #[doc = "output buffering is 1 MB"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PpInBufSel::B0
    }
    #[doc = "output buffering is 4 MB"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PpInBufSel::B1
    }
}
#[doc = "alpha blending exists flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AbldExistFlag {
    #[doc = "0: no exist"]
    B0 = 0,
    #[doc = "1: exist"]
    B1 = 1,
}
impl From<AbldExistFlag> for bool {
    #[inline(always)]
    fn from(variant: AbldExistFlag) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABLD_EXIST_FLAG` reader - alpha blending exists flag"]
pub type AbldExistFlagR = crate::BitReader<AbldExistFlag>;
impl AbldExistFlagR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AbldExistFlag {
        match self.bits {
            false => AbldExistFlag::B0,
            true => AbldExistFlag::B1,
        }
    }
    #[doc = "no exist"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AbldExistFlag::B0
    }
    #[doc = "exist"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AbldExistFlag::B1
    }
}
#[doc = "Deinterlacing exits flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeinterlExistFlag {
    #[doc = "0: no exist"]
    B0 = 0,
    #[doc = "1: exist"]
    B1 = 1,
}
impl From<DeinterlExistFlag> for bool {
    #[inline(always)]
    fn from(variant: DeinterlExistFlag) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEINTERL_EXIST_FLAG` reader - Deinterlacing exits flag"]
pub type DeinterlExistFlagR = crate::BitReader<DeinterlExistFlag>;
impl DeinterlExistFlagR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DeinterlExistFlag {
        match self.bits {
            false => DeinterlExistFlag::B0,
            true => DeinterlExistFlag::B1,
        }
    }
    #[doc = "no exist"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DeinterlExistFlag::B0
    }
    #[doc = "exist"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DeinterlExistFlag::B1
    }
}
#[doc = "scaling perfomance sel\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SclPerfSel {
    #[doc = "0: without scaling"]
    D0 = 0,
    #[doc = "1: low perfomance scaling"]
    D1 = 1,
    #[doc = "2: high perfomance scaling"]
    D2 = 2,
    #[doc = "3: high and fast perfomance scaling"]
    D3 = 3,
}
impl From<SclPerfSel> for u8 {
    #[inline(always)]
    fn from(variant: SclPerfSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SclPerfSel {
    type Ux = u8;
}
#[doc = "Field `SCL_PERF_SEL` reader - scaling perfomance sel"]
pub type SclPerfSelR = crate::FieldReader<SclPerfSel>;
impl SclPerfSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SclPerfSel {
        match self.bits {
            0 => SclPerfSel::D0,
            1 => SclPerfSel::D1,
            2 => SclPerfSel::D2,
            3 => SclPerfSel::D3,
            _ => unreachable!(),
        }
    }
    #[doc = "without scaling"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == SclPerfSel::D0
    }
    #[doc = "low perfomance scaling"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == SclPerfSel::D1
    }
    #[doc = "high perfomance scaling"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == SclPerfSel::D2
    }
    #[doc = "high and fast perfomance scaling"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == SclPerfSel::D3
    }
}
#[doc = "Dithering exists flag\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DitherExistFlag {
    #[doc = "0: no exist"]
    B0 = 0,
    #[doc = "1: exist"]
    B1 = 1,
}
impl From<DitherExistFlag> for bool {
    #[inline(always)]
    fn from(variant: DitherExistFlag) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DITHER_EXIST_FLAG` reader - Dithering exists flag"]
pub type DitherExistFlagR = crate::BitReader<DitherExistFlag>;
impl DitherExistFlagR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DitherExistFlag {
        match self.bits {
            false => DitherExistFlag::B0,
            true => DitherExistFlag::B1,
        }
    }
    #[doc = "no exist"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DitherExistFlag::B0
    }
    #[doc = "exist"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DitherExistFlag::B1
    }
}
#[doc = "the output of yuv422 tiled exist flag\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileExistFlag {
    #[doc = "0: no exist"]
    B0 = 0,
    #[doc = "1: exist"]
    B1 = 1,
}
impl From<TileExistFlag> for bool {
    #[inline(always)]
    fn from(variant: TileExistFlag) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TILE_EXIST_FLAG` reader - the output of yuv422 tiled exist flag"]
pub type TileExistFlagR = crate::BitReader<TileExistFlag>;
impl TileExistFlagR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TileExistFlag {
        match self.bits {
            false => TileExistFlag::B0,
            true => TileExistFlag::B1,
        }
    }
    #[doc = "no exist"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TileExistFlag::B0
    }
    #[doc = "exist"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TileExistFlag::B1
    }
}
#[doc = "Pixel Accurate PP output mode exists flag\n\nPIP:\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccutOutExistFlag {
    #[doc = "0: use 8 pixels (width) or 2 pixels (height) steps to adjust Scaling and masks"]
    B0 = 0,
    #[doc = "1: use 1 pixel for RGB and 2 pixels for subsampled chroma formats to adjust Scaling and masks"]
    B1 = 1,
}
impl From<AccutOutExistFlag> for bool {
    #[inline(always)]
    fn from(variant: AccutOutExistFlag) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACCUT_OUT_EXIST_FLAG` reader - Pixel Accurate PP output mode exists flag\n\nPIP:"]
pub type AccutOutExistFlagR = crate::BitReader<AccutOutExistFlag>;
impl AccutOutExistFlagR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AccutOutExistFlag {
        match self.bits {
            false => AccutOutExistFlag::B0,
            true => AccutOutExistFlag::B1,
        }
    }
    #[doc = "use 8 pixels (width) or 2 pixels (height) steps to adjust Scaling and masks"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AccutOutExistFlag::B0
    }
    #[doc = "use 1 pixel for RGB and 2 pixels for subsampled chroma formats to adjust Scaling and masks"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AccutOutExistFlag::B1
    }
}
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AbldCropFlag {
    #[doc = "0: unsupport crop,the exact image of the area being alpha blended should exist in the external memory"]
    B0 = 0,
    #[doc = "1: supprot crop,one picture in external memory which come from blended area can be cropped"]
    B1 = 1,
}
impl From<AbldCropFlag> for bool {
    #[inline(always)]
    fn from(variant: AbldCropFlag) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABLD_CROP_FLAG` reader - "]
pub type AbldCropFlagR = crate::BitReader<AbldCropFlag>;
impl AbldCropFlagR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AbldCropFlag {
        match self.bits {
            false => AbldCropFlag::B0,
            true => AbldCropFlag::B1,
        }
    }
    #[doc = "unsupport crop,the exact image of the area being alpha blended should exist in the external memory"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AbldCropFlag::B0
    }
    #[doc = "supprot crop,one picture in external memory which come from blended area can be cropped"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AbldCropFlag::B1
    }
}
impl R {
    #[doc = "Bits 0:10 - the max pixels width allow for pp output"]
    #[inline(always)]
    pub fn ppd_max_outw(&self) -> PpdMaxOutwR {
        PpdMaxOutwR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 14:15 - PD input tiled mode"]
    #[inline(always)]
    pub fn pp_tile_in_mode(&self) -> PpTileInModeR {
        PpTileInModeR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - PPD exists flag"]
    #[inline(always)]
    pub fn ppd_exist_flag(&self) -> PpdExistFlagR {
        PpdExistFlagR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PP output buffering select"]
    #[inline(always)]
    pub fn pp_out_buf_sel(&self) -> PpOutBufSelR {
        PpOutBufSelR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PP output endian mode select"]
    #[inline(always)]
    pub fn pp_endian_mode(&self) -> PpEndianModeR {
        PpEndianModeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 23 - PP input buffering select"]
    #[inline(always)]
    pub fn pp_in_buf_sel(&self) -> PpInBufSelR {
        PpInBufSelR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - alpha blending exists flag"]
    #[inline(always)]
    pub fn abld_exist_flag(&self) -> AbldExistFlagR {
        AbldExistFlagR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Deinterlacing exits flag"]
    #[inline(always)]
    pub fn deinterl_exist_flag(&self) -> DeinterlExistFlagR {
        DeinterlExistFlagR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - scaling perfomance sel"]
    #[inline(always)]
    pub fn scl_perf_sel(&self) -> SclPerfSelR {
        SclPerfSelR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - Dithering exists flag"]
    #[inline(always)]
    pub fn dither_exist_flag(&self) -> DitherExistFlagR {
        DitherExistFlagR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - the output of yuv422 tiled exist flag"]
    #[inline(always)]
    pub fn tile_exist_flag(&self) -> TileExistFlagR {
        TileExistFlagR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Pixel Accurate PP output mode exists flag\n\nPIP:"]
    #[inline(always)]
    pub fn accut_out_exist_flag(&self) -> AccutOutExistFlagR {
        AccutOutExistFlagR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn abld_crop_flag(&self) -> AbldCropFlagR {
        AbldCropFlagR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Synthesis configuration register post-processor (read only)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg33::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg33Spec;
impl crate::RegisterSpec for Swreg33Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg33::R`](R) reader structure"]
impl crate::Readable for Swreg33Spec {}
#[doc = "`reset()` method sets SWREG33 to value 0xfc87_4780"]
impl crate::Resettable for Swreg33Spec {
    const RESET_VALUE: u32 = 0xfc87_4780;
}
