#[doc = "Register `COMP0_INT` reader"]
pub type R = crate::R<Comp0IntSpec>;
#[doc = "Register `COMP0_INT` writer"]
pub type W = crate::W<Comp0IntSpec>;
#[doc = "Field `TSADC_COMP_SRC0` reader - TSADC high temperature level.\n\nTSADC output is bigger than tsadc_comp, means the temperature\n\nis high.\n\nTSADC_INT will be valid."]
pub type TsadcCompSrc0R = crate::FieldReader<u16>;
#[doc = "Field `TSADC_COMP_SRC0` writer - TSADC high temperature level.\n\nTSADC output is bigger than tsadc_comp, means the temperature\n\nis high.\n\nTSADC_INT will be valid."]
pub type TsadcCompSrc0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - TSADC high temperature level.\n\nTSADC output is bigger than tsadc_comp, means the temperature\n\nis high.\n\nTSADC_INT will be valid."]
    #[inline(always)]
    pub fn tsadc_comp_src0(&self) -> TsadcCompSrc0R {
        TsadcCompSrc0R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - TSADC high temperature level.\n\nTSADC output is bigger than tsadc_comp, means the temperature\n\nis high.\n\nTSADC_INT will be valid."]
    #[inline(always)]
    #[must_use]
    pub fn tsadc_comp_src0(&mut self) -> TsadcCompSrc0W<Comp0IntSpec> {
        TsadcCompSrc0W::new(self, 0)
    }
}
#[doc = "TSADC high temperature level for source 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp0_int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp0_int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp0IntSpec;
impl crate::RegisterSpec for Comp0IntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp0_int::R`](R) reader structure"]
impl crate::Readable for Comp0IntSpec {}
#[doc = "`write(|w| ..)` method takes [`comp0_int::W`](W) writer structure"]
impl crate::Writable for Comp0IntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMP0_INT to value 0"]
impl crate::Resettable for Comp0IntSpec {
    const RESET_VALUE: u32 = 0;
}
