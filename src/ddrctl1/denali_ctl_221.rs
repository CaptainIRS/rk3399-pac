#[doc = "Register `DENALI_CTL_221` reader"]
pub type R = crate::R<DenaliCtl221Spec>;
#[doc = "Register `DENALI_CTL_221` writer"]
pub type W = crate::W<DenaliCtl221Spec>;
#[doc = "Field `W2W_SAMECS_DLY` reader - Additional delay to insert between two writes to the same chip select. Any value including 0x0 supported."]
pub type W2wSamecsDlyR = crate::FieldReader;
#[doc = "Field `W2W_SAMECS_DLY` writer - Additional delay to insert between two writes to the same chip select. Any value including 0x0 supported."]
pub type W2wSamecsDlyW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TDQSCK_MAX_F0` reader - Additional delay needed for tDQSCK."]
pub type TdqsckMaxF0R = crate::FieldReader;
#[doc = "Field `TDQSCK_MAX_F0` writer - Additional delay needed for tDQSCK."]
pub type TdqsckMaxF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TDQSCK_MIN_F0` reader - Additional delay needed for tDQSCK."]
pub type TdqsckMinF0R = crate::FieldReader;
#[doc = "Field `TDQSCK_MIN_F0` writer - Additional delay needed for tDQSCK."]
pub type TdqsckMinF0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TDQSCK_MAX_F1` reader - Additional delay needed for tDQSCK."]
pub type TdqsckMaxF1R = crate::FieldReader;
#[doc = "Field `TDQSCK_MAX_F1` writer - Additional delay needed for tDQSCK."]
pub type TdqsckMaxF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - Additional delay to insert between two writes to the same chip select. Any value including 0x0 supported."]
    #[inline(always)]
    pub fn w2w_samecs_dly(&self) -> W2wSamecsDlyR {
        W2wSamecsDlyR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Additional delay needed for tDQSCK."]
    #[inline(always)]
    pub fn tdqsck_max_f0(&self) -> TdqsckMaxF0R {
        TdqsckMaxF0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Additional delay needed for tDQSCK."]
    #[inline(always)]
    pub fn tdqsck_min_f0(&self) -> TdqsckMinF0R {
        TdqsckMinF0R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Additional delay needed for tDQSCK."]
    #[inline(always)]
    pub fn tdqsck_max_f1(&self) -> TdqsckMaxF1R {
        TdqsckMaxF1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Additional delay to insert between two writes to the same chip select. Any value including 0x0 supported."]
    #[inline(always)]
    #[must_use]
    pub fn w2w_samecs_dly(&mut self) -> W2wSamecsDlyW<DenaliCtl221Spec> {
        W2wSamecsDlyW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Additional delay needed for tDQSCK."]
    #[inline(always)]
    #[must_use]
    pub fn tdqsck_max_f0(&mut self) -> TdqsckMaxF0W<DenaliCtl221Spec> {
        TdqsckMaxF0W::new(self, 8)
    }
    #[doc = "Bits 16:17 - Additional delay needed for tDQSCK."]
    #[inline(always)]
    #[must_use]
    pub fn tdqsck_min_f0(&mut self) -> TdqsckMinF0W<DenaliCtl221Spec> {
        TdqsckMinF0W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Additional delay needed for tDQSCK."]
    #[inline(always)]
    #[must_use]
    pub fn tdqsck_max_f1(&mut self) -> TdqsckMaxF1W<DenaliCtl221Spec> {
        TdqsckMaxF1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_221::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_221::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl221Spec;
impl crate::RegisterSpec for DenaliCtl221Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_221::R`](R) reader structure"]
impl crate::Readable for DenaliCtl221Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_221::W`](W) writer structure"]
impl crate::Writable for DenaliCtl221Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_221 to value 0"]
impl crate::Resettable for DenaliCtl221Spec {
    const RESET_VALUE: u32 = 0;
}
