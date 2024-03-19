#[doc = "Register `DDR_DENALI_CTL_54` reader"]
pub type R = crate::R<DdrDenaliCtl54Spec>;
#[doc = "Register `DDR_DENALI_CTL_54` writer"]
pub type W = crate::W<DdrDenaliCtl54Spec>;
#[doc = "Field `TXPDLL_F1` reader - DRAM TXPDLL value for frequency copy 1 in cycles."]
pub type TxpdllF1R = crate::FieldReader<u16>;
#[doc = "Field `TXPDLL_F1` writer - DRAM TXPDLL value for frequency copy 1 in cycles."]
pub type TxpdllF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TXPDLL_F2` reader - DRAM TXPDLL value for frequency copy 2 in cycles."]
pub type TxpdllF2R = crate::FieldReader<u16>;
#[doc = "Field `TXPDLL_F2` writer - DRAM TXPDLL value for frequency copy 2 in cycles."]
pub type TxpdllF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DRAM TXPDLL value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn txpdll_f1(&self) -> TxpdllF1R {
        TxpdllF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - DRAM TXPDLL value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn txpdll_f2(&self) -> TxpdllF2R {
        TxpdllF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DRAM TXPDLL value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn txpdll_f1(&mut self) -> TxpdllF1W<DdrDenaliCtl54Spec> {
        TxpdllF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - DRAM TXPDLL value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn txpdll_f2(&mut self) -> TxpdllF2W<DdrDenaliCtl54Spec> {
        TxpdllF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_54::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_54::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl54Spec;
impl crate::RegisterSpec for DdrDenaliCtl54Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_54::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl54Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_54::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl54Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_54 to value 0"]
impl crate::Resettable for DdrDenaliCtl54Spec {
    const RESET_VALUE: u32 = 0;
}
