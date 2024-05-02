#[doc = "Register `HIST_WEIGHT_44` reader"]
pub type R = crate::R<HistWeight44Spec>;
#[doc = "Register `HIST_WEIGHT_44` writer"]
pub type W = crate::W<HistWeight44Spec>;
#[doc = "Field `hist_weight_44` reader - weighting factor for sub-window 44"]
pub type HistWeight44R = crate::FieldReader;
#[doc = "Field `hist_weight_44` writer - weighting factor for sub-window 44"]
pub type HistWeight44W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - weighting factor for sub-window 44"]
    #[inline(always)]
    pub fn hist_weight_44(&self) -> HistWeight44R {
        HistWeight44R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - weighting factor for sub-window 44"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_44(&mut self) -> HistWeight44W<HistWeight44Spec> {
        HistWeight44W::new(self, 0)
    }
}
#[doc = "Weighting factor for sub-windows\n\nNote: Allowed value range for weight factor is 0 to 16. The resulting weight is \n\nregister_value / 16. The host software has to limit the register value for each factor to 16. \n\nWeight registers are available, if histogram version 3 is implemented. \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_weight_44::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_weight_44::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HistWeight44Spec;
impl crate::RegisterSpec for HistWeight44Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_weight_44::R`](R) reader structure"]
impl crate::Readable for HistWeight44Spec {}
#[doc = "`write(|w| ..)` method takes [`hist_weight_44::W`](W) writer structure"]
impl crate::Writable for HistWeight44Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HIST_WEIGHT_44 to value 0x10"]
impl crate::Resettable for HistWeight44Spec {
    const RESET_VALUE: u32 = 0x10;
}
