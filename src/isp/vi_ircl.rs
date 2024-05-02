#[doc = "Register `VI_IRCL` reader"]
pub type R = crate::R<ViIrclSpec>;
#[doc = "Register `VI_IRCL` writer"]
pub type W = crate::W<ViIrclSpec>;
#[doc = "isp software reset\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViIspSoftRst {
    #[doc = "0: processing mode"]
    B0 = 0,
    #[doc = "1: reset state"]
    B1 = 1,
}
impl From<ViIspSoftRst> for bool {
    #[inline(always)]
    fn from(variant: ViIspSoftRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `vi_isp_soft_rst` reader - isp software reset"]
pub type ViIspSoftRstR = crate::BitReader<ViIspSoftRst>;
impl ViIspSoftRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ViIspSoftRst {
        match self.bits {
            false => ViIspSoftRst::B0,
            true => ViIspSoftRst::B1,
        }
    }
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ViIspSoftRst::B0
    }
    #[doc = "reset state"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ViIspSoftRst::B1
    }
}
#[doc = "Field `vi_isp_soft_rst` writer - isp software reset"]
pub type ViIspSoftRstW<'a, REG> = crate::BitWriter<'a, REG, ViIspSoftRst>;
impl<'a, REG> ViIspSoftRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ViIspSoftRst::B0)
    }
    #[doc = "reset state"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ViIspSoftRst::B1)
    }
}
#[doc = "color processing software reset\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViCpSoftRst {
    #[doc = "0: processing mode"]
    B0 = 0,
    #[doc = "1: reset state"]
    B1 = 1,
}
impl From<ViCpSoftRst> for bool {
    #[inline(always)]
    fn from(variant: ViCpSoftRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `vi_cp_soft_rst` reader - color processing software reset"]
pub type ViCpSoftRstR = crate::BitReader<ViCpSoftRst>;
impl ViCpSoftRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ViCpSoftRst {
        match self.bits {
            false => ViCpSoftRst::B0,
            true => ViCpSoftRst::B1,
        }
    }
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ViCpSoftRst::B0
    }
    #[doc = "reset state"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ViCpSoftRst::B1
    }
}
#[doc = "Field `vi_cp_soft_rst` writer - color processing software reset"]
pub type ViCpSoftRstW<'a, REG> = crate::BitWriter<'a, REG, ViCpSoftRst>;
impl<'a, REG> ViCpSoftRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ViCpSoftRst::B0)
    }
    #[doc = "reset state"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ViCpSoftRst::B1)
    }
}
#[doc = "y/c splitter software reset\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViYcsSoftRst {
    #[doc = "0: processing mode"]
    B0 = 0,
    #[doc = "1: reset state"]
    B1 = 1,
}
impl From<ViYcsSoftRst> for bool {
    #[inline(always)]
    fn from(variant: ViYcsSoftRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `vi_ycs_soft_rst` reader - y/c splitter software reset"]
pub type ViYcsSoftRstR = crate::BitReader<ViYcsSoftRst>;
impl ViYcsSoftRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ViYcsSoftRst {
        match self.bits {
            false => ViYcsSoftRst::B0,
            true => ViYcsSoftRst::B1,
        }
    }
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ViYcsSoftRst::B0
    }
    #[doc = "reset state"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ViYcsSoftRst::B1
    }
}
#[doc = "Field `vi_ycs_soft_rst` writer - y/c splitter software reset"]
pub type ViYcsSoftRstW<'a, REG> = crate::BitWriter<'a, REG, ViYcsSoftRst>;
impl<'a, REG> ViYcsSoftRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ViYcsSoftRst::B0)
    }
    #[doc = "reset state"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ViYcsSoftRst::B1)
    }
}
#[doc = "Main-picture resize software reset\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViMrszSoftRst {
    #[doc = "0: processing mode"]
    B0 = 0,
    #[doc = "1: reset state"]
    B1 = 1,
}
impl From<ViMrszSoftRst> for bool {
    #[inline(always)]
    fn from(variant: ViMrszSoftRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `vi_mrsz_soft_rst` reader - Main-picture resize software reset"]
pub type ViMrszSoftRstR = crate::BitReader<ViMrszSoftRst>;
impl ViMrszSoftRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ViMrszSoftRst {
        match self.bits {
            false => ViMrszSoftRst::B0,
            true => ViMrszSoftRst::B1,
        }
    }
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ViMrszSoftRst::B0
    }
    #[doc = "reset state"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ViMrszSoftRst::B1
    }
}
#[doc = "Field `vi_mrsz_soft_rst` writer - Main-picture resize software reset"]
pub type ViMrszSoftRstW<'a, REG> = crate::BitWriter<'a, REG, ViMrszSoftRst>;
impl<'a, REG> ViMrszSoftRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ViMrszSoftRst::B0)
    }
    #[doc = "reset state"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ViMrszSoftRst::B1)
    }
}
#[doc = "Self-picture resize software reset\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViSrszSoftRst {
    #[doc = "0: processing mode"]
    B0 = 0,
    #[doc = "1: reset state"]
    B1 = 1,
}
impl From<ViSrszSoftRst> for bool {
    #[inline(always)]
    fn from(variant: ViSrszSoftRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `vi_srsz_soft_rst` reader - Self-picture resize software reset"]
pub type ViSrszSoftRstR = crate::BitReader<ViSrszSoftRst>;
impl ViSrszSoftRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ViSrszSoftRst {
        match self.bits {
            false => ViSrszSoftRst::B0,
            true => ViSrszSoftRst::B1,
        }
    }
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ViSrszSoftRst::B0
    }
    #[doc = "reset state"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ViSrszSoftRst::B1
    }
}
#[doc = "Field `vi_srsz_soft_rst` writer - Self-picture resize software reset"]
pub type ViSrszSoftRstW<'a, REG> = crate::BitWriter<'a, REG, ViSrszSoftRst>;
impl<'a, REG> ViSrszSoftRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ViSrszSoftRst::B0)
    }
    #[doc = "reset state"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ViSrszSoftRst::B1)
    }
}
#[doc = "JPEG encoder software reset\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViJpegSoftRst {
    #[doc = "0: processing mode"]
    B0 = 0,
    #[doc = "1: reset state"]
    B1 = 1,
}
impl From<ViJpegSoftRst> for bool {
    #[inline(always)]
    fn from(variant: ViJpegSoftRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `vi_jpeg_soft_rst` reader - JPEG encoder software reset"]
pub type ViJpegSoftRstR = crate::BitReader<ViJpegSoftRst>;
impl ViJpegSoftRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ViJpegSoftRst {
        match self.bits {
            false => ViJpegSoftRst::B0,
            true => ViJpegSoftRst::B1,
        }
    }
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ViJpegSoftRst::B0
    }
    #[doc = "reset state"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ViJpegSoftRst::B1
    }
}
#[doc = "Field `vi_jpeg_soft_rst` writer - JPEG encoder software reset"]
pub type ViJpegSoftRstW<'a, REG> = crate::BitWriter<'a, REG, ViJpegSoftRst>;
impl<'a, REG> ViJpegSoftRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ViJpegSoftRst::B0)
    }
    #[doc = "reset state"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ViJpegSoftRst::B1)
    }
}
#[doc = "memory interface software reset\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViMiSoftRst {
    #[doc = "0: processing mode"]
    B0 = 0,
    #[doc = "1: reset state"]
    B1 = 1,
}
impl From<ViMiSoftRst> for bool {
    #[inline(always)]
    fn from(variant: ViMiSoftRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `vi_mi_soft_rst` reader - memory interface software reset"]
pub type ViMiSoftRstR = crate::BitReader<ViMiSoftRst>;
impl ViMiSoftRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ViMiSoftRst {
        match self.bits {
            false => ViMiSoftRst::B0,
            true => ViMiSoftRst::B1,
        }
    }
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ViMiSoftRst::B0
    }
    #[doc = "reset state"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ViMiSoftRst::B1
    }
}
#[doc = "Field `vi_mi_soft_rst` writer - memory interface software reset"]
pub type ViMiSoftRstW<'a, REG> = crate::BitWriter<'a, REG, ViMiSoftRst>;
impl<'a, REG> ViMiSoftRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ViMiSoftRst::B0)
    }
    #[doc = "reset state"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ViMiSoftRst::B1)
    }
}
#[doc = "hardware reset of entire ISP\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViIspRst {
    #[doc = "0: processing mode"]
    B0 = 0,
    #[doc = "1: reset state"]
    B1 = 1,
}
impl From<ViIspRst> for bool {
    #[inline(always)]
    fn from(variant: ViIspRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `vi_ISP_rst` reader - hardware reset of entire ISP"]
pub type ViIspRstR = crate::BitReader<ViIspRst>;
impl ViIspRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ViIspRst {
        match self.bits {
            false => ViIspRst::B0,
            true => ViIspRst::B1,
        }
    }
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ViIspRst::B0
    }
    #[doc = "reset state"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ViIspRst::B1
    }
}
#[doc = "Field `vi_ISP_rst` writer - hardware reset of entire ISP"]
pub type ViIspRstW<'a, REG> = crate::BitWriter<'a, REG, ViIspRst>;
impl<'a, REG> ViIspRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ViIspRst::B0)
    }
    #[doc = "reset state"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ViIspRst::B1)
    }
}
#[doc = "Image effect software reset\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViIeSoftRst {
    #[doc = "0: processing mode"]
    B0 = 0,
    #[doc = "1: reset state"]
    B1 = 1,
}
impl From<ViIeSoftRst> for bool {
    #[inline(always)]
    fn from(variant: ViIeSoftRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `vi_ie_soft_rst` reader - Image effect software reset"]
pub type ViIeSoftRstR = crate::BitReader<ViIeSoftRst>;
impl ViIeSoftRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ViIeSoftRst {
        match self.bits {
            false => ViIeSoftRst::B0,
            true => ViIeSoftRst::B1,
        }
    }
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ViIeSoftRst::B0
    }
    #[doc = "reset state"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ViIeSoftRst::B1
    }
}
#[doc = "Field `vi_ie_soft_rst` writer - Image effect software reset"]
pub type ViIeSoftRstW<'a, REG> = crate::BitWriter<'a, REG, ViIeSoftRst>;
impl<'a, REG> ViIeSoftRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ViIeSoftRst::B0)
    }
    #[doc = "reset state"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ViIeSoftRst::B1)
    }
}
#[doc = "Superimpose software reset\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViSimpSoftRst {
    #[doc = "0: processing mode"]
    B0 = 0,
    #[doc = "1: reset state"]
    B1 = 1,
}
impl From<ViSimpSoftRst> for bool {
    #[inline(always)]
    fn from(variant: ViSimpSoftRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `vi_simp_soft_rst` reader - Superimpose software reset"]
pub type ViSimpSoftRstR = crate::BitReader<ViSimpSoftRst>;
impl ViSimpSoftRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ViSimpSoftRst {
        match self.bits {
            false => ViSimpSoftRst::B0,
            true => ViSimpSoftRst::B1,
        }
    }
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ViSimpSoftRst::B0
    }
    #[doc = "reset state"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ViSimpSoftRst::B1
    }
}
#[doc = "Field `vi_simp_soft_rst` writer - Superimpose software reset"]
pub type ViSimpSoftRstW<'a, REG> = crate::BitWriter<'a, REG, ViSimpSoftRst>;
impl<'a, REG> ViSimpSoftRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ViSimpSoftRst::B0)
    }
    #[doc = "reset state"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ViSimpSoftRst::B1)
    }
}
#[doc = "SMIA Interface software reset\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViSmiaSoftRst {
    #[doc = "0: processing mode"]
    B0 = 0,
    #[doc = "1: reset state"]
    B1 = 1,
}
impl From<ViSmiaSoftRst> for bool {
    #[inline(always)]
    fn from(variant: ViSmiaSoftRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `vi_smia_soft_rst` reader - SMIA Interface software reset"]
pub type ViSmiaSoftRstR = crate::BitReader<ViSmiaSoftRst>;
impl ViSmiaSoftRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ViSmiaSoftRst {
        match self.bits {
            false => ViSmiaSoftRst::B0,
            true => ViSmiaSoftRst::B1,
        }
    }
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ViSmiaSoftRst::B0
    }
    #[doc = "reset state"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ViSmiaSoftRst::B1
    }
}
#[doc = "Field `vi_smia_soft_rst` writer - SMIA Interface software reset"]
pub type ViSmiaSoftRstW<'a, REG> = crate::BitWriter<'a, REG, ViSmiaSoftRst>;
impl<'a, REG> ViSmiaSoftRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ViSmiaSoftRst::B0)
    }
    #[doc = "reset state"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ViSmiaSoftRst::B1)
    }
}
#[doc = "MIPI Interface software\n\nreset\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViMipiSoftRst {
    #[doc = "0: processing mode"]
    B0 = 0,
    #[doc = "1: reset state"]
    B1 = 1,
}
impl From<ViMipiSoftRst> for bool {
    #[inline(always)]
    fn from(variant: ViMipiSoftRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `vi_mipi_soft_rst` reader - MIPI Interface software\n\nreset"]
pub type ViMipiSoftRstR = crate::BitReader<ViMipiSoftRst>;
impl ViMipiSoftRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ViMipiSoftRst {
        match self.bits {
            false => ViMipiSoftRst::B0,
            true => ViMipiSoftRst::B1,
        }
    }
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ViMipiSoftRst::B0
    }
    #[doc = "reset state"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ViMipiSoftRst::B1
    }
}
#[doc = "Field `vi_mipi_soft_rst` writer - MIPI Interface software\n\nreset"]
pub type ViMipiSoftRstW<'a, REG> = crate::BitWriter<'a, REG, ViMipiSoftRst>;
impl<'a, REG> ViMipiSoftRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "processing mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ViMipiSoftRst::B0)
    }
    #[doc = "reset state"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ViMipiSoftRst::B1)
    }
}
impl R {
    #[doc = "Bit 0 - isp software reset"]
    #[inline(always)]
    pub fn vi_isp_soft_rst(&self) -> ViIspSoftRstR {
        ViIspSoftRstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - color processing software reset"]
    #[inline(always)]
    pub fn vi_cp_soft_rst(&self) -> ViCpSoftRstR {
        ViCpSoftRstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - y/c splitter software reset"]
    #[inline(always)]
    pub fn vi_ycs_soft_rst(&self) -> ViYcsSoftRstR {
        ViYcsSoftRstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Main-picture resize software reset"]
    #[inline(always)]
    pub fn vi_mrsz_soft_rst(&self) -> ViMrszSoftRstR {
        ViMrszSoftRstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Self-picture resize software reset"]
    #[inline(always)]
    pub fn vi_srsz_soft_rst(&self) -> ViSrszSoftRstR {
        ViSrszSoftRstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - JPEG encoder software reset"]
    #[inline(always)]
    pub fn vi_jpeg_soft_rst(&self) -> ViJpegSoftRstR {
        ViJpegSoftRstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - memory interface software reset"]
    #[inline(always)]
    pub fn vi_mi_soft_rst(&self) -> ViMiSoftRstR {
        ViMiSoftRstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - hardware reset of entire ISP"]
    #[inline(always)]
    pub fn vi_isp_rst(&self) -> ViIspRstR {
        ViIspRstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Image effect software reset"]
    #[inline(always)]
    pub fn vi_ie_soft_rst(&self) -> ViIeSoftRstR {
        ViIeSoftRstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Superimpose software reset"]
    #[inline(always)]
    pub fn vi_simp_soft_rst(&self) -> ViSimpSoftRstR {
        ViSimpSoftRstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SMIA Interface software reset"]
    #[inline(always)]
    pub fn vi_smia_soft_rst(&self) -> ViSmiaSoftRstR {
        ViSmiaSoftRstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MIPI Interface software\n\nreset"]
    #[inline(always)]
    pub fn vi_mipi_soft_rst(&self) -> ViMipiSoftRstR {
        ViMipiSoftRstR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - isp software reset"]
    #[inline(always)]
    #[must_use]
    pub fn vi_isp_soft_rst(&mut self) -> ViIspSoftRstW<ViIrclSpec> {
        ViIspSoftRstW::new(self, 0)
    }
    #[doc = "Bit 1 - color processing software reset"]
    #[inline(always)]
    #[must_use]
    pub fn vi_cp_soft_rst(&mut self) -> ViCpSoftRstW<ViIrclSpec> {
        ViCpSoftRstW::new(self, 1)
    }
    #[doc = "Bit 2 - y/c splitter software reset"]
    #[inline(always)]
    #[must_use]
    pub fn vi_ycs_soft_rst(&mut self) -> ViYcsSoftRstW<ViIrclSpec> {
        ViYcsSoftRstW::new(self, 2)
    }
    #[doc = "Bit 3 - Main-picture resize software reset"]
    #[inline(always)]
    #[must_use]
    pub fn vi_mrsz_soft_rst(&mut self) -> ViMrszSoftRstW<ViIrclSpec> {
        ViMrszSoftRstW::new(self, 3)
    }
    #[doc = "Bit 4 - Self-picture resize software reset"]
    #[inline(always)]
    #[must_use]
    pub fn vi_srsz_soft_rst(&mut self) -> ViSrszSoftRstW<ViIrclSpec> {
        ViSrszSoftRstW::new(self, 4)
    }
    #[doc = "Bit 5 - JPEG encoder software reset"]
    #[inline(always)]
    #[must_use]
    pub fn vi_jpeg_soft_rst(&mut self) -> ViJpegSoftRstW<ViIrclSpec> {
        ViJpegSoftRstW::new(self, 5)
    }
    #[doc = "Bit 6 - memory interface software reset"]
    #[inline(always)]
    #[must_use]
    pub fn vi_mi_soft_rst(&mut self) -> ViMiSoftRstW<ViIrclSpec> {
        ViMiSoftRstW::new(self, 6)
    }
    #[doc = "Bit 7 - hardware reset of entire ISP"]
    #[inline(always)]
    #[must_use]
    pub fn vi_isp_rst(&mut self) -> ViIspRstW<ViIrclSpec> {
        ViIspRstW::new(self, 7)
    }
    #[doc = "Bit 8 - Image effect software reset"]
    #[inline(always)]
    #[must_use]
    pub fn vi_ie_soft_rst(&mut self) -> ViIeSoftRstW<ViIrclSpec> {
        ViIeSoftRstW::new(self, 8)
    }
    #[doc = "Bit 9 - Superimpose software reset"]
    #[inline(always)]
    #[must_use]
    pub fn vi_simp_soft_rst(&mut self) -> ViSimpSoftRstW<ViIrclSpec> {
        ViSimpSoftRstW::new(self, 9)
    }
    #[doc = "Bit 10 - SMIA Interface software reset"]
    #[inline(always)]
    #[must_use]
    pub fn vi_smia_soft_rst(&mut self) -> ViSmiaSoftRstW<ViIrclSpec> {
        ViSmiaSoftRstW::new(self, 10)
    }
    #[doc = "Bit 11 - MIPI Interface software\n\nreset"]
    #[inline(always)]
    #[must_use]
    pub fn vi_mipi_soft_rst(&mut self) -> ViMipiSoftRstW<ViIrclSpec> {
        ViMipiSoftRstW::new(self, 11)
    }
}
#[doc = "Internal reset control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vi_ircl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vi_ircl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ViIrclSpec;
impl crate::RegisterSpec for ViIrclSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vi_ircl::R`](R) reader structure"]
impl crate::Readable for ViIrclSpec {}
#[doc = "`write(|w| ..)` method takes [`vi_ircl::W`](W) writer structure"]
impl crate::Writable for ViIrclSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VI_IRCL to value 0"]
impl crate::Resettable for ViIrclSpec {
    const RESET_VALUE: u32 = 0;
}
