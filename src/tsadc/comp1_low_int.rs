#[doc = "Register `COMP1_LOW_INT` reader"]
pub type R = crate::R<Comp1LowIntSpec>;
#[doc = "Register `COMP1_LOW_INT` writer"]
pub type W = crate::W<Comp1LowIntSpec>;
#[doc = "Field `TSADC_COMP_SRC1` reader - TSADC low temperature level.\n\nTSADC output is lower than tsadc_comp, means the temperature\n\nis low.\n\nTSADC_LOW_INT will be valid."]
pub type TsadcCompSrc1R = crate::FieldReader<u16>;
#[doc = "Field `TSADC_COMP_SRC1` writer - TSADC low temperature level.\n\nTSADC output is lower than tsadc_comp, means the temperature\n\nis low.\n\nTSADC_LOW_INT will be valid."]
pub type TsadcCompSrc1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - TSADC low temperature level.\n\nTSADC output is lower than tsadc_comp, means the temperature\n\nis low.\n\nTSADC_LOW_INT will be valid."]
    #[inline(always)]
    pub fn tsadc_comp_src1(&self) -> TsadcCompSrc1R {
        TsadcCompSrc1R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - TSADC low temperature level.\n\nTSADC output is lower than tsadc_comp, means the temperature\n\nis low.\n\nTSADC_LOW_INT will be valid."]
    #[inline(always)]
    #[must_use]
    pub fn tsadc_comp_src1(&mut self) -> TsadcCompSrc1W<Comp1LowIntSpec> {
        TsadcCompSrc1W::new(self, 0)
    }
}
#[doc = "TSADC low temperature level for source 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp1_low_int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp1_low_int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp1LowIntSpec;
impl crate::RegisterSpec for Comp1LowIntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp1_low_int::R`](R) reader structure"]
impl crate::Readable for Comp1LowIntSpec {}
#[doc = "`write(|w| ..)` method takes [`comp1_low_int::W`](W) writer structure"]
impl crate::Writable for Comp1LowIntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMP1_LOW_INT to value 0"]
impl crate::Resettable for Comp1LowIntSpec {
    const RESET_VALUE: u32 = 0;
}
