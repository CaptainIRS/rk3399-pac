#[doc = "Register `INT_PD` reader"]
pub type R = crate::R<IntPdSpec>;
#[doc = "Register `INT_PD` writer"]
pub type W = crate::W<IntPdSpec>;
#[doc = "Field `HT_IRQ_SRC0` reader - When TSADC output is bigger than COMP_INT, this bit will be valid,\n\nwhich means temperature is high, and the application should in\n\ncharge of this.\n\nwrite 1 to it , this bit will be cleared."]
pub type HtIrqSrc0R = crate::BitReader;
#[doc = "Field `HT_IRQ_SRC0` writer - When TSADC output is bigger than COMP_INT, this bit will be valid,\n\nwhich means temperature is high, and the application should in\n\ncharge of this.\n\nwrite 1 to it , this bit will be cleared."]
pub type HtIrqSrc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HT_IRQ_SRC1` reader - When TSADC output is bigger than COMP_INT, this bit will be valid,\n\nwhich means temperature is high, and the application should in\n\ncharge of this.\n\nwrite 1 to it , this bit will be cleared."]
pub type HtIrqSrc1R = crate::BitReader;
#[doc = "Field `HT_IRQ_SRC1` writer - When TSADC output is bigger than COMP_INT, this bit will be valid,\n\nwhich means temperature is high, and the application should in\n\ncharge of this.\n\nwrite 1 to it , this bit will be cleared."]
pub type HtIrqSrc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSHUT_O_SRC0` reader - TSHUT output status\n\nWhen TSADC output is bigger than COMP_SHUT, this bit will be\n\nvalid, which means temperature is VERY high, and the application\n\nshould in charge of this.\n\nwrite 1 to it , this bit will be cleared."]
pub type TshutOSrc0R = crate::BitReader;
#[doc = "Field `TSHUT_O_SRC0` writer - TSHUT output status\n\nWhen TSADC output is bigger than COMP_SHUT, this bit will be\n\nvalid, which means temperature is VERY high, and the application\n\nshould in charge of this.\n\nwrite 1 to it , this bit will be cleared."]
pub type TshutOSrc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSHUT_O_SRC1` reader - TSHUT output status\n\nWhen TSADC output is bigger than COMP_SHUT, this bit will be\n\nvalid, which means temperature is VERY high, and the application\n\nshould in charge of this.\n\nwrite 1 to it , this bit will be cleared."]
pub type TshutOSrc1R = crate::BitReader;
#[doc = "Field `TSHUT_O_SRC1` writer - TSHUT output status\n\nWhen TSADC output is bigger than COMP_SHUT, this bit will be\n\nvalid, which means temperature is VERY high, and the application\n\nshould in charge of this.\n\nwrite 1 to it , this bit will be cleared."]
pub type TshutOSrc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LT_IRQ_SRC0` reader - When TSADC output is lower than COMP_INT_LOW, this bit will be\n\nvalid, which means temperature is low, and the application should\n\nin charge of this.\n\nwrite 1 to it , this bit will be cleared."]
pub type LtIrqSrc0R = crate::BitReader;
#[doc = "Field `LT_IRQ_SRC0` writer - When TSADC output is lower than COMP_INT_LOW, this bit will be\n\nvalid, which means temperature is low, and the application should\n\nin charge of this.\n\nwrite 1 to it , this bit will be cleared."]
pub type LtIrqSrc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LT_IRQ_SRC1` reader - When TSADC output is lower than COMP_INT_LOW, this bit will be\n\nvalid, which means temperature is low, and the application should\n\nin charge of this.\n\nwrite 1 to it , this bit will be cleared."]
pub type LtIrqSrc1R = crate::BitReader;
#[doc = "Field `LT_IRQ_SRC1` writer - When TSADC output is lower than COMP_INT_LOW, this bit will be\n\nvalid, which means temperature is low, and the application should\n\nin charge of this.\n\nwrite 1 to it , this bit will be cleared."]
pub type LtIrqSrc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC_INT_PD` reader - Interrupt status.\n\nThis bit will be set to 1 when end-of-conversion.\n\nSet 0 to clear the interrupt."]
pub type EocIntPdR = crate::BitReader;
#[doc = "Field `EOC_INT_PD` writer - Interrupt status.\n\nThis bit will be set to 1 when end-of-conversion.\n\nSet 0 to clear the interrupt."]
pub type EocIntPdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When TSADC output is bigger than COMP_INT, this bit will be valid,\n\nwhich means temperature is high, and the application should in\n\ncharge of this.\n\nwrite 1 to it , this bit will be cleared."]
    #[inline(always)]
    pub fn ht_irq_src0(&self) -> HtIrqSrc0R {
        HtIrqSrc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When TSADC output is bigger than COMP_INT, this bit will be valid,\n\nwhich means temperature is high, and the application should in\n\ncharge of this.\n\nwrite 1 to it , this bit will be cleared."]
    #[inline(always)]
    pub fn ht_irq_src1(&self) -> HtIrqSrc1R {
        HtIrqSrc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - TSHUT output status\n\nWhen TSADC output is bigger than COMP_SHUT, this bit will be\n\nvalid, which means temperature is VERY high, and the application\n\nshould in charge of this.\n\nwrite 1 to it , this bit will be cleared."]
    #[inline(always)]
    pub fn tshut_o_src0(&self) -> TshutOSrc0R {
        TshutOSrc0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TSHUT output status\n\nWhen TSADC output is bigger than COMP_SHUT, this bit will be\n\nvalid, which means temperature is VERY high, and the application\n\nshould in charge of this.\n\nwrite 1 to it , this bit will be cleared."]
    #[inline(always)]
    pub fn tshut_o_src1(&self) -> TshutOSrc1R {
        TshutOSrc1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - When TSADC output is lower than COMP_INT_LOW, this bit will be\n\nvalid, which means temperature is low, and the application should\n\nin charge of this.\n\nwrite 1 to it , this bit will be cleared."]
    #[inline(always)]
    pub fn lt_irq_src0(&self) -> LtIrqSrc0R {
        LtIrqSrc0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - When TSADC output is lower than COMP_INT_LOW, this bit will be\n\nvalid, which means temperature is low, and the application should\n\nin charge of this.\n\nwrite 1 to it , this bit will be cleared."]
    #[inline(always)]
    pub fn lt_irq_src1(&self) -> LtIrqSrc1R {
        LtIrqSrc1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt status.\n\nThis bit will be set to 1 when end-of-conversion.\n\nSet 0 to clear the interrupt."]
    #[inline(always)]
    pub fn eoc_int_pd(&self) -> EocIntPdR {
        EocIntPdR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When TSADC output is bigger than COMP_INT, this bit will be valid,\n\nwhich means temperature is high, and the application should in\n\ncharge of this.\n\nwrite 1 to it , this bit will be cleared."]
    #[inline(always)]
    #[must_use]
    pub fn ht_irq_src0(&mut self) -> HtIrqSrc0W<IntPdSpec> {
        HtIrqSrc0W::new(self, 0)
    }
    #[doc = "Bit 1 - When TSADC output is bigger than COMP_INT, this bit will be valid,\n\nwhich means temperature is high, and the application should in\n\ncharge of this.\n\nwrite 1 to it , this bit will be cleared."]
    #[inline(always)]
    #[must_use]
    pub fn ht_irq_src1(&mut self) -> HtIrqSrc1W<IntPdSpec> {
        HtIrqSrc1W::new(self, 1)
    }
    #[doc = "Bit 4 - TSHUT output status\n\nWhen TSADC output is bigger than COMP_SHUT, this bit will be\n\nvalid, which means temperature is VERY high, and the application\n\nshould in charge of this.\n\nwrite 1 to it , this bit will be cleared."]
    #[inline(always)]
    #[must_use]
    pub fn tshut_o_src0(&mut self) -> TshutOSrc0W<IntPdSpec> {
        TshutOSrc0W::new(self, 4)
    }
    #[doc = "Bit 5 - TSHUT output status\n\nWhen TSADC output is bigger than COMP_SHUT, this bit will be\n\nvalid, which means temperature is VERY high, and the application\n\nshould in charge of this.\n\nwrite 1 to it , this bit will be cleared."]
    #[inline(always)]
    #[must_use]
    pub fn tshut_o_src1(&mut self) -> TshutOSrc1W<IntPdSpec> {
        TshutOSrc1W::new(self, 5)
    }
    #[doc = "Bit 12 - When TSADC output is lower than COMP_INT_LOW, this bit will be\n\nvalid, which means temperature is low, and the application should\n\nin charge of this.\n\nwrite 1 to it , this bit will be cleared."]
    #[inline(always)]
    #[must_use]
    pub fn lt_irq_src0(&mut self) -> LtIrqSrc0W<IntPdSpec> {
        LtIrqSrc0W::new(self, 12)
    }
    #[doc = "Bit 13 - When TSADC output is lower than COMP_INT_LOW, this bit will be\n\nvalid, which means temperature is low, and the application should\n\nin charge of this.\n\nwrite 1 to it , this bit will be cleared."]
    #[inline(always)]
    #[must_use]
    pub fn lt_irq_src1(&mut self) -> LtIrqSrc1W<IntPdSpec> {
        LtIrqSrc1W::new(self, 13)
    }
    #[doc = "Bit 16 - Interrupt status.\n\nThis bit will be set to 1 when end-of-conversion.\n\nSet 0 to clear the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn eoc_int_pd(&mut self) -> EocIntPdW<IntPdSpec> {
        EocIntPdW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_pd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_pd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntPdSpec;
impl crate::RegisterSpec for IntPdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_pd::R`](R) reader structure"]
impl crate::Readable for IntPdSpec {}
#[doc = "`write(|w| ..)` method takes [`int_pd::W`](W) writer structure"]
impl crate::Writable for IntPdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_PD to value 0"]
impl crate::Resettable for IntPdSpec {
    const RESET_VALUE: u32 = 0;
}
