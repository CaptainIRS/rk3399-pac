#[doc = "Register `VI_ICCL` reader"]
pub type R = crate::R<ViIcclSpec>;
#[doc = "Register `VI_ICCL` writer"]
pub type W = crate::W<ViIcclSpec>;
#[doc = "isp processing clock enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViIspClkEnable {
    #[doc = "1: processing mode"]
    B1 = 1,
    #[doc = "0: power safe"]
    B0 = 0,
}
impl From<ViIspClkEnable> for bool {
    #[inline(always)]
    fn from(variant: ViIspClkEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `vi_isp_clk_enable` reader - isp processing clock enable"]
pub type ViIspClkEnableR = crate::BitReader<ViIspClkEnable>;
impl ViIspClkEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ViIspClkEnable {
        match self.bits {
            true => ViIspClkEnable::B1,
            false => ViIspClkEnable::B0,
        }
    }
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ViIspClkEnable::B1
    }
    #[doc = "power safe"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ViIspClkEnable::B0
    }
}
#[doc = "Field `vi_isp_clk_enable` writer - isp processing clock enable"]
pub type ViIspClkEnableW<'a, REG> = crate::BitWriter<'a, REG, ViIspClkEnable>;
impl<'a, REG> ViIspClkEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ViIspClkEnable::B1)
    }
    #[doc = "power safe"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ViIspClkEnable::B0)
    }
}
#[doc = "color processing clock enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViCpClkEnable {
    #[doc = "1: processing mode"]
    B1 = 1,
    #[doc = "0: power safe"]
    B0 = 0,
}
impl From<ViCpClkEnable> for bool {
    #[inline(always)]
    fn from(variant: ViCpClkEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `vi_cp_clk_enable` reader - color processing clock enable"]
pub type ViCpClkEnableR = crate::BitReader<ViCpClkEnable>;
impl ViCpClkEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ViCpClkEnable {
        match self.bits {
            true => ViCpClkEnable::B1,
            false => ViCpClkEnable::B0,
        }
    }
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ViCpClkEnable::B1
    }
    #[doc = "power safe"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ViCpClkEnable::B0
    }
}
#[doc = "Field `vi_cp_clk_enable` writer - color processing clock enable"]
pub type ViCpClkEnableW<'a, REG> = crate::BitWriter<'a, REG, ViCpClkEnable>;
impl<'a, REG> ViCpClkEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ViCpClkEnable::B1)
    }
    #[doc = "power safe"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ViCpClkEnable::B0)
    }
}
#[doc = "main picture resize clock enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViMrszClkEnable {
    #[doc = "1: processing mode"]
    B1 = 1,
    #[doc = "0: power safe"]
    B0 = 0,
}
impl From<ViMrszClkEnable> for bool {
    #[inline(always)]
    fn from(variant: ViMrszClkEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `vi_mrsz_clk_enable` reader - main picture resize clock enable"]
pub type ViMrszClkEnableR = crate::BitReader<ViMrszClkEnable>;
impl ViMrszClkEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ViMrszClkEnable {
        match self.bits {
            true => ViMrszClkEnable::B1,
            false => ViMrszClkEnable::B0,
        }
    }
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ViMrszClkEnable::B1
    }
    #[doc = "power safe"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ViMrszClkEnable::B0
    }
}
#[doc = "Field `vi_mrsz_clk_enable` writer - main picture resize clock enable"]
pub type ViMrszClkEnableW<'a, REG> = crate::BitWriter<'a, REG, ViMrszClkEnable>;
impl<'a, REG> ViMrszClkEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ViMrszClkEnable::B1)
    }
    #[doc = "power safe"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ViMrszClkEnable::B0)
    }
}
#[doc = "self picture resize clock enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViSrszClkEnable {
    #[doc = "1: processing mode"]
    B1 = 1,
    #[doc = "0: power safe"]
    B0 = 0,
}
impl From<ViSrszClkEnable> for bool {
    #[inline(always)]
    fn from(variant: ViSrszClkEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `vi_srsz_clk_enable` reader - self picture resize clock enable"]
pub type ViSrszClkEnableR = crate::BitReader<ViSrszClkEnable>;
impl ViSrszClkEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ViSrszClkEnable {
        match self.bits {
            true => ViSrszClkEnable::B1,
            false => ViSrszClkEnable::B0,
        }
    }
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ViSrszClkEnable::B1
    }
    #[doc = "power safe"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ViSrszClkEnable::B0
    }
}
#[doc = "Field `vi_srsz_clk_enable` writer - self picture resize clock enable"]
pub type ViSrszClkEnableW<'a, REG> = crate::BitWriter<'a, REG, ViSrszClkEnable>;
impl<'a, REG> ViSrszClkEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ViSrszClkEnable::B1)
    }
    #[doc = "power safe"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ViSrszClkEnable::B0)
    }
}
#[doc = "JPEG encoder clock enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViJpegClkEnable {
    #[doc = "1: processing mode"]
    B1 = 1,
    #[doc = "0: power safe"]
    B0 = 0,
}
impl From<ViJpegClkEnable> for bool {
    #[inline(always)]
    fn from(variant: ViJpegClkEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `vi_jpeg_clk_enable` reader - JPEG encoder clock enable"]
pub type ViJpegClkEnableR = crate::BitReader<ViJpegClkEnable>;
impl ViJpegClkEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ViJpegClkEnable {
        match self.bits {
            true => ViJpegClkEnable::B1,
            false => ViJpegClkEnable::B0,
        }
    }
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ViJpegClkEnable::B1
    }
    #[doc = "power safe"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ViJpegClkEnable::B0
    }
}
#[doc = "Field `vi_jpeg_clk_enable` writer - JPEG encoder clock enable"]
pub type ViJpegClkEnableW<'a, REG> = crate::BitWriter<'a, REG, ViJpegClkEnable>;
impl<'a, REG> ViJpegClkEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ViJpegClkEnable::B1)
    }
    #[doc = "power safe"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ViJpegClkEnable::B0)
    }
}
#[doc = "memory interface clock enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViMiClkEnable {
    #[doc = "1: processing mode"]
    B1 = 1,
    #[doc = "0: power safe"]
    B0 = 0,
}
impl From<ViMiClkEnable> for bool {
    #[inline(always)]
    fn from(variant: ViMiClkEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `vi_mi_clk_enable` reader - memory interface clock enable"]
pub type ViMiClkEnableR = crate::BitReader<ViMiClkEnable>;
impl ViMiClkEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ViMiClkEnable {
        match self.bits {
            true => ViMiClkEnable::B1,
            false => ViMiClkEnable::B0,
        }
    }
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ViMiClkEnable::B1
    }
    #[doc = "power safe"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ViMiClkEnable::B0
    }
}
#[doc = "Field `vi_mi_clk_enable` writer - memory interface clock enable"]
pub type ViMiClkEnableW<'a, REG> = crate::BitWriter<'a, REG, ViMiClkEnable>;
impl<'a, REG> ViMiClkEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ViMiClkEnable::B1)
    }
    #[doc = "power safe"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ViMiClkEnable::B0)
    }
}
#[doc = "Image effect clock enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViIeClkEnable {
    #[doc = "1: processing mode"]
    B1 = 1,
    #[doc = "0: power safe"]
    B0 = 0,
}
impl From<ViIeClkEnable> for bool {
    #[inline(always)]
    fn from(variant: ViIeClkEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `vi_ie_clk_enable` reader - Image effect clock enable"]
pub type ViIeClkEnableR = crate::BitReader<ViIeClkEnable>;
impl ViIeClkEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ViIeClkEnable {
        match self.bits {
            true => ViIeClkEnable::B1,
            false => ViIeClkEnable::B0,
        }
    }
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ViIeClkEnable::B1
    }
    #[doc = "power safe"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ViIeClkEnable::B0
    }
}
#[doc = "Field `vi_ie_clk_enable` writer - Image effect clock enable"]
pub type ViIeClkEnableW<'a, REG> = crate::BitWriter<'a, REG, ViIeClkEnable>;
impl<'a, REG> ViIeClkEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ViIeClkEnable::B1)
    }
    #[doc = "power safe"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ViIeClkEnable::B0)
    }
}
#[doc = "Superimpose clock enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViSimpClkEnable {
    #[doc = "1: processing mode"]
    B1 = 1,
    #[doc = "0: power safe"]
    B0 = 0,
}
impl From<ViSimpClkEnable> for bool {
    #[inline(always)]
    fn from(variant: ViSimpClkEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `vi_simp_clk_enable` reader - Superimpose clock enable"]
pub type ViSimpClkEnableR = crate::BitReader<ViSimpClkEnable>;
impl ViSimpClkEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ViSimpClkEnable {
        match self.bits {
            true => ViSimpClkEnable::B1,
            false => ViSimpClkEnable::B0,
        }
    }
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ViSimpClkEnable::B1
    }
    #[doc = "power safe"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ViSimpClkEnable::B0
    }
}
#[doc = "Field `vi_simp_clk_enable` writer - Superimpose clock enable"]
pub type ViSimpClkEnableW<'a, REG> = crate::BitWriter<'a, REG, ViSimpClkEnable>;
impl<'a, REG> ViSimpClkEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ViSimpClkEnable::B1)
    }
    #[doc = "power safe"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ViSimpClkEnable::B0)
    }
}
#[doc = "SMIA interface clock enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViSmiaClkEnable {
    #[doc = "1: processing mode"]
    B1 = 1,
    #[doc = "0: power safe"]
    B0 = 0,
}
impl From<ViSmiaClkEnable> for bool {
    #[inline(always)]
    fn from(variant: ViSmiaClkEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `vi_smia_clk_enable` reader - SMIA interface clock enable"]
pub type ViSmiaClkEnableR = crate::BitReader<ViSmiaClkEnable>;
impl ViSmiaClkEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ViSmiaClkEnable {
        match self.bits {
            true => ViSmiaClkEnable::B1,
            false => ViSmiaClkEnable::B0,
        }
    }
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ViSmiaClkEnable::B1
    }
    #[doc = "power safe"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ViSmiaClkEnable::B0
    }
}
#[doc = "Field `vi_smia_clk_enable` writer - SMIA interface clock enable"]
pub type ViSmiaClkEnableW<'a, REG> = crate::BitWriter<'a, REG, ViSmiaClkEnable>;
impl<'a, REG> ViSmiaClkEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ViSmiaClkEnable::B1)
    }
    #[doc = "power safe"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ViSmiaClkEnable::B0)
    }
}
#[doc = "MIPI interface clock\n\nenable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViMipiClkEnable {
    #[doc = "1: processing mode"]
    B1 = 1,
    #[doc = "0: power safe"]
    B0 = 0,
}
impl From<ViMipiClkEnable> for bool {
    #[inline(always)]
    fn from(variant: ViMipiClkEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `vi_mipi_clk_enable` reader - MIPI interface clock\n\nenable"]
pub type ViMipiClkEnableR = crate::BitReader<ViMipiClkEnable>;
impl ViMipiClkEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ViMipiClkEnable {
        match self.bits {
            true => ViMipiClkEnable::B1,
            false => ViMipiClkEnable::B0,
        }
    }
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ViMipiClkEnable::B1
    }
    #[doc = "power safe"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ViMipiClkEnable::B0
    }
}
#[doc = "Field `vi_mipi_clk_enable` writer - MIPI interface clock\n\nenable"]
pub type ViMipiClkEnableW<'a, REG> = crate::BitWriter<'a, REG, ViMipiClkEnable>;
impl<'a, REG> ViMipiClkEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ViMipiClkEnable::B1)
    }
    #[doc = "power safe"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ViMipiClkEnable::B0)
    }
}
impl R {
    #[doc = "Bit 0 - isp processing clock enable"]
    #[inline(always)]
    pub fn vi_isp_clk_enable(&self) -> ViIspClkEnableR {
        ViIspClkEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - color processing clock enable"]
    #[inline(always)]
    pub fn vi_cp_clk_enable(&self) -> ViCpClkEnableR {
        ViCpClkEnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - main picture resize clock enable"]
    #[inline(always)]
    pub fn vi_mrsz_clk_enable(&self) -> ViMrszClkEnableR {
        ViMrszClkEnableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - self picture resize clock enable"]
    #[inline(always)]
    pub fn vi_srsz_clk_enable(&self) -> ViSrszClkEnableR {
        ViSrszClkEnableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - JPEG encoder clock enable"]
    #[inline(always)]
    pub fn vi_jpeg_clk_enable(&self) -> ViJpegClkEnableR {
        ViJpegClkEnableR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - memory interface clock enable"]
    #[inline(always)]
    pub fn vi_mi_clk_enable(&self) -> ViMiClkEnableR {
        ViMiClkEnableR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Image effect clock enable"]
    #[inline(always)]
    pub fn vi_ie_clk_enable(&self) -> ViIeClkEnableR {
        ViIeClkEnableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Superimpose clock enable"]
    #[inline(always)]
    pub fn vi_simp_clk_enable(&self) -> ViSimpClkEnableR {
        ViSimpClkEnableR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SMIA interface clock enable"]
    #[inline(always)]
    pub fn vi_smia_clk_enable(&self) -> ViSmiaClkEnableR {
        ViSmiaClkEnableR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MIPI interface clock\n\nenable"]
    #[inline(always)]
    pub fn vi_mipi_clk_enable(&self) -> ViMipiClkEnableR {
        ViMipiClkEnableR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - isp processing clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn vi_isp_clk_enable(&mut self) -> ViIspClkEnableW<ViIcclSpec> {
        ViIspClkEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - color processing clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn vi_cp_clk_enable(&mut self) -> ViCpClkEnableW<ViIcclSpec> {
        ViCpClkEnableW::new(self, 1)
    }
    #[doc = "Bit 3 - main picture resize clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn vi_mrsz_clk_enable(&mut self) -> ViMrszClkEnableW<ViIcclSpec> {
        ViMrszClkEnableW::new(self, 3)
    }
    #[doc = "Bit 4 - self picture resize clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn vi_srsz_clk_enable(&mut self) -> ViSrszClkEnableW<ViIcclSpec> {
        ViSrszClkEnableW::new(self, 4)
    }
    #[doc = "Bit 5 - JPEG encoder clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn vi_jpeg_clk_enable(&mut self) -> ViJpegClkEnableW<ViIcclSpec> {
        ViJpegClkEnableW::new(self, 5)
    }
    #[doc = "Bit 6 - memory interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn vi_mi_clk_enable(&mut self) -> ViMiClkEnableW<ViIcclSpec> {
        ViMiClkEnableW::new(self, 6)
    }
    #[doc = "Bit 8 - Image effect clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn vi_ie_clk_enable(&mut self) -> ViIeClkEnableW<ViIcclSpec> {
        ViIeClkEnableW::new(self, 8)
    }
    #[doc = "Bit 9 - Superimpose clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn vi_simp_clk_enable(&mut self) -> ViSimpClkEnableW<ViIcclSpec> {
        ViSimpClkEnableW::new(self, 9)
    }
    #[doc = "Bit 10 - SMIA interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn vi_smia_clk_enable(&mut self) -> ViSmiaClkEnableW<ViIcclSpec> {
        ViSmiaClkEnableW::new(self, 10)
    }
    #[doc = "Bit 11 - MIPI interface clock\n\nenable"]
    #[inline(always)]
    #[must_use]
    pub fn vi_mipi_clk_enable(&mut self) -> ViMipiClkEnableW<ViIcclSpec> {
        ViMipiClkEnableW::new(self, 11)
    }
}
#[doc = "Internal clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vi_iccl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vi_iccl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ViIcclSpec;
impl crate::RegisterSpec for ViIcclSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vi_iccl::R`](R) reader structure"]
impl crate::Readable for ViIcclSpec {}
#[doc = "`write(|w| ..)` method takes [`vi_iccl::W`](W) writer structure"]
impl crate::Writable for ViIcclSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VI_ICCL to value 0x1f7b"]
impl crate::Resettable for ViIcclSpec {
    const RESET_VALUE: u32 = 0x1f7b;
}
