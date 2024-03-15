#[doc = "Register `DENALI_CTL_50` reader"]
pub type R = crate::R<DenaliCtl50Spec>;
#[doc = "Register `DENALI_CTL_50` writer"]
pub type W = crate::W<DenaliCtl50Spec>;
#[doc = "Field `TRFC_F2` reader - DRAM TRFC value for frequency copy 2 in cycles."]
pub type TrfcF2R = crate::FieldReader<u16>;
#[doc = "Field `TRFC_F2` writer - DRAM TRFC value for frequency copy 2 in cycles."]
pub type TrfcF2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TREF_F2` reader - DRAM TREF value for frequency copy 2 in cycles."]
pub type TrefF2R = crate::FieldReader<u16>;
#[doc = "Field `TREF_F2` writer - DRAM TREF value for frequency copy 2 in cycles."]
pub type TrefF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:9 - DRAM TRFC value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn trfc_f2(&self) -> TrfcF2R {
        TrfcF2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:31 - DRAM TREF value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn tref_f2(&self) -> TrefF2R {
        TrefF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DRAM TRFC value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn trfc_f2(&mut self) -> TrfcF2W<DenaliCtl50Spec> {
        TrfcF2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - DRAM TREF value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tref_f2(&mut self) -> TrefF2W<DenaliCtl50Spec> {
        TrefF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_50::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_50::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl50Spec;
impl crate::RegisterSpec for DenaliCtl50Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_50::R`](R) reader structure"]
impl crate::Readable for DenaliCtl50Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_50::W`](W) writer structure"]
impl crate::Writable for DenaliCtl50Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_50 to value 0"]
impl crate::Resettable for DenaliCtl50Spec {
    const RESET_VALUE: u32 = 0;
}
