#[doc = "Register `FILT_THRESH_BL1` reader"]
pub type R = crate::R<FiltThreshBl1Spec>;
#[doc = "Register `FILT_THRESH_BL1` writer"]
pub type W = crate::W<FiltThreshBl1Spec>;
#[doc = "Field `filt_thresh_bl1` reader - If sum_grad &lt; filt_thresh_bl1 then filt_fac_bl1 is\n\nselected"]
pub type FiltThreshBl1R = crate::FieldReader<u16>;
#[doc = "Field `filt_thresh_bl1` writer - If sum_grad &lt; filt_thresh_bl1 then filt_fac_bl1 is\n\nselected"]
pub type FiltThreshBl1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - If sum_grad &lt; filt_thresh_bl1 then filt_fac_bl1 is\n\nselected"]
    #[inline(always)]
    pub fn filt_thresh_bl1(&self) -> FiltThreshBl1R {
        FiltThreshBl1R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - If sum_grad &lt; filt_thresh_bl1 then filt_fac_bl1 is\n\nselected"]
    #[inline(always)]
    #[must_use]
    pub fn filt_thresh_bl1(&mut self) -> FiltThreshBl1W<FiltThreshBl1Spec> {
        FiltThreshBl1W::new(self, 0)
    }
}
#[doc = "Blurring threshold 1\n\nNote: sum_grad is calculated by the texture detection unit as the sum of horizontal and \n\nvertical gradients \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filt_thresh_bl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filt_thresh_bl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FiltThreshBl1Spec;
impl crate::RegisterSpec for FiltThreshBl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filt_thresh_bl1::R`](R) reader structure"]
impl crate::Readable for FiltThreshBl1Spec {}
#[doc = "`write(|w| ..)` method takes [`filt_thresh_bl1::W`](W) writer structure"]
impl crate::Writable for FiltThreshBl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FILT_THRESH_BL1 to value 0x05"]
impl crate::Resettable for FiltThreshBl1Spec {
    const RESET_VALUE: u32 = 0x05;
}
