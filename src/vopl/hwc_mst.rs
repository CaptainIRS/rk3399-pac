#[doc = "Register `HWC_MST` reader"]
pub type R = crate::R<HwcMstSpec>;
#[doc = "Register `HWC_MST` writer"]
pub type W = crate::W<HwcMstSpec>;
#[doc = "Field `HWC_MST` reader - HWC data memory start address"]
pub type HwcMstR = crate::FieldReader<u32>;
#[doc = "Field `HWC_MST` writer - HWC data memory start address"]
pub type HwcMstW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - HWC data memory start address"]
    #[inline(always)]
    pub fn hwc_mst(&self) -> HwcMstR {
        HwcMstR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - HWC data memory start address"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_mst(&mut self) -> HwcMstW<HwcMstSpec> {
        HwcMstW::new(self, 0)
    }
}
#[doc = "Hwc memory start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwc_mst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwc_mst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwcMstSpec;
impl crate::RegisterSpec for HwcMstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwc_mst::R`](R) reader structure"]
impl crate::Readable for HwcMstSpec {}
#[doc = "`write(|w| ..)` method takes [`hwc_mst::W`](W) writer structure"]
impl crate::Writable for HwcMstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWC_MST to value 0"]
impl crate::Resettable for HwcMstSpec {
    const RESET_VALUE: u32 = 0;
}
