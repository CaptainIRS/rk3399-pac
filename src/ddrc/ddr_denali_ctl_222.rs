#[doc = "Register `DDR_DENALI_CTL_222` reader"]
pub type R = crate::R<DdrDenaliCtl222Spec>;
#[doc = "Register `DDR_DENALI_CTL_222` writer"]
pub type W = crate::W<DdrDenaliCtl222Spec>;
#[doc = "Field `TDQSCK_MIN_F1` reader - Additional delay needed for tDQSCK."]
pub type TdqsckMinF1R = crate::FieldReader;
#[doc = "Field `TDQSCK_MIN_F1` writer - Additional delay needed for tDQSCK."]
pub type TdqsckMinF1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TDQSCK_MAX_F2` reader - Additional delay needed for tDQSCK."]
pub type TdqsckMaxF2R = crate::FieldReader;
#[doc = "Field `TDQSCK_MAX_F2` writer - Additional delay needed for tDQSCK."]
pub type TdqsckMaxF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TDQSCK_MIN_F2` reader - Additional delay needed for tDQSCK."]
pub type TdqsckMinF2R = crate::FieldReader;
#[doc = "Field `TDQSCK_MIN_F2` writer - Additional delay needed for tDQSCK."]
pub type TdqsckMinF2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SW_LEVELING_MODE` reader - Defines the leveling operation for software leveling. Clear to 0 for none, program to 1 for write leveling, program to 2 for data eye training, or program to 3 for gate training."]
pub type SwLevelingModeR = crate::FieldReader;
#[doc = "Field `SW_LEVELING_MODE` writer - Defines the leveling operation for software leveling. Clear to 0 for none, program to 1 for write leveling, program to 2 for data eye training, or program to 3 for gate training."]
pub type SwLevelingModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - Additional delay needed for tDQSCK."]
    #[inline(always)]
    pub fn tdqsck_min_f1(&self) -> TdqsckMinF1R {
        TdqsckMinF1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - Additional delay needed for tDQSCK."]
    #[inline(always)]
    pub fn tdqsck_max_f2(&self) -> TdqsckMaxF2R {
        TdqsckMaxF2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Additional delay needed for tDQSCK."]
    #[inline(always)]
    pub fn tdqsck_min_f2(&self) -> TdqsckMinF2R {
        TdqsckMinF2R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Defines the leveling operation for software leveling. Clear to 0 for none, program to 1 for write leveling, program to 2 for data eye training, or program to 3 for gate training."]
    #[inline(always)]
    pub fn sw_leveling_mode(&self) -> SwLevelingModeR {
        SwLevelingModeR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Additional delay needed for tDQSCK."]
    #[inline(always)]
    #[must_use]
    pub fn tdqsck_min_f1(&mut self) -> TdqsckMinF1W<DdrDenaliCtl222Spec> {
        TdqsckMinF1W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Additional delay needed for tDQSCK."]
    #[inline(always)]
    #[must_use]
    pub fn tdqsck_max_f2(&mut self) -> TdqsckMaxF2W<DdrDenaliCtl222Spec> {
        TdqsckMaxF2W::new(self, 8)
    }
    #[doc = "Bits 16:17 - Additional delay needed for tDQSCK."]
    #[inline(always)]
    #[must_use]
    pub fn tdqsck_min_f2(&mut self) -> TdqsckMinF2W<DdrDenaliCtl222Spec> {
        TdqsckMinF2W::new(self, 16)
    }
    #[doc = "Bits 24:26 - Defines the leveling operation for software leveling. Clear to 0 for none, program to 1 for write leveling, program to 2 for data eye training, or program to 3 for gate training."]
    #[inline(always)]
    #[must_use]
    pub fn sw_leveling_mode(&mut self) -> SwLevelingModeW<DdrDenaliCtl222Spec> {
        SwLevelingModeW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_222::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_222::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl222Spec;
impl crate::RegisterSpec for DdrDenaliCtl222Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_222::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl222Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_222::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl222Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_222 to value 0"]
impl crate::Resettable for DdrDenaliCtl222Spec {
    const RESET_VALUE: u32 = 0;
}
