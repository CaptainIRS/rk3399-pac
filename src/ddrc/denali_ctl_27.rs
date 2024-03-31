#[doc = "Register `DENALI_CTL_27` reader"]
pub type R = crate::R<DenaliCtl27Spec>;
#[doc = "Register `DENALI_CTL_27` writer"]
pub type W = crate::W<DenaliCtl27Spec>;
#[doc = "Field `TRAS_MIN_F0` reader - DRAM TRAS_MIN value for frequency copy 0 in cycles."]
pub type TrasMinF0R = crate::FieldReader;
#[doc = "Field `TRAS_MIN_F0` writer - DRAM TRAS_MIN value for frequency copy 0 in cycles."]
pub type TrasMinF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TWTR_F0` reader - DRAM TWTR value for frequency copy 0 in cycles."]
pub type TwtrF0R = crate::FieldReader;
#[doc = "Field `TWTR_F0` writer - DRAM TWTR value for frequency copy 0 in cycles."]
pub type TwtrF0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TRP_F0` reader - DRAM TRP value for frequency copy 0 in cycles."]
pub type TrpF0R = crate::FieldReader;
#[doc = "Field `TRP_F0` writer - DRAM TRP value for frequency copy 0 in cycles."]
pub type TrpF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TFAW_F0` reader - DRAM TFAW value for frequency copy 0 in cycles."]
pub type TfawF0R = crate::FieldReader;
#[doc = "Field `TFAW_F0` writer - DRAM TFAW value for frequency copy 0 in cycles."]
pub type TfawF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DRAM TRAS_MIN value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn tras_min_f0(&self) -> TrasMinF0R {
        TrasMinF0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - DRAM TWTR value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn twtr_f0(&self) -> TwtrF0R {
        TwtrF0R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:23 - DRAM TRP value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn trp_f0(&self) -> TrpF0R {
        TrpF0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DRAM TFAW value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn tfaw_f0(&self) -> TfawF0R {
        TfawF0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DRAM TRAS_MIN value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tras_min_f0(&mut self) -> TrasMinF0W<DenaliCtl27Spec> {
        TrasMinF0W::new(self, 0)
    }
    #[doc = "Bits 8:13 - DRAM TWTR value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn twtr_f0(&mut self) -> TwtrF0W<DenaliCtl27Spec> {
        TwtrF0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - DRAM TRP value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn trp_f0(&mut self) -> TrpF0W<DenaliCtl27Spec> {
        TrpF0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - DRAM TFAW value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tfaw_f0(&mut self) -> TfawF0W<DenaliCtl27Spec> {
        TfawF0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_27::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_27::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl27Spec;
impl crate::RegisterSpec for DenaliCtl27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_27::R`](R) reader structure"]
impl crate::Readable for DenaliCtl27Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_27::W`](W) writer structure"]
impl crate::Writable for DenaliCtl27Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_27 to value 0"]
impl crate::Resettable for DenaliCtl27Spec {
    const RESET_VALUE: u32 = 0;
}
