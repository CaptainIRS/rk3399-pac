#[doc = "Register `FILT_THRESH_SH0` reader"]
pub type R = crate::R<FiltThreshSh0Spec>;
#[doc = "Register `FILT_THRESH_SH0` writer"]
pub type W = crate::W<FiltThreshSh0Spec>;
#[doc = "Field `filt_thresh_sh0` reader - If filt_thresh_sh0 &lt; sum_grad &lt; filt_thresh_sh1 then filt_thresh_sh0 is selected"]
pub type FiltThreshSh0R = crate::FieldReader<u16>;
#[doc = "Field `filt_thresh_sh0` writer - If filt_thresh_sh0 &lt; sum_grad &lt; filt_thresh_sh1 then filt_thresh_sh0 is selected"]
pub type FiltThreshSh0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - If filt_thresh_sh0 &lt; sum_grad &lt; filt_thresh_sh1 then filt_thresh_sh0 is selected"]
    #[inline(always)]
    pub fn filt_thresh_sh0(&self) -> FiltThreshSh0R {
        FiltThreshSh0R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - If filt_thresh_sh0 &lt; sum_grad &lt; filt_thresh_sh1 then filt_thresh_sh0 is selected"]
    #[inline(always)]
    #[must_use]
    pub fn filt_thresh_sh0(&mut self) -> FiltThreshSh0W<FiltThreshSh0Spec> {
        FiltThreshSh0W::new(self, 0)
    }
}
#[doc = "Sharpening threshold 0\n\nNote: sum_grad is calculated by the texture detection unit as the sum of horizontal and \n\nvertical gradients \n\n\n\n \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filt_thresh_sh0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filt_thresh_sh0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FiltThreshSh0Spec;
impl crate::RegisterSpec for FiltThreshSh0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filt_thresh_sh0::R`](R) reader structure"]
impl crate::Readable for FiltThreshSh0Spec {}
#[doc = "`write(|w| ..)` method takes [`filt_thresh_sh0::W`](W) writer structure"]
impl crate::Writable for FiltThreshSh0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FILT_THRESH_SH0 to value 0x1a"]
impl crate::Resettable for FiltThreshSh0Spec {
    const RESET_VALUE: u32 = 0x1a;
}
