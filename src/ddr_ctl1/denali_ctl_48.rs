#[doc = "Register `DENALI_CTL_48` reader"]
pub type R = crate::R<DenaliCtl48Spec>;
#[doc = "Register `DENALI_CTL_48` writer"]
pub type W = crate::W<DenaliCtl48Spec>;
#[doc = "Field `TRFC_F0` reader - DRAM TRFC value for frequency copy 0 in cycles."]
pub type TrfcF0R = crate::FieldReader<u16>;
#[doc = "Field `TRFC_F0` writer - DRAM TRFC value for frequency copy 0 in cycles."]
pub type TrfcF0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TREF_F0` reader - DRAM TREF value for frequency copy 0 in cycles."]
pub type TrefF0R = crate::FieldReader<u16>;
#[doc = "Field `TREF_F0` writer - DRAM TREF value for frequency copy 0 in cycles."]
pub type TrefF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:9 - DRAM TRFC value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn trfc_f0(&self) -> TrfcF0R {
        TrfcF0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:31 - DRAM TREF value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn tref_f0(&self) -> TrefF0R {
        TrefF0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DRAM TRFC value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn trfc_f0(&mut self) -> TrfcF0W<DenaliCtl48Spec> {
        TrfcF0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - DRAM TREF value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tref_f0(&mut self) -> TrefF0W<DenaliCtl48Spec> {
        TrefF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_48::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_48::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl48Spec;
impl crate::RegisterSpec for DenaliCtl48Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_48::R`](R) reader structure"]
impl crate::Readable for DenaliCtl48Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_48::W`](W) writer structure"]
impl crate::Writable for DenaliCtl48Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_48 to value 0"]
impl crate::Resettable for DenaliCtl48Spec {
    const RESET_VALUE: u32 = 0;
}
