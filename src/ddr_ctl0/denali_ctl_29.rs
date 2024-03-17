#[doc = "Register `DENALI_CTL_29` reader"]
pub type R = crate::R<DenaliCtl29Spec>;
#[doc = "Register `DENALI_CTL_29` writer"]
pub type W = crate::W<DenaliCtl29Spec>;
#[doc = "Field `TWTR_F1` reader - DRAM TWTR value for frequency copy 1 in cycles."]
pub type TwtrF1R = crate::FieldReader;
#[doc = "Field `TWTR_F1` writer - DRAM TWTR value for frequency copy 1 in cycles."]
pub type TwtrF1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TRP_F1` reader - DRAM TRP value for frequency copy 1 in cycles."]
pub type TrpF1R = crate::FieldReader;
#[doc = "Field `TRP_F1` writer - DRAM TRP value for frequency copy 1 in cycles."]
pub type TrpF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TFAW_F1` reader - DRAM TFAW value for frequency copy 1 in cycles."]
pub type TfawF1R = crate::FieldReader;
#[doc = "Field `TFAW_F1` writer - DRAM TFAW value for frequency copy 1 in cycles."]
pub type TfawF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CA_DEFAULT_VAL_F1` reader - Defines how unused address/ command bits are driven for frequency copy 1. Set to 1 to use last value or clear to 0 to drive low."]
pub type CaDefaultValF1R = crate::BitReader;
#[doc = "Field `CA_DEFAULT_VAL_F1` writer - Defines how unused address/ command bits are driven for frequency copy 1. Set to 1 to use last value or clear to 0 to drive low."]
pub type CaDefaultValF1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - DRAM TWTR value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn twtr_f1(&self) -> TwtrF1R {
        TwtrF1R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:15 - DRAM TRP value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn trp_f1(&self) -> TrpF1R {
        TrpF1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DRAM TFAW value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn tfaw_f1(&self) -> TfawF1R {
        TfawF1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Defines how unused address/ command bits are driven for frequency copy 1. Set to 1 to use last value or clear to 0 to drive low."]
    #[inline(always)]
    pub fn ca_default_val_f1(&self) -> CaDefaultValF1R {
        CaDefaultValF1R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - DRAM TWTR value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn twtr_f1(&mut self) -> TwtrF1W<DenaliCtl29Spec> {
        TwtrF1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DRAM TRP value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn trp_f1(&mut self) -> TrpF1W<DenaliCtl29Spec> {
        TrpF1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - DRAM TFAW value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tfaw_f1(&mut self) -> TfawF1W<DenaliCtl29Spec> {
        TfawF1W::new(self, 16)
    }
    #[doc = "Bit 24 - Defines how unused address/ command bits are driven for frequency copy 1. Set to 1 to use last value or clear to 0 to drive low."]
    #[inline(always)]
    #[must_use]
    pub fn ca_default_val_f1(&mut self) -> CaDefaultValF1W<DenaliCtl29Spec> {
        CaDefaultValF1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_29::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_29::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl29Spec;
impl crate::RegisterSpec for DenaliCtl29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_29::R`](R) reader structure"]
impl crate::Readable for DenaliCtl29Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_29::W`](W) writer structure"]
impl crate::Writable for DenaliCtl29Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_29 to value 0"]
impl crate::Resettable for DenaliCtl29Spec {
    const RESET_VALUE: u32 = 0;
}
