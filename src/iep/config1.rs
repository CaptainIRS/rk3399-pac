#[doc = "Register `CONFIG1` reader"]
pub type R = crate::R<Config1Spec>;
#[doc = "Register `CONFIG1` writer"]
pub type W = crate::W<Config1Spec>;
#[doc = "Input image Format\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SrcFmt {
    #[doc = "0: XRGB"]
    B00 = 0,
    #[doc = "1: RGB565"]
    B01 = 1,
    #[doc = "2: YUV422"]
    B10 = 2,
    #[doc = "3: YUV420"]
    B11 = 3,
}
impl From<SrcFmt> for u8 {
    #[inline(always)]
    fn from(variant: SrcFmt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SrcFmt {
    type Ux = u8;
}
#[doc = "Field `SRC_FMT` reader - Input image Format"]
pub type SrcFmtR = crate::FieldReader<SrcFmt>;
impl SrcFmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SrcFmt {
        match self.bits {
            0 => SrcFmt::B00,
            1 => SrcFmt::B01,
            2 => SrcFmt::B10,
            3 => SrcFmt::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "XRGB"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == SrcFmt::B00
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == SrcFmt::B01
    }
    #[doc = "YUV422"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == SrcFmt::B10
    }
    #[doc = "YUV420"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == SrcFmt::B11
    }
}
#[doc = "Field `SRC_FMT` writer - Input image Format"]
pub type SrcFmtW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SrcFmt>;
impl<'a, REG> SrcFmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "XRGB"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(SrcFmt::B00)
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(SrcFmt::B01)
    }
    #[doc = "YUV422"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(SrcFmt::B10)
    }
    #[doc = "YUV420"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(SrcFmt::B11)
    }
}
#[doc = "source RGB swap\n\nXRGB source\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SrcRgbSwap {
    #[doc = "0: XRGB"]
    B00 = 0,
    #[doc = "1: XBGR"]
    B01 = 1,
    #[doc = "2: RGBX"]
    B10 = 2,
    #[doc = "3: BGRX RGB565 source 00,10:RGB 01,11:BGR"]
    B11 = 3,
}
impl From<SrcRgbSwap> for u8 {
    #[inline(always)]
    fn from(variant: SrcRgbSwap) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SrcRgbSwap {
    type Ux = u8;
}
#[doc = "Field `SRC_RGB_SWAP` reader - source RGB swap\n\nXRGB source"]
pub type SrcRgbSwapR = crate::FieldReader<SrcRgbSwap>;
impl SrcRgbSwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SrcRgbSwap {
        match self.bits {
            0 => SrcRgbSwap::B00,
            1 => SrcRgbSwap::B01,
            2 => SrcRgbSwap::B10,
            3 => SrcRgbSwap::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "XRGB"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == SrcRgbSwap::B00
    }
    #[doc = "XBGR"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == SrcRgbSwap::B01
    }
    #[doc = "RGBX"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == SrcRgbSwap::B10
    }
    #[doc = "BGRX RGB565 source 00,10:RGB 01,11:BGR"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == SrcRgbSwap::B11
    }
}
#[doc = "Field `SRC_RGB_SWAP` writer - source RGB swap\n\nXRGB source"]
pub type SrcRgbSwapW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SrcRgbSwap>;
impl<'a, REG> SrcRgbSwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "XRGB"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(SrcRgbSwap::B00)
    }
    #[doc = "XBGR"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(SrcRgbSwap::B01)
    }
    #[doc = "RGBX"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(SrcRgbSwap::B10)
    }
    #[doc = "BGRX RGB565 source 00,10:RGB 01,11:BGR"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(SrcRgbSwap::B11)
    }
}
#[doc = "source YUV swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SrcYuvSwap {
    #[doc = "0: SP UV"]
    B00 = 0,
    #[doc = "1: SP VU 10,"]
    B01 = 1,
    #[doc = "3: P"]
    B11 = 3,
}
impl From<SrcYuvSwap> for u8 {
    #[inline(always)]
    fn from(variant: SrcYuvSwap) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SrcYuvSwap {
    type Ux = u8;
}
#[doc = "Field `SRC_YUV_SWAP` reader - source YUV swap"]
pub type SrcYuvSwapR = crate::FieldReader<SrcYuvSwap>;
impl SrcYuvSwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SrcYuvSwap> {
        match self.bits {
            0 => Some(SrcYuvSwap::B00),
            1 => Some(SrcYuvSwap::B01),
            3 => Some(SrcYuvSwap::B11),
            _ => None,
        }
    }
    #[doc = "SP UV"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == SrcYuvSwap::B00
    }
    #[doc = "SP VU 10,"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == SrcYuvSwap::B01
    }
    #[doc = "P"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == SrcYuvSwap::B11
    }
}
#[doc = "Field `SRC_YUV_SWAP` writer - source YUV swap"]
pub type SrcYuvSwapW<'a, REG> = crate::FieldWriter<'a, REG, 2, SrcYuvSwap>;
impl<'a, REG> SrcYuvSwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SP UV"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(SrcYuvSwap::B00)
    }
    #[doc = "SP VU 10,"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(SrcYuvSwap::B01)
    }
    #[doc = "P"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(SrcYuvSwap::B11)
    }
}
#[doc = "Ouput image Format\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DstFmt {
    #[doc = "0: ARGB"]
    B00 = 0,
    #[doc = "1: RGB565"]
    B01 = 1,
    #[doc = "2: YUV422"]
    B10 = 2,
    #[doc = "3: YUV420"]
    B11 = 3,
}
impl From<DstFmt> for u8 {
    #[inline(always)]
    fn from(variant: DstFmt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DstFmt {
    type Ux = u8;
}
#[doc = "Field `DST_FMT` reader - Ouput image Format"]
pub type DstFmtR = crate::FieldReader<DstFmt>;
impl DstFmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DstFmt {
        match self.bits {
            0 => DstFmt::B00,
            1 => DstFmt::B01,
            2 => DstFmt::B10,
            3 => DstFmt::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "ARGB"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == DstFmt::B00
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == DstFmt::B01
    }
    #[doc = "YUV422"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == DstFmt::B10
    }
    #[doc = "YUV420"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == DstFmt::B11
    }
}
#[doc = "Field `DST_FMT` writer - Ouput image Format"]
pub type DstFmtW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DstFmt>;
impl<'a, REG> DstFmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ARGB"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(DstFmt::B00)
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(DstFmt::B01)
    }
    #[doc = "YUV422"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(DstFmt::B10)
    }
    #[doc = "YUV420"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(DstFmt::B11)
    }
}
#[doc = "destination RGB swap\n\nARGB destination\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DstRgbSwap {
    #[doc = "0: ARGB"]
    B00 = 0,
    #[doc = "1: ABGR"]
    B01 = 1,
    #[doc = "2: RGBA"]
    B10 = 2,
    #[doc = "3: BGRA RGB565 destination 00,10:RGB 01,11:BGR"]
    B11 = 3,
}
impl From<DstRgbSwap> for u8 {
    #[inline(always)]
    fn from(variant: DstRgbSwap) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DstRgbSwap {
    type Ux = u8;
}
#[doc = "Field `DST_RGB_SWAP` reader - destination RGB swap\n\nARGB destination"]
pub type DstRgbSwapR = crate::FieldReader<DstRgbSwap>;
impl DstRgbSwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DstRgbSwap {
        match self.bits {
            0 => DstRgbSwap::B00,
            1 => DstRgbSwap::B01,
            2 => DstRgbSwap::B10,
            3 => DstRgbSwap::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "ARGB"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == DstRgbSwap::B00
    }
    #[doc = "ABGR"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == DstRgbSwap::B01
    }
    #[doc = "RGBA"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == DstRgbSwap::B10
    }
    #[doc = "BGRA RGB565 destination 00,10:RGB 01,11:BGR"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == DstRgbSwap::B11
    }
}
#[doc = "Field `DST_RGB_SWAP` writer - destination RGB swap\n\nARGB destination"]
pub type DstRgbSwapW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DstRgbSwap>;
impl<'a, REG> DstRgbSwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ARGB"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(DstRgbSwap::B00)
    }
    #[doc = "ABGR"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(DstRgbSwap::B01)
    }
    #[doc = "RGBA"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(DstRgbSwap::B10)
    }
    #[doc = "BGRA RGB565 destination 00,10:RGB 01,11:BGR"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(DstRgbSwap::B11)
    }
}
#[doc = "destination YUV swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DstYuvSwap {
    #[doc = "0: SP UV"]
    B00 = 0,
    #[doc = "1: SP VU 10,"]
    B01 = 1,
    #[doc = "3: P"]
    B11 = 3,
}
impl From<DstYuvSwap> for u8 {
    #[inline(always)]
    fn from(variant: DstYuvSwap) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DstYuvSwap {
    type Ux = u8;
}
#[doc = "Field `DST_YUV_SWAP` reader - destination YUV swap"]
pub type DstYuvSwapR = crate::FieldReader<DstYuvSwap>;
impl DstYuvSwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DstYuvSwap> {
        match self.bits {
            0 => Some(DstYuvSwap::B00),
            1 => Some(DstYuvSwap::B01),
            3 => Some(DstYuvSwap::B11),
            _ => None,
        }
    }
    #[doc = "SP UV"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == DstYuvSwap::B00
    }
    #[doc = "SP VU 10,"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == DstYuvSwap::B01
    }
    #[doc = "P"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == DstYuvSwap::B11
    }
}
#[doc = "Field `DST_YUV_SWAP` writer - destination YUV swap"]
pub type DstYuvSwapW<'a, REG> = crate::FieldWriter<'a, REG, 2, DstYuvSwap>;
impl<'a, REG> DstYuvSwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SP UV"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(DstYuvSwap::B00)
    }
    #[doc = "SP VU 10,"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(DstYuvSwap::B01)
    }
    #[doc = "P"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(DstYuvSwap::B11)
    }
}
#[doc = "dither up enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DthrUpEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<DthrUpEn> for bool {
    #[inline(always)]
    fn from(variant: DthrUpEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTHR_UP_EN` reader - dither up enable"]
pub type DthrUpEnR = crate::BitReader<DthrUpEn>;
impl DthrUpEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DthrUpEn {
        match self.bits {
            false => DthrUpEn::B0,
            true => DthrUpEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DthrUpEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DthrUpEn::B1
    }
}
#[doc = "Field `DTHR_UP_EN` writer - dither up enable"]
pub type DthrUpEnW<'a, REG> = crate::BitWriter<'a, REG, DthrUpEn>;
impl<'a, REG> DthrUpEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DthrUpEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DthrUpEn::B1)
    }
}
#[doc = "dither down enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DthrDownEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<DthrDownEn> for bool {
    #[inline(always)]
    fn from(variant: DthrDownEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTHR_DOWN_EN` reader - dither down enable"]
pub type DthrDownEnR = crate::BitReader<DthrDownEn>;
impl DthrDownEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DthrDownEn {
        match self.bits {
            false => DthrDownEn::B0,
            true => DthrDownEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DthrDownEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DthrDownEn::B1
    }
}
#[doc = "Field `DTHR_DOWN_EN` writer - dither down enable"]
pub type DthrDownEnW<'a, REG> = crate::BitWriter<'a, REG, DthrDownEn>;
impl<'a, REG> DthrDownEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DthrDownEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DthrDownEn::B1)
    }
}
#[doc = "yuv2rgb coefficient select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Yuv2rgbCoeSel {
    #[doc = "0: bt601_1"]
    B00 = 0,
    #[doc = "1: bt601_f"]
    B01 = 1,
    #[doc = "2: bt709_1"]
    B10 = 2,
    #[doc = "3: bt709_f"]
    B11 = 3,
}
impl From<Yuv2rgbCoeSel> for u8 {
    #[inline(always)]
    fn from(variant: Yuv2rgbCoeSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Yuv2rgbCoeSel {
    type Ux = u8;
}
#[doc = "Field `YUV2RGB_COE_SEL` reader - yuv2rgb coefficient select"]
pub type Yuv2rgbCoeSelR = crate::FieldReader<Yuv2rgbCoeSel>;
impl Yuv2rgbCoeSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Yuv2rgbCoeSel {
        match self.bits {
            0 => Yuv2rgbCoeSel::B00,
            1 => Yuv2rgbCoeSel::B01,
            2 => Yuv2rgbCoeSel::B10,
            3 => Yuv2rgbCoeSel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "bt601_1"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Yuv2rgbCoeSel::B00
    }
    #[doc = "bt601_f"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Yuv2rgbCoeSel::B01
    }
    #[doc = "bt709_1"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Yuv2rgbCoeSel::B10
    }
    #[doc = "bt709_f"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Yuv2rgbCoeSel::B11
    }
}
#[doc = "Field `YUV2RGB_COE_SEL` writer - yuv2rgb coefficient select"]
pub type Yuv2rgbCoeSelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Yuv2rgbCoeSel>;
impl<'a, REG> Yuv2rgbCoeSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "bt601_1"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Yuv2rgbCoeSel::B00)
    }
    #[doc = "bt601_f"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Yuv2rgbCoeSel::B01)
    }
    #[doc = "bt709_1"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Yuv2rgbCoeSel::B10)
    }
    #[doc = "bt709_f"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Yuv2rgbCoeSel::B11)
    }
}
#[doc = "rgb2yuv coefficient select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rgb2yuvCoeSel {
    #[doc = "0: bt601_1"]
    B00 = 0,
    #[doc = "1: bt601_f"]
    B01 = 1,
    #[doc = "2: bt709_1"]
    B10 = 2,
    #[doc = "3: bt709_f"]
    B11 = 3,
}
impl From<Rgb2yuvCoeSel> for u8 {
    #[inline(always)]
    fn from(variant: Rgb2yuvCoeSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rgb2yuvCoeSel {
    type Ux = u8;
}
#[doc = "Field `RGB2YUV_COE_SEL` reader - rgb2yuv coefficient select"]
pub type Rgb2yuvCoeSelR = crate::FieldReader<Rgb2yuvCoeSel>;
impl Rgb2yuvCoeSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rgb2yuvCoeSel {
        match self.bits {
            0 => Rgb2yuvCoeSel::B00,
            1 => Rgb2yuvCoeSel::B01,
            2 => Rgb2yuvCoeSel::B10,
            3 => Rgb2yuvCoeSel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "bt601_1"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Rgb2yuvCoeSel::B00
    }
    #[doc = "bt601_f"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Rgb2yuvCoeSel::B01
    }
    #[doc = "bt709_1"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Rgb2yuvCoeSel::B10
    }
    #[doc = "bt709_f"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Rgb2yuvCoeSel::B11
    }
}
#[doc = "Field `RGB2YUV_COE_SEL` writer - rgb2yuv coefficient select"]
pub type Rgb2yuvCoeSelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Rgb2yuvCoeSel>;
impl<'a, REG> Rgb2yuvCoeSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "bt601_1"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Rgb2yuvCoeSel::B00)
    }
    #[doc = "bt601_f"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Rgb2yuvCoeSel::B01)
    }
    #[doc = "bt709_1"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Rgb2yuvCoeSel::B10)
    }
    #[doc = "bt709_f"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Rgb2yuvCoeSel::B11)
    }
}
#[doc = "YUV to RGB conversion enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum YuvToRgbEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<YuvToRgbEn> for bool {
    #[inline(always)]
    fn from(variant: YuvToRgbEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `YUV_TO_RGB_EN` reader - YUV to RGB conversion enable"]
pub type YuvToRgbEnR = crate::BitReader<YuvToRgbEn>;
impl YuvToRgbEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> YuvToRgbEn {
        match self.bits {
            false => YuvToRgbEn::B0,
            true => YuvToRgbEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == YuvToRgbEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == YuvToRgbEn::B1
    }
}
#[doc = "Field `YUV_TO_RGB_EN` writer - YUV to RGB conversion enable"]
pub type YuvToRgbEnW<'a, REG> = crate::BitWriter<'a, REG, YuvToRgbEn>;
impl<'a, REG> YuvToRgbEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(YuvToRgbEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(YuvToRgbEn::B1)
    }
}
#[doc = "RGB to YUV conversion enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RgbToYuvEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<RgbToYuvEn> for bool {
    #[inline(always)]
    fn from(variant: RgbToYuvEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGB_TO_YUV_EN` reader - RGB to YUV conversion enable"]
pub type RgbToYuvEnR = crate::BitReader<RgbToYuvEn>;
impl RgbToYuvEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RgbToYuvEn {
        match self.bits {
            false => RgbToYuvEn::B0,
            true => RgbToYuvEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RgbToYuvEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RgbToYuvEn::B1
    }
}
#[doc = "Field `RGB_TO_YUV_EN` writer - RGB to YUV conversion enable"]
pub type RgbToYuvEnW<'a, REG> = crate::BitWriter<'a, REG, RgbToYuvEn>;
impl<'a, REG> RgbToYuvEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(RgbToYuvEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(RgbToYuvEn::B1)
    }
}
#[doc = "YUV to RGB input range\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Yuv2rgbInputClip {
    #[doc = "0: Y/U/V=\\[0,255\\]"]
    B0 = 0,
    #[doc = "1: Y=\\[16,235\\],U/V=\\[16,240\\]"]
    B1 = 1,
}
impl From<Yuv2rgbInputClip> for bool {
    #[inline(always)]
    fn from(variant: Yuv2rgbInputClip) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `YUV2RGB_INPUT_CLIP` reader - YUV to RGB input range"]
pub type Yuv2rgbInputClipR = crate::BitReader<Yuv2rgbInputClip>;
impl Yuv2rgbInputClipR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Yuv2rgbInputClip {
        match self.bits {
            false => Yuv2rgbInputClip::B0,
            true => Yuv2rgbInputClip::B1,
        }
    }
    #[doc = "Y/U/V=\\[0,255\\]"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Yuv2rgbInputClip::B0
    }
    #[doc = "Y=\\[16,235\\],U/V=\\[16,240\\]"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Yuv2rgbInputClip::B1
    }
}
#[doc = "Field `YUV2RGB_INPUT_CLIP` writer - YUV to RGB input range"]
pub type Yuv2rgbInputClipW<'a, REG> = crate::BitWriter<'a, REG, Yuv2rgbInputClip>;
impl<'a, REG> Yuv2rgbInputClipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Y/U/V=\\[0,255\\]"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Yuv2rgbInputClip::B0)
    }
    #[doc = "Y=\\[16,235\\],U/V=\\[16,240\\]"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Yuv2rgbInputClip::B1)
    }
}
#[doc = "RGB to YUV input range\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rgb2yuvInputClip {
    #[doc = "0: R/G/B=\\[0,255\\]"]
    B0 = 0,
    #[doc = "1: R/G/B=\\[16,235\\]"]
    B1 = 1,
}
impl From<Rgb2yuvInputClip> for bool {
    #[inline(always)]
    fn from(variant: Rgb2yuvInputClip) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGB2YUV_INPUT_CLIP` reader - RGB to YUV input range"]
pub type Rgb2yuvInputClipR = crate::BitReader<Rgb2yuvInputClip>;
impl Rgb2yuvInputClipR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rgb2yuvInputClip {
        match self.bits {
            false => Rgb2yuvInputClip::B0,
            true => Rgb2yuvInputClip::B1,
        }
    }
    #[doc = "R/G/B=\\[0,255\\]"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rgb2yuvInputClip::B0
    }
    #[doc = "R/G/B=\\[16,235\\]"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rgb2yuvInputClip::B1
    }
}
#[doc = "Field `RGB2YUV_INPUT_CLIP` writer - RGB to YUV input range"]
pub type Rgb2yuvInputClipW<'a, REG> = crate::BitWriter<'a, REG, Rgb2yuvInputClip>;
impl<'a, REG> Rgb2yuvInputClipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "R/G/B=\\[0,255\\]"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rgb2yuvInputClip::B0)
    }
    #[doc = "R/G/B=\\[16,235\\]"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rgb2yuvInputClip::B1)
    }
}
#[doc = "Field `GLB_ALPHA` reader - global alpha value\n\nonly valid when destination format is ARGB"]
pub type GlbAlphaR = crate::FieldReader;
#[doc = "Field `GLB_ALPHA` writer - global alpha value\n\nonly valid when destination format is ARGB"]
pub type GlbAlphaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - Input image Format"]
    #[inline(always)]
    pub fn src_fmt(&self) -> SrcFmtR {
        SrcFmtR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - source RGB swap\n\nXRGB source"]
    #[inline(always)]
    pub fn src_rgb_swap(&self) -> SrcRgbSwapR {
        SrcRgbSwapR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - source YUV swap"]
    #[inline(always)]
    pub fn src_yuv_swap(&self) -> SrcYuvSwapR {
        SrcYuvSwapR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Ouput image Format"]
    #[inline(always)]
    pub fn dst_fmt(&self) -> DstFmtR {
        DstFmtR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - destination RGB swap\n\nARGB destination"]
    #[inline(always)]
    pub fn dst_rgb_swap(&self) -> DstRgbSwapR {
        DstRgbSwapR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - destination YUV swap"]
    #[inline(always)]
    pub fn dst_yuv_swap(&self) -> DstYuvSwapR {
        DstYuvSwapR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - dither up enable"]
    #[inline(always)]
    pub fn dthr_up_en(&self) -> DthrUpEnR {
        DthrUpEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - dither down enable"]
    #[inline(always)]
    pub fn dthr_down_en(&self) -> DthrDownEnR {
        DthrDownEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - yuv2rgb coefficient select"]
    #[inline(always)]
    pub fn yuv2rgb_coe_sel(&self) -> Yuv2rgbCoeSelR {
        Yuv2rgbCoeSelR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - rgb2yuv coefficient select"]
    #[inline(always)]
    pub fn rgb2yuv_coe_sel(&self) -> Rgb2yuvCoeSelR {
        Rgb2yuvCoeSelR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - YUV to RGB conversion enable"]
    #[inline(always)]
    pub fn yuv_to_rgb_en(&self) -> YuvToRgbEnR {
        YuvToRgbEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RGB to YUV conversion enable"]
    #[inline(always)]
    pub fn rgb_to_yuv_en(&self) -> RgbToYuvEnR {
        RgbToYuvEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - YUV to RGB input range"]
    #[inline(always)]
    pub fn yuv2rgb_input_clip(&self) -> Yuv2rgbInputClipR {
        Yuv2rgbInputClipR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - RGB to YUV input range"]
    #[inline(always)]
    pub fn rgb2yuv_input_clip(&self) -> Rgb2yuvInputClipR {
        Rgb2yuvInputClipR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - global alpha value\n\nonly valid when destination format is ARGB"]
    #[inline(always)]
    pub fn glb_alpha(&self) -> GlbAlphaR {
        GlbAlphaR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Input image Format"]
    #[inline(always)]
    #[must_use]
    pub fn src_fmt(&mut self) -> SrcFmtW<Config1Spec> {
        SrcFmtW::new(self, 0)
    }
    #[doc = "Bits 2:3 - source RGB swap\n\nXRGB source"]
    #[inline(always)]
    #[must_use]
    pub fn src_rgb_swap(&mut self) -> SrcRgbSwapW<Config1Spec> {
        SrcRgbSwapW::new(self, 2)
    }
    #[doc = "Bits 4:5 - source YUV swap"]
    #[inline(always)]
    #[must_use]
    pub fn src_yuv_swap(&mut self) -> SrcYuvSwapW<Config1Spec> {
        SrcYuvSwapW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Ouput image Format"]
    #[inline(always)]
    #[must_use]
    pub fn dst_fmt(&mut self) -> DstFmtW<Config1Spec> {
        DstFmtW::new(self, 8)
    }
    #[doc = "Bits 10:11 - destination RGB swap\n\nARGB destination"]
    #[inline(always)]
    #[must_use]
    pub fn dst_rgb_swap(&mut self) -> DstRgbSwapW<Config1Spec> {
        DstRgbSwapW::new(self, 10)
    }
    #[doc = "Bits 12:13 - destination YUV swap"]
    #[inline(always)]
    #[must_use]
    pub fn dst_yuv_swap(&mut self) -> DstYuvSwapW<Config1Spec> {
        DstYuvSwapW::new(self, 12)
    }
    #[doc = "Bit 14 - dither up enable"]
    #[inline(always)]
    #[must_use]
    pub fn dthr_up_en(&mut self) -> DthrUpEnW<Config1Spec> {
        DthrUpEnW::new(self, 14)
    }
    #[doc = "Bit 15 - dither down enable"]
    #[inline(always)]
    #[must_use]
    pub fn dthr_down_en(&mut self) -> DthrDownEnW<Config1Spec> {
        DthrDownEnW::new(self, 15)
    }
    #[doc = "Bits 16:17 - yuv2rgb coefficient select"]
    #[inline(always)]
    #[must_use]
    pub fn yuv2rgb_coe_sel(&mut self) -> Yuv2rgbCoeSelW<Config1Spec> {
        Yuv2rgbCoeSelW::new(self, 16)
    }
    #[doc = "Bits 18:19 - rgb2yuv coefficient select"]
    #[inline(always)]
    #[must_use]
    pub fn rgb2yuv_coe_sel(&mut self) -> Rgb2yuvCoeSelW<Config1Spec> {
        Rgb2yuvCoeSelW::new(self, 18)
    }
    #[doc = "Bit 20 - YUV to RGB conversion enable"]
    #[inline(always)]
    #[must_use]
    pub fn yuv_to_rgb_en(&mut self) -> YuvToRgbEnW<Config1Spec> {
        YuvToRgbEnW::new(self, 20)
    }
    #[doc = "Bit 21 - RGB to YUV conversion enable"]
    #[inline(always)]
    #[must_use]
    pub fn rgb_to_yuv_en(&mut self) -> RgbToYuvEnW<Config1Spec> {
        RgbToYuvEnW::new(self, 21)
    }
    #[doc = "Bit 22 - YUV to RGB input range"]
    #[inline(always)]
    #[must_use]
    pub fn yuv2rgb_input_clip(&mut self) -> Yuv2rgbInputClipW<Config1Spec> {
        Yuv2rgbInputClipW::new(self, 22)
    }
    #[doc = "Bit 23 - RGB to YUV input range"]
    #[inline(always)]
    #[must_use]
    pub fn rgb2yuv_input_clip(&mut self) -> Rgb2yuvInputClipW<Config1Spec> {
        Rgb2yuvInputClipW::new(self, 23)
    }
    #[doc = "Bits 24:31 - global alpha value\n\nonly valid when destination format is ARGB"]
    #[inline(always)]
    #[must_use]
    pub fn glb_alpha(&mut self) -> GlbAlphaW<Config1Spec> {
        GlbAlphaW::new(self, 24)
    }
}
#[doc = "configuration register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Config1Spec;
impl crate::RegisterSpec for Config1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config1::R`](R) reader structure"]
impl crate::Readable for Config1Spec {}
#[doc = "`write(|w| ..)` method takes [`config1::W`](W) writer structure"]
impl crate::Writable for Config1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG1 to value 0"]
impl crate::Resettable for Config1Spec {
    const RESET_VALUE: u32 = 0;
}
