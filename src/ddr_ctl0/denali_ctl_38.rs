#[doc = "Register `DENALI_CTL_38` reader"]
pub type R = crate::R<DenaliCtl38Spec>;
#[doc = "Register `DENALI_CTL_38` writer"]
pub type W = crate::W<DenaliCtl38Spec>;
#[doc = "Field `TCKESR_F2` reader - Minimum CKE low pulse width during a self-refresh for frequency copy 2."]
pub type TckesrF2R = crate::FieldReader;
#[doc = "Field `TCKESR_F2` writer - Minimum CKE low pulse width during a self-refresh for frequency copy 2."]
pub type TckesrF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TPPD` reader - DRAM TPPD value in cycles."]
pub type TppdR = crate::FieldReader;
#[doc = "Field `TPPD` writer - DRAM TPPD value in cycles."]
pub type TppdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - Minimum CKE low pulse width during a self-refresh for frequency copy 2."]
    #[inline(always)]
    pub fn tckesr_f2(&self) -> TckesrF2R {
        TckesrF2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - DRAM TPPD value in cycles."]
    #[inline(always)]
    pub fn tppd(&self) -> TppdR {
        TppdR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Minimum CKE low pulse width during a self-refresh for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn tckesr_f2(&mut self) -> TckesrF2W<DenaliCtl38Spec> {
        TckesrF2W::new(self, 0)
    }
    #[doc = "Bits 8:10 - DRAM TPPD value in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tppd(&mut self) -> TppdW<DenaliCtl38Spec> {
        TppdW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_38::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_38::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl38Spec;
impl crate::RegisterSpec for DenaliCtl38Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_38::R`](R) reader structure"]
impl crate::Readable for DenaliCtl38Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_38::W`](W) writer structure"]
impl crate::Writable for DenaliCtl38Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_38 to value 0x0400"]
impl crate::Resettable for DenaliCtl38Spec {
    const RESET_VALUE: u32 = 0x0400;
}
