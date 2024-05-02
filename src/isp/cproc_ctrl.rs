#[doc = "Register `CPROC_CTRL` reader"]
pub type R = crate::R<CprocCtrlSpec>;
#[doc = "Register `CPROC_CTRL` writer"]
pub type W = crate::W<CprocCtrlSpec>;
#[doc = "Field `cproc_enable` reader - color processing enable\n\n0: color processing is bypassed\n\n2 * 10 Bit input data are truncated to 2 * 8Bit output\n\ndata 1: color processing is active\n\noutput data are rounded to 2 * 8Bit and clipping is\n\nactive\n\n"]
pub type CprocEnableR = crate::BitReader;
#[doc = "Field `cproc_enable` writer - color processing enable\n\n0: color processing is bypassed\n\n2 * 10 Bit input data are truncated to 2 * 8Bit output\n\ndata 1: color processing is active\n\noutput data are rounded to 2 * 8Bit and clipping is\n\nactive\n\n"]
pub type CprocEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Color processing luminance output clipping range\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CprocYOutRange {
    #[doc = "0: Y_out clipping range 16..235; offset of 16 is added to Y_out according to ITU-R BT.601 standard"]
    B0 = 0,
    #[doc = "1: Y_out clipping range 0..255; no offset is added to Y_out"]
    B1 = 1,
}
impl From<CprocYOutRange> for bool {
    #[inline(always)]
    fn from(variant: CprocYOutRange) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `cproc_y_out_range` reader - Color processing luminance output clipping range"]
pub type CprocYOutRangeR = crate::BitReader<CprocYOutRange>;
impl CprocYOutRangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CprocYOutRange {
        match self.bits {
            false => CprocYOutRange::B0,
            true => CprocYOutRange::B1,
        }
    }
    #[doc = "Y_out clipping range 16..235; offset of 16 is added to Y_out according to ITU-R BT.601 standard"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CprocYOutRange::B0
    }
    #[doc = "Y_out clipping range 0..255; no offset is added to Y_out"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CprocYOutRange::B1
    }
}
#[doc = "Field `cproc_y_out_range` writer - Color processing luminance output clipping range"]
pub type CprocYOutRangeW<'a, REG> = crate::BitWriter<'a, REG, CprocYOutRange>;
impl<'a, REG> CprocYOutRangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Y_out clipping range 16..235; offset of 16 is added to Y_out according to ITU-R BT.601 standard"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CprocYOutRange::B0)
    }
    #[doc = "Y_out clipping range 0..255; no offset is added to Y_out"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CprocYOutRange::B1)
    }
}
#[doc = "Field `cproc_y_in_range` reader - Color processing luminance input range (offset\n\nprocessing) 0: Y_in range 64..940 according to ITU-R\n\nBT.601 standard; offset of 64 will be subtracted from Y_in\n\n1: Y_in full range 0..1023; no offset will be subtracted\n\nfrom Y_in\n\n"]
pub type CprocYInRangeR = crate::BitReader;
#[doc = "Field `cproc_y_in_range` writer - Color processing luminance input range (offset\n\nprocessing) 0: Y_in range 64..940 according to ITU-R\n\nBT.601 standard; offset of 64 will be subtracted from Y_in\n\n1: Y_in full range 0..1023; no offset will be subtracted\n\nfrom Y_in\n\n"]
pub type CprocYInRangeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Color processing chrominance pixel clipping range at\n\noutput\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CprocCOutRange {
    #[doc = "0: CbCr_out clipping range 16..240 according to ITU-R BT.601 standard"]
    B0 = 0,
    #[doc = "1: full UV_out clipping range 0..255"]
    B1 = 1,
}
impl From<CprocCOutRange> for bool {
    #[inline(always)]
    fn from(variant: CprocCOutRange) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `cproc_c_out_range` reader - Color processing chrominance pixel clipping range at\n\noutput"]
pub type CprocCOutRangeR = crate::BitReader<CprocCOutRange>;
impl CprocCOutRangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CprocCOutRange {
        match self.bits {
            false => CprocCOutRange::B0,
            true => CprocCOutRange::B1,
        }
    }
    #[doc = "CbCr_out clipping range 16..240 according to ITU-R BT.601 standard"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CprocCOutRange::B0
    }
    #[doc = "full UV_out clipping range 0..255"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CprocCOutRange::B1
    }
}
#[doc = "Field `cproc_c_out_range` writer - Color processing chrominance pixel clipping range at\n\noutput"]
pub type CprocCOutRangeW<'a, REG> = crate::BitWriter<'a, REG, CprocCOutRange>;
impl<'a, REG> CprocCOutRangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CbCr_out clipping range 16..240 according to ITU-R BT.601 standard"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CprocCOutRange::B0)
    }
    #[doc = "full UV_out clipping range 0..255"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CprocCOutRange::B1)
    }
}
impl R {
    #[doc = "Bit 0 - color processing enable\n\n0: color processing is bypassed\n\n2 * 10 Bit input data are truncated to 2 * 8Bit output\n\ndata 1: color processing is active\n\noutput data are rounded to 2 * 8Bit and clipping is\n\nactive\n\n"]
    #[inline(always)]
    pub fn cproc_enable(&self) -> CprocEnableR {
        CprocEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Color processing luminance output clipping range"]
    #[inline(always)]
    pub fn cproc_y_out_range(&self) -> CprocYOutRangeR {
        CprocYOutRangeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Color processing luminance input range (offset\n\nprocessing) 0: Y_in range 64..940 according to ITU-R\n\nBT.601 standard; offset of 64 will be subtracted from Y_in\n\n1: Y_in full range 0..1023; no offset will be subtracted\n\nfrom Y_in\n\n"]
    #[inline(always)]
    pub fn cproc_y_in_range(&self) -> CprocYInRangeR {
        CprocYInRangeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Color processing chrominance pixel clipping range at\n\noutput"]
    #[inline(always)]
    pub fn cproc_c_out_range(&self) -> CprocCOutRangeR {
        CprocCOutRangeR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - color processing enable\n\n0: color processing is bypassed\n\n2 * 10 Bit input data are truncated to 2 * 8Bit output\n\ndata 1: color processing is active\n\noutput data are rounded to 2 * 8Bit and clipping is\n\nactive\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn cproc_enable(&mut self) -> CprocEnableW<CprocCtrlSpec> {
        CprocEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Color processing luminance output clipping range"]
    #[inline(always)]
    #[must_use]
    pub fn cproc_y_out_range(&mut self) -> CprocYOutRangeW<CprocCtrlSpec> {
        CprocYOutRangeW::new(self, 1)
    }
    #[doc = "Bit 2 - Color processing luminance input range (offset\n\nprocessing) 0: Y_in range 64..940 according to ITU-R\n\nBT.601 standard; offset of 64 will be subtracted from Y_in\n\n1: Y_in full range 0..1023; no offset will be subtracted\n\nfrom Y_in\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn cproc_y_in_range(&mut self) -> CprocYInRangeW<CprocCtrlSpec> {
        CprocYInRangeW::new(self, 2)
    }
    #[doc = "Bit 3 - Color processing chrominance pixel clipping range at\n\noutput"]
    #[inline(always)]
    #[must_use]
    pub fn cproc_c_out_range(&mut self) -> CprocCOutRangeW<CprocCtrlSpec> {
        CprocCOutRangeW::new(self, 3)
    }
}
#[doc = "Global control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cproc_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cproc_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CprocCtrlSpec;
impl crate::RegisterSpec for CprocCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cproc_ctrl::R`](R) reader structure"]
impl crate::Readable for CprocCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cproc_ctrl::W`](W) writer structure"]
impl crate::Writable for CprocCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPROC_CTRL to value 0"]
impl crate::Resettable for CprocCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
