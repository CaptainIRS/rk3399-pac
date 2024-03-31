#[doc = "Register `DENALI_CTL_30` reader"]
pub type R = crate::R<DenaliCtl30Spec>;
#[doc = "Register `DENALI_CTL_30` writer"]
pub type W = crate::W<DenaliCtl30Spec>;
#[doc = "Field `TRRD_F2` reader - DRAM TRRD value for frequency copy 2 in cycles."]
pub type TrrdF2R = crate::FieldReader;
#[doc = "Field `TRRD_F2` writer - DRAM TRRD value for frequency copy 2 in cycles."]
pub type TrrdF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRC_F2` reader - DRAM TRC value for frequency copy 2 in cycles."]
pub type TrcF2R = crate::FieldReader;
#[doc = "Field `TRC_F2` writer - DRAM TRC value for frequency copy 2 in cycles."]
pub type TrcF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRAS_MIN_F2` reader - DRAM TRAS_MIN value for frequency copy 2 in cycles."]
pub type TrasMinF2R = crate::FieldReader;
#[doc = "Field `TRAS_MIN_F2` writer - DRAM TRAS_MIN value for frequency copy 2 in cycles."]
pub type TrasMinF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TWTR_F2` reader - DRAM TWTR value for frequency copy 2 in cycles."]
pub type TwtrF2R = crate::FieldReader;
#[doc = "Field `TWTR_F2` writer - DRAM TWTR value for frequency copy 2 in cycles."]
pub type TwtrF2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:7 - DRAM TRRD value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn trrd_f2(&self) -> TrrdF2R {
        TrrdF2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DRAM TRC value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn trc_f2(&self) -> TrcF2R {
        TrcF2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DRAM TRAS_MIN value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn tras_min_f2(&self) -> TrasMinF2R {
        TrasMinF2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:29 - DRAM TWTR value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn twtr_f2(&self) -> TwtrF2R {
        TwtrF2R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DRAM TRRD value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn trrd_f2(&mut self) -> TrrdF2W<DenaliCtl30Spec> {
        TrrdF2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DRAM TRC value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn trc_f2(&mut self) -> TrcF2W<DenaliCtl30Spec> {
        TrcF2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - DRAM TRAS_MIN value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tras_min_f2(&mut self) -> TrasMinF2W<DenaliCtl30Spec> {
        TrasMinF2W::new(self, 16)
    }
    #[doc = "Bits 24:29 - DRAM TWTR value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn twtr_f2(&mut self) -> TwtrF2W<DenaliCtl30Spec> {
        TwtrF2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_30::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_30::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl30Spec;
impl crate::RegisterSpec for DenaliCtl30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_30::R`](R) reader structure"]
impl crate::Readable for DenaliCtl30Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_30::W`](W) writer structure"]
impl crate::Writable for DenaliCtl30Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_30 to value 0"]
impl crate::Resettable for DenaliCtl30Spec {
    const RESET_VALUE: u32 = 0;
}
