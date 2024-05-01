#[doc = "Register `SWREG141` reader"]
pub type R = crate::R<Swreg141Spec>;
#[doc = "Register `SWREG141` writer"]
pub type W = crate::W<Swreg141Spec>;
#[doc = "Field `MFR_REG21` reader - multi format reuse register21 except h264\n\nJPEG:\n\n\\[31:28\\]
: tab1:code words of length 8\n\n\\[27:24\\]
: tab1:code words of length 7\n\n\\[23:20\\]
: tab1:code words of length 6\n\n\\[19:16\\]
: tab1:code words of length 5\n\n\\[15:12\\]
: tab1:code words of length 4\n\n\\[11:8\\]
: tab1:code words of length 3\n\n\\[6:4\\]
: tab1:code words of length 2\n\n\\[1:0\\]
: tab1:code words of length 1\n\nVP6:\n\n\\[29:24\\]
: 21st coef of scan read index\n\n\\[23:18\\]
: 22st coef of scan read index\n\n\\[17:12\\]
: 23st coef of scan read index\n\n\\[11:6\\]
: 24st coef of scan read index\n\n\\[5:0\\]
: 25st coef of scan read index\n\nVP7\n\n\\[31:2\\]
: DCT stream MB row 2,2n+2 start address"]
pub type MfrReg21R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG21` writer - multi format reuse register21 except h264\n\nJPEG:\n\n\\[31:28\\]
: tab1:code words of length 8\n\n\\[27:24\\]
: tab1:code words of length 7\n\n\\[23:20\\]
: tab1:code words of length 6\n\n\\[19:16\\]
: tab1:code words of length 5\n\n\\[15:12\\]
: tab1:code words of length 4\n\n\\[11:8\\]
: tab1:code words of length 3\n\n\\[6:4\\]
: tab1:code words of length 2\n\n\\[1:0\\]
: tab1:code words of length 1\n\nVP6:\n\n\\[29:24\\]
: 21st coef of scan read index\n\n\\[23:18\\]
: 22st coef of scan read index\n\n\\[17:12\\]
: 23st coef of scan read index\n\n\\[11:6\\]
: 24st coef of scan read index\n\n\\[5:0\\]
: 25st coef of scan read index\n\nVP7\n\n\\[31:2\\]
: DCT stream MB row 2,2n+2 start address"]
pub type MfrReg21W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register21 except h264\n\nJPEG:\n\n\\[31:28\\]
: tab1:code words of length 8\n\n\\[27:24\\]
: tab1:code words of length 7\n\n\\[23:20\\]
: tab1:code words of length 6\n\n\\[19:16\\]
: tab1:code words of length 5\n\n\\[15:12\\]
: tab1:code words of length 4\n\n\\[11:8\\]
: tab1:code words of length 3\n\n\\[6:4\\]
: tab1:code words of length 2\n\n\\[1:0\\]
: tab1:code words of length 1\n\nVP6:\n\n\\[29:24\\]
: 21st coef of scan read index\n\n\\[23:18\\]
: 22st coef of scan read index\n\n\\[17:12\\]
: 23st coef of scan read index\n\n\\[11:6\\]
: 24st coef of scan read index\n\n\\[5:0\\]
: 25st coef of scan read index\n\nVP7\n\n\\[31:2\\]
: DCT stream MB row 2,2n+2 start address"]
    #[inline(always)]
    pub fn mfr_reg21(&self) -> MfrReg21R {
        MfrReg21R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register21 except h264\n\nJPEG:\n\n\\[31:28\\]
: tab1:code words of length 8\n\n\\[27:24\\]
: tab1:code words of length 7\n\n\\[23:20\\]
: tab1:code words of length 6\n\n\\[19:16\\]
: tab1:code words of length 5\n\n\\[15:12\\]
: tab1:code words of length 4\n\n\\[11:8\\]
: tab1:code words of length 3\n\n\\[6:4\\]
: tab1:code words of length 2\n\n\\[1:0\\]
: tab1:code words of length 1\n\nVP6:\n\n\\[29:24\\]
: 21st coef of scan read index\n\n\\[23:18\\]
: 22st coef of scan read index\n\n\\[17:12\\]
: 23st coef of scan read index\n\n\\[11:6\\]
: 24st coef of scan read index\n\n\\[5:0\\]
: 25st coef of scan read index\n\nVP7\n\n\\[31:2\\]
: DCT stream MB row 2,2n+2 start address"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg21(&mut self) -> MfrReg21W<Swreg141Spec> {
        MfrReg21W::new(self, 0)
    }
}
#[doc = "multi format reuse register21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg141::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg141::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg141Spec;
impl crate::RegisterSpec for Swreg141Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg141::R`](R) reader structure"]
impl crate::Readable for Swreg141Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg141::W`](W) writer structure"]
impl crate::Writable for Swreg141Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG141 to value 0"]
impl crate::Resettable for Swreg141Spec {
    const RESET_VALUE: u32 = 0;
}
