#[doc = "Register `HWC_LUT_ADDR` reader"]
pub type R = crate::R<HwcLutAddrSpec>;
#[doc = "Register `HWC_LUT_ADDR` writer"]
pub type W = crate::W<HwcLutAddrSpec>;
#[doc = "Field `HWC_LUT_ADDR` reader - the head of hwc lut address"]
pub type HwcLutAddrR = crate::FieldReader<u32>;
#[doc = "Field `HWC_LUT_ADDR` writer - the head of hwc lut address"]
pub type HwcLutAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the head of hwc lut address"]
    #[inline(always)]
    pub fn hwc_lut_addr(&self) -> HwcLutAddrR {
        HwcLutAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the head of hwc lut address"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_lut_addr(&mut self) -> HwcLutAddrW<HwcLutAddrSpec> {
        HwcLutAddrW::new(self, 0)
    }
}
#[doc = "Hwc lut base address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwc_lut_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwc_lut_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwcLutAddrSpec;
impl crate::RegisterSpec for HwcLutAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwc_lut_addr::R`](R) reader structure"]
impl crate::Readable for HwcLutAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`hwc_lut_addr::W`](W) writer structure"]
impl crate::Writable for HwcLutAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWC_LUT_ADDR to value 0"]
impl crate::Resettable for HwcLutAddrSpec {
    const RESET_VALUE: u32 = 0;
}
