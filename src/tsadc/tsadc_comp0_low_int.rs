#[doc = "Register `TSADC_COMP0_LOW_INT` reader"]
pub type R = crate::R<TsadcComp0LowIntSpec>;
#[doc = "Register `TSADC_COMP0_LOW_INT` writer"]
pub type W = crate::W<TsadcComp0LowIntSpec>;
#[doc = "Field `TSADC_COMP_SRC0` reader - TSADC low temperature level.\n\nTSADC output is lower than tsadc_comp, means the temperature\n\nis low.\n\nTSADC_LOW_INT will be valid."]
pub type TsadcCompSrc0R = crate::FieldReader<u16>;
#[doc = "Field `TSADC_COMP_SRC0` writer - TSADC low temperature level.\n\nTSADC output is lower than tsadc_comp, means the temperature\n\nis low.\n\nTSADC_LOW_INT will be valid."]
pub type TsadcCompSrc0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - TSADC low temperature level.\n\nTSADC output is lower than tsadc_comp, means the temperature\n\nis low.\n\nTSADC_LOW_INT will be valid."]
    #[inline(always)]
    pub fn tsadc_comp_src0(&self) -> TsadcCompSrc0R {
        TsadcCompSrc0R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - TSADC low temperature level.\n\nTSADC output is lower than tsadc_comp, means the temperature\n\nis low.\n\nTSADC_LOW_INT will be valid."]
    #[inline(always)]
    #[must_use]
    pub fn tsadc_comp_src0(&mut self) -> TsadcCompSrc0W<TsadcComp0LowIntSpec> {
        TsadcCompSrc0W::new(self, 0)
    }
}
#[doc = "TSADC low temperature level for source 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsadc_comp0_low_int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsadc_comp0_low_int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsadcComp0LowIntSpec;
impl crate::RegisterSpec for TsadcComp0LowIntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsadc_comp0_low_int::R`](R) reader structure"]
impl crate::Readable for TsadcComp0LowIntSpec {}
#[doc = "`write(|w| ..)` method takes [`tsadc_comp0_low_int::W`](W) writer structure"]
impl crate::Writable for TsadcComp0LowIntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSADC_COMP0_LOW_INT to value 0"]
impl crate::Resettable for TsadcComp0LowIntSpec {
    const RESET_VALUE: u32 = 0;
}
