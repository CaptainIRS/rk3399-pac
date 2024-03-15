#[doc = "Register `DENALI_CTL_59` reader"]
pub type R = crate::R<DenaliCtl59Spec>;
#[doc = "Register `DENALI_CTL_59` writer"]
pub type W = crate::W<DenaliCtl59Spec>;
#[doc = "Field `TMRWCKEL_F2` reader - DRAM TMRWCKEL value for frequency copy 2 in cycles."]
pub type TmrwckelF2R = crate::FieldReader;
#[doc = "Field `TMRWCKEL_F2` writer - DRAM TMRWCKEL value for frequency copy 2 in cycles."]
pub type TmrwckelF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TZQCKE_F2` reader - DRAM TZQCKE value for frequency copy 2 in cycles."]
pub type TzqckeF2R = crate::FieldReader;
#[doc = "Field `TZQCKE_F2` writer - DRAM TZQCKE value for frequency copy 2 in cycles."]
pub type TzqckeF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TXSR_F0` reader - DRAM TXSR value for frequency copy 0 in cycles."]
pub type TxsrF0R = crate::FieldReader<u16>;
#[doc = "Field `TXSR_F0` writer - DRAM TXSR value for frequency copy 0 in cycles."]
pub type TxsrF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - DRAM TMRWCKEL value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn tmrwckel_f2(&self) -> TmrwckelF2R {
        TmrwckelF2R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - DRAM TZQCKE value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn tzqcke_f2(&self) -> TzqckeF2R {
        TzqckeF2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - DRAM TXSR value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn txsr_f0(&self) -> TxsrF0R {
        TxsrF0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - DRAM TMRWCKEL value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tmrwckel_f2(&mut self) -> TmrwckelF2W<DenaliCtl59Spec> {
        TmrwckelF2W::new(self, 0)
    }
    #[doc = "Bits 8:11 - DRAM TZQCKE value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tzqcke_f2(&mut self) -> TzqckeF2W<DenaliCtl59Spec> {
        TzqckeF2W::new(self, 8)
    }
    #[doc = "Bits 16:31 - DRAM TXSR value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn txsr_f0(&mut self) -> TxsrF0W<DenaliCtl59Spec> {
        TxsrF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_59::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_59::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl59Spec;
impl crate::RegisterSpec for DenaliCtl59Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_59::R`](R) reader structure"]
impl crate::Readable for DenaliCtl59Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_59::W`](W) writer structure"]
impl crate::Writable for DenaliCtl59Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_59 to value 0"]
impl crate::Resettable for DenaliCtl59Spec {
    const RESET_VALUE: u32 = 0;
}
