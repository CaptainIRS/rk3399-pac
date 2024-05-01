#[doc = "Register `SWREG147` reader"]
pub type R = crate::R<Swreg147Spec>;
#[doc = "Register `SWREG147` writer"]
pub type W = crate::W<Swreg147Spec>;
#[doc = "Field `MFR_REG27` reader - multi format reuse register27 except h264\n\nVP6:\n\n\\[29:24\\]
: 46st coef of scan read index\n\n\\[23:18\\]
: 47st coef of scan read index\n\n\\[17:12\\]
: 48st coef of scan read index\n\n\\[11:6\\]
: 49st coef of scan read index\n\n\\[5:0\\]
: 50st coef of scan read index\n\nVP7\n\n\\[31:2\\]
: DCT stream MB row 2,2n+7 start address"]
pub type MfrReg27R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG27` writer - multi format reuse register27 except h264\n\nVP6:\n\n\\[29:24\\]
: 46st coef of scan read index\n\n\\[23:18\\]
: 47st coef of scan read index\n\n\\[17:12\\]
: 48st coef of scan read index\n\n\\[11:6\\]
: 49st coef of scan read index\n\n\\[5:0\\]
: 50st coef of scan read index\n\nVP7\n\n\\[31:2\\]
: DCT stream MB row 2,2n+7 start address"]
pub type MfrReg27W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register27 except h264\n\nVP6:\n\n\\[29:24\\]
: 46st coef of scan read index\n\n\\[23:18\\]
: 47st coef of scan read index\n\n\\[17:12\\]
: 48st coef of scan read index\n\n\\[11:6\\]
: 49st coef of scan read index\n\n\\[5:0\\]
: 50st coef of scan read index\n\nVP7\n\n\\[31:2\\]
: DCT stream MB row 2,2n+7 start address"]
    #[inline(always)]
    pub fn mfr_reg27(&self) -> MfrReg27R {
        MfrReg27R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register27 except h264\n\nVP6:\n\n\\[29:24\\]
: 46st coef of scan read index\n\n\\[23:18\\]
: 47st coef of scan read index\n\n\\[17:12\\]
: 48st coef of scan read index\n\n\\[11:6\\]
: 49st coef of scan read index\n\n\\[5:0\\]
: 50st coef of scan read index\n\nVP7\n\n\\[31:2\\]
: DCT stream MB row 2,2n+7 start address"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg27(&mut self) -> MfrReg27W<Swreg147Spec> {
        MfrReg27W::new(self, 0)
    }
}
#[doc = "multi format reuse register27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg147::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg147::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg147Spec;
impl crate::RegisterSpec for Swreg147Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg147::R`](R) reader structure"]
impl crate::Readable for Swreg147Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg147::W`](W) writer structure"]
impl crate::Writable for Swreg147Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG147 to value 0"]
impl crate::Resettable for Swreg147Spec {
    const RESET_VALUE: u32 = 0;
}
