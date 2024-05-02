#[doc = "Register `FILT_THRESH_BL0` reader"]
pub type R = crate::R<FiltThreshBl0Spec>;
#[doc = "Register `FILT_THRESH_BL0` writer"]
pub type W = crate::W<FiltThreshBl0Spec>;
#[doc = "Field `filt_thresh_bl0` reader - If filt_thresh_bl1 &lt; sum_grad &lt; filt_thresh_bl0 then filt_fac_bl0 is selected"]
pub type FiltThreshBl0R = crate::FieldReader<u16>;
#[doc = "Field `filt_thresh_bl0` writer - If filt_thresh_bl1 &lt; sum_grad &lt; filt_thresh_bl0 then filt_fac_bl0 is selected"]
pub type FiltThreshBl0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - If filt_thresh_bl1 &lt; sum_grad &lt; filt_thresh_bl0 then filt_fac_bl0 is selected"]
    #[inline(always)]
    pub fn filt_thresh_bl0(&self) -> FiltThreshBl0R {
        FiltThreshBl0R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - If filt_thresh_bl1 &lt; sum_grad &lt; filt_thresh_bl0 then filt_fac_bl0 is selected"]
    #[inline(always)]
    #[must_use]
    pub fn filt_thresh_bl0(&mut self) -> FiltThreshBl0W<FiltThreshBl0Spec> {
        FiltThreshBl0W::new(self, 0)
    }
}
#[doc = "Blurring threshold 0\n\nNote: sum_grad is calculated by the texture detection unit as the sum of \n\n\n\nhorizontal and vertical gradients \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filt_thresh_bl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filt_thresh_bl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FiltThreshBl0Spec;
impl crate::RegisterSpec for FiltThreshBl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filt_thresh_bl0::R`](R) reader structure"]
impl crate::Readable for FiltThreshBl0Spec {}
#[doc = "`write(|w| ..)` method takes [`filt_thresh_bl0::W`](W) writer structure"]
impl crate::Writable for FiltThreshBl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FILT_THRESH_BL0 to value 0x0d"]
impl crate::Resettable for FiltThreshBl0Spec {
    const RESET_VALUE: u32 = 0x0d;
}
