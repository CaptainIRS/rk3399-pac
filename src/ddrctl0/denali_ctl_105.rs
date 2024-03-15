#[doc = "Register `DENALI_CTL_105` reader"]
pub type R = crate::R<DenaliCtl105Spec>;
#[doc = "Register `DENALI_CTL_105` writer"]
pub type W = crate::W<DenaliCtl105Spec>;
#[doc = "Field `LPC_PROMOTE_THRESHOLD_F0` reader - LPC promotion number of long counts until the high priority request is asserted for frequency copy 0. Applies to SW and auto low power commands."]
pub type LpcPromoteThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `LPC_PROMOTE_THRESHOLD_F0` writer - LPC promotion number of long counts until the high priority request is asserted for frequency copy 0. Applies to SW and auto low power commands."]
pub type LpcPromoteThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LPC_PROMOTE_THRESHOLD_F1` reader - LPC promotion number of long counts until the high priority request is asserted for frequency copy 1. Applies to SW and auto low power commands."]
pub type LpcPromoteThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `LPC_PROMOTE_THRESHOLD_F1` writer - LPC promotion number of long counts until the high priority request is asserted for frequency copy 1. Applies to SW and auto low power commands."]
pub type LpcPromoteThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - LPC promotion number of long counts until the high priority request is asserted for frequency copy 0. Applies to SW and auto low power commands."]
    #[inline(always)]
    pub fn lpc_promote_threshold_f0(&self) -> LpcPromoteThresholdF0R {
        LpcPromoteThresholdF0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - LPC promotion number of long counts until the high priority request is asserted for frequency copy 1. Applies to SW and auto low power commands."]
    #[inline(always)]
    pub fn lpc_promote_threshold_f1(&self) -> LpcPromoteThresholdF1R {
        LpcPromoteThresholdF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - LPC promotion number of long counts until the high priority request is asserted for frequency copy 0. Applies to SW and auto low power commands."]
    #[inline(always)]
    #[must_use]
    pub fn lpc_promote_threshold_f0(&mut self) -> LpcPromoteThresholdF0W<DenaliCtl105Spec> {
        LpcPromoteThresholdF0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - LPC promotion number of long counts until the high priority request is asserted for frequency copy 1. Applies to SW and auto low power commands."]
    #[inline(always)]
    #[must_use]
    pub fn lpc_promote_threshold_f1(&mut self) -> LpcPromoteThresholdF1W<DenaliCtl105Spec> {
        LpcPromoteThresholdF1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_105::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_105::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl105Spec;
impl crate::RegisterSpec for DenaliCtl105Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_105::R`](R) reader structure"]
impl crate::Readable for DenaliCtl105Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_105::W`](W) writer structure"]
impl crate::Writable for DenaliCtl105Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_105 to value 0"]
impl crate::Resettable for DenaliCtl105Spec {
    const RESET_VALUE: u32 = 0;
}
