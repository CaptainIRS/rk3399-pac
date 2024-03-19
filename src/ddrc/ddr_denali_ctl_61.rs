#[doc = "Register `DDR_DENALI_CTL_61` reader"]
pub type R = crate::R<DdrDenaliCtl61Spec>;
#[doc = "Register `DDR_DENALI_CTL_61` writer"]
pub type W = crate::W<DdrDenaliCtl61Spec>;
#[doc = "Field `TXSNR_F1` reader - DRAM TXSNR value for frequency copy 1 in cycles."]
pub type TxsnrF1R = crate::FieldReader<u16>;
#[doc = "Field `TXSNR_F1` writer - DRAM TXSNR value for frequency copy 1 in cycles."]
pub type TxsnrF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TXSR_F2` reader - DRAM TXSR value for frequency copy 2 in cycles."]
pub type TxsrF2R = crate::FieldReader<u16>;
#[doc = "Field `TXSR_F2` writer - DRAM TXSR value for frequency copy 2 in cycles."]
pub type TxsrF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DRAM TXSNR value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn txsnr_f1(&self) -> TxsnrF1R {
        TxsnrF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - DRAM TXSR value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn txsr_f2(&self) -> TxsrF2R {
        TxsrF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DRAM TXSNR value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn txsnr_f1(&mut self) -> TxsnrF1W<DdrDenaliCtl61Spec> {
        TxsnrF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - DRAM TXSR value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn txsr_f2(&mut self) -> TxsrF2W<DdrDenaliCtl61Spec> {
        TxsrF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_61::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_61::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl61Spec;
impl crate::RegisterSpec for DdrDenaliCtl61Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_61::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl61Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_61::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl61Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_61 to value 0"]
impl crate::Resettable for DdrDenaliCtl61Spec {
    const RESET_VALUE: u32 = 0;
}
