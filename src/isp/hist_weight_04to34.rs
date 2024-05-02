#[doc = "Register `HIST_WEIGHT_04TO34` reader"]
pub type R = crate::R<HistWeight04to34Spec>;
#[doc = "Register `HIST_WEIGHT_04TO34` writer"]
pub type W = crate::W<HistWeight04to34Spec>;
#[doc = "Field `hist_weight_04` reader - weighting factor for sub-window 04"]
pub type HistWeight04R = crate::FieldReader;
#[doc = "Field `hist_weight_04` writer - weighting factor for sub-window 04"]
pub type HistWeight04W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `hist_weight_14` reader - weighting factor for sub-window 14\n\n"]
pub type HistWeight14R = crate::FieldReader;
#[doc = "Field `hist_weight_14` writer - weighting factor for sub-window 14\n\n"]
pub type HistWeight14W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `hist_weight_24` reader - weighting factor for sub-window 24"]
pub type HistWeight24R = crate::FieldReader;
#[doc = "Field `hist_weight_24` writer - weighting factor for sub-window 24"]
pub type HistWeight24W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `hist_weight_34` reader - weighting factor for sub-window 34"]
pub type HistWeight34R = crate::FieldReader;
#[doc = "Field `hist_weight_34` writer - weighting factor for sub-window 34"]
pub type HistWeight34W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - weighting factor for sub-window 04"]
    #[inline(always)]
    pub fn hist_weight_04(&self) -> HistWeight04R {
        HistWeight04R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - weighting factor for sub-window 14\n\n"]
    #[inline(always)]
    pub fn hist_weight_14(&self) -> HistWeight14R {
        HistWeight14R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - weighting factor for sub-window 24"]
    #[inline(always)]
    pub fn hist_weight_24(&self) -> HistWeight24R {
        HistWeight24R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - weighting factor for sub-window 34"]
    #[inline(always)]
    pub fn hist_weight_34(&self) -> HistWeight34R {
        HistWeight34R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - weighting factor for sub-window 04"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_04(&mut self) -> HistWeight04W<HistWeight04to34Spec> {
        HistWeight04W::new(self, 0)
    }
    #[doc = "Bits 8:12 - weighting factor for sub-window 14\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_14(&mut self) -> HistWeight14W<HistWeight04to34Spec> {
        HistWeight14W::new(self, 8)
    }
    #[doc = "Bits 16:20 - weighting factor for sub-window 24"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_24(&mut self) -> HistWeight24W<HistWeight04to34Spec> {
        HistWeight24W::new(self, 16)
    }
    #[doc = "Bits 24:28 - weighting factor for sub-window 34"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_34(&mut self) -> HistWeight34W<HistWeight04to34Spec> {
        HistWeight34W::new(self, 24)
    }
}
#[doc = "Weighting factor for sub-windows\n\nNote: Allowed value range for weight factor is 0 to 16. The resulting weight is \n\nregister_value / 16. The host software has to limit the register value for each factor to 16. \n\nWeight registers are available, if histogram version 3 is implemented. \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_weight_04to34::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_weight_04to34::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HistWeight04to34Spec;
impl crate::RegisterSpec for HistWeight04to34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_weight_04to34::R`](R) reader structure"]
impl crate::Readable for HistWeight04to34Spec {}
#[doc = "`write(|w| ..)` method takes [`hist_weight_04to34::W`](W) writer structure"]
impl crate::Writable for HistWeight04to34Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HIST_WEIGHT_04TO34 to value 0x1010_1010"]
impl crate::Resettable for HistWeight04to34Spec {
    const RESET_VALUE: u32 = 0x1010_1010;
}
