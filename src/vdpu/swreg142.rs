#[doc = "Register `SWREG142` reader"]
pub type R = crate::R<Swreg142Spec>;
#[doc = "Register `SWREG142` writer"]
pub type W = crate::W<Swreg142Spec>;
#[doc = "Field `MFR_REG22` reader - multi format reuse register22 except h264\n\nJPEG:\n\n\\[31:28\\]
: tab1:code words of length 16\n\n\\[27:24\\]
: tab1:code words of length 15\n\n\\[23:20\\]
: tab1:code words of length 14\n\n\\[19:16\\]
: tab1:code words of length 13\n\n\\[15:12\\]
: tab1:code words of length 12\n\n\\[11:8\\]
: tab1:code words of length 11\n\n\\[6:4\\]
: tab1:code words of length 10\n\n\\[1:0\\]
: tab1:code words of length 9\n\nVP6:\n\n\\[29:24\\]
: 26st coef of scan read index\n\n\\[23:18\\]
: 27st coef of scan read index\n\n\\[17:12\\]
: 28st coef of scan read index\n\n\\[11:6\\]
: 29st coef of scan read index\n\n\\[5:0\\]
: 30st coef of scan read index\n\nVP7\n\n\\[31:2\\]
: DCT stream MB row 2,2n+3 start address"]
pub type MfrReg22R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG22` writer - multi format reuse register22 except h264\n\nJPEG:\n\n\\[31:28\\]
: tab1:code words of length 16\n\n\\[27:24\\]
: tab1:code words of length 15\n\n\\[23:20\\]
: tab1:code words of length 14\n\n\\[19:16\\]
: tab1:code words of length 13\n\n\\[15:12\\]
: tab1:code words of length 12\n\n\\[11:8\\]
: tab1:code words of length 11\n\n\\[6:4\\]
: tab1:code words of length 10\n\n\\[1:0\\]
: tab1:code words of length 9\n\nVP6:\n\n\\[29:24\\]
: 26st coef of scan read index\n\n\\[23:18\\]
: 27st coef of scan read index\n\n\\[17:12\\]
: 28st coef of scan read index\n\n\\[11:6\\]
: 29st coef of scan read index\n\n\\[5:0\\]
: 30st coef of scan read index\n\nVP7\n\n\\[31:2\\]
: DCT stream MB row 2,2n+3 start address"]
pub type MfrReg22W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register22 except h264\n\nJPEG:\n\n\\[31:28\\]
: tab1:code words of length 16\n\n\\[27:24\\]
: tab1:code words of length 15\n\n\\[23:20\\]
: tab1:code words of length 14\n\n\\[19:16\\]
: tab1:code words of length 13\n\n\\[15:12\\]
: tab1:code words of length 12\n\n\\[11:8\\]
: tab1:code words of length 11\n\n\\[6:4\\]
: tab1:code words of length 10\n\n\\[1:0\\]
: tab1:code words of length 9\n\nVP6:\n\n\\[29:24\\]
: 26st coef of scan read index\n\n\\[23:18\\]
: 27st coef of scan read index\n\n\\[17:12\\]
: 28st coef of scan read index\n\n\\[11:6\\]
: 29st coef of scan read index\n\n\\[5:0\\]
: 30st coef of scan read index\n\nVP7\n\n\\[31:2\\]
: DCT stream MB row 2,2n+3 start address"]
    #[inline(always)]
    pub fn mfr_reg22(&self) -> MfrReg22R {
        MfrReg22R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register22 except h264\n\nJPEG:\n\n\\[31:28\\]
: tab1:code words of length 16\n\n\\[27:24\\]
: tab1:code words of length 15\n\n\\[23:20\\]
: tab1:code words of length 14\n\n\\[19:16\\]
: tab1:code words of length 13\n\n\\[15:12\\]
: tab1:code words of length 12\n\n\\[11:8\\]
: tab1:code words of length 11\n\n\\[6:4\\]
: tab1:code words of length 10\n\n\\[1:0\\]
: tab1:code words of length 9\n\nVP6:\n\n\\[29:24\\]
: 26st coef of scan read index\n\n\\[23:18\\]
: 27st coef of scan read index\n\n\\[17:12\\]
: 28st coef of scan read index\n\n\\[11:6\\]
: 29st coef of scan read index\n\n\\[5:0\\]
: 30st coef of scan read index\n\nVP7\n\n\\[31:2\\]
: DCT stream MB row 2,2n+3 start address"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg22(&mut self) -> MfrReg22W<Swreg142Spec> {
        MfrReg22W::new(self, 0)
    }
}
#[doc = "multi format reuse register22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg142::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg142::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg142Spec;
impl crate::RegisterSpec for Swreg142Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg142::R`](R) reader structure"]
impl crate::Readable for Swreg142Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg142::W`](W) writer structure"]
impl crate::Writable for Swreg142Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG142 to value 0"]
impl crate::Resettable for Swreg142Spec {
    const RESET_VALUE: u32 = 0;
}
