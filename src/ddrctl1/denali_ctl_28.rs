#[doc = "Register `DENALI_CTL_28` reader"]
pub type R = crate::R<DenaliCtl28Spec>;
#[doc = "Register `DENALI_CTL_28` writer"]
pub type W = crate::W<DenaliCtl28Spec>;
#[doc = "Field `CA_DEFAULT_VAL_F0` reader - Defines how unused address/ command bits are driven for frequency copy 0. Set to 1 to use last value or clear to 0 to drive low."]
pub type CaDefaultValF0R = crate::BitReader;
#[doc = "Field `CA_DEFAULT_VAL_F0` writer - Defines how unused address/ command bits are driven for frequency copy 0. Set to 1 to use last value or clear to 0 to drive low."]
pub type CaDefaultValF0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRRD_F1` reader - DRAM TRRD value for frequency copy 1 in cycles."]
pub type TrrdF1R = crate::FieldReader;
#[doc = "Field `TRRD_F1` writer - DRAM TRRD value for frequency copy 1 in cycles."]
pub type TrrdF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRC_F1` reader - DRAM TRC value for frequency copy 1 in cycles."]
pub type TrcF1R = crate::FieldReader;
#[doc = "Field `TRC_F1` writer - DRAM TRC value for frequency copy 1 in cycles."]
pub type TrcF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRAS_MIN_F1` reader - DRAM TRAS_MIN value for frequency copy 1 in cycles."]
pub type TrasMinF1R = crate::FieldReader;
#[doc = "Field `TRAS_MIN_F1` writer - DRAM TRAS_MIN value for frequency copy 1 in cycles."]
pub type TrasMinF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Defines how unused address/ command bits are driven for frequency copy 0. Set to 1 to use last value or clear to 0 to drive low."]
    #[inline(always)]
    pub fn ca_default_val_f0(&self) -> CaDefaultValF0R {
        CaDefaultValF0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - DRAM TRRD value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn trrd_f1(&self) -> TrrdF1R {
        TrrdF1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DRAM TRC value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn trc_f1(&self) -> TrcF1R {
        TrcF1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DRAM TRAS_MIN value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn tras_min_f1(&self) -> TrasMinF1R {
        TrasMinF1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Defines how unused address/ command bits are driven for frequency copy 0. Set to 1 to use last value or clear to 0 to drive low."]
    #[inline(always)]
    #[must_use]
    pub fn ca_default_val_f0(&mut self) -> CaDefaultValF0W<DenaliCtl28Spec> {
        CaDefaultValF0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DRAM TRRD value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn trrd_f1(&mut self) -> TrrdF1W<DenaliCtl28Spec> {
        TrrdF1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - DRAM TRC value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn trc_f1(&mut self) -> TrcF1W<DenaliCtl28Spec> {
        TrcF1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - DRAM TRAS_MIN value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tras_min_f1(&mut self) -> TrasMinF1W<DenaliCtl28Spec> {
        TrasMinF1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_28::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_28::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl28Spec;
impl crate::RegisterSpec for DenaliCtl28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_28::R`](R) reader structure"]
impl crate::Readable for DenaliCtl28Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_28::W`](W) writer structure"]
impl crate::Writable for DenaliCtl28Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_28 to value 0"]
impl crate::Resettable for DenaliCtl28Spec {
    const RESET_VALUE: u32 = 0;
}
