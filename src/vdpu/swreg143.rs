#[doc = "Register `SWREG143` reader"]
pub type R = crate::R<Swreg143Spec>;
#[doc = "Register `SWREG143` writer"]
pub type W = crate::W<Swreg143Spec>;
#[doc = "Field `MFR_REG23` reader - multi format reuse register23 except h264\n\nJPEG:\n\n\\[31:28\\]
: tab2:code words of length 8\n\n\\[27:24\\]
: tab2:code words of length 7\n\n\\[23:20\\]
: tab2:code words of length 6\n\n\\[19:16\\]
: tab2:code words of length 5\n\n\\[15:12\\]
: tab2:code words of length 4\n\n\\[11:8\\]
: tab2:code words of length 3\n\n\\[6:4\\]
: tab2:code words of length 2\n\n\\[1:0\\]
: tab2:code words of length 1\n\nVP6:\n\n\\[29:24\\]
: 31st coef of scan read index\n\n\\[23:18\\]
: 32st coef of scan read index\n\n\\[17:12\\]
: 33st coef of scan read index\n\n\\[11:6\\]
: 34st coef of scan read index\n\n\\[5:0\\]
: 35st coef of scan read index\n\nVP7\n\n\\[31:2\\]
: DCT stream MB row 2,2n+4 start address"]
pub type MfrReg23R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG23` writer - multi format reuse register23 except h264\n\nJPEG:\n\n\\[31:28\\]
: tab2:code words of length 8\n\n\\[27:24\\]
: tab2:code words of length 7\n\n\\[23:20\\]
: tab2:code words of length 6\n\n\\[19:16\\]
: tab2:code words of length 5\n\n\\[15:12\\]
: tab2:code words of length 4\n\n\\[11:8\\]
: tab2:code words of length 3\n\n\\[6:4\\]
: tab2:code words of length 2\n\n\\[1:0\\]
: tab2:code words of length 1\n\nVP6:\n\n\\[29:24\\]
: 31st coef of scan read index\n\n\\[23:18\\]
: 32st coef of scan read index\n\n\\[17:12\\]
: 33st coef of scan read index\n\n\\[11:6\\]
: 34st coef of scan read index\n\n\\[5:0\\]
: 35st coef of scan read index\n\nVP7\n\n\\[31:2\\]
: DCT stream MB row 2,2n+4 start address"]
pub type MfrReg23W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register23 except h264\n\nJPEG:\n\n\\[31:28\\]
: tab2:code words of length 8\n\n\\[27:24\\]
: tab2:code words of length 7\n\n\\[23:20\\]
: tab2:code words of length 6\n\n\\[19:16\\]
: tab2:code words of length 5\n\n\\[15:12\\]
: tab2:code words of length 4\n\n\\[11:8\\]
: tab2:code words of length 3\n\n\\[6:4\\]
: tab2:code words of length 2\n\n\\[1:0\\]
: tab2:code words of length 1\n\nVP6:\n\n\\[29:24\\]
: 31st coef of scan read index\n\n\\[23:18\\]
: 32st coef of scan read index\n\n\\[17:12\\]
: 33st coef of scan read index\n\n\\[11:6\\]
: 34st coef of scan read index\n\n\\[5:0\\]
: 35st coef of scan read index\n\nVP7\n\n\\[31:2\\]
: DCT stream MB row 2,2n+4 start address"]
    #[inline(always)]
    pub fn mfr_reg23(&self) -> MfrReg23R {
        MfrReg23R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register23 except h264\n\nJPEG:\n\n\\[31:28\\]
: tab2:code words of length 8\n\n\\[27:24\\]
: tab2:code words of length 7\n\n\\[23:20\\]
: tab2:code words of length 6\n\n\\[19:16\\]
: tab2:code words of length 5\n\n\\[15:12\\]
: tab2:code words of length 4\n\n\\[11:8\\]
: tab2:code words of length 3\n\n\\[6:4\\]
: tab2:code words of length 2\n\n\\[1:0\\]
: tab2:code words of length 1\n\nVP6:\n\n\\[29:24\\]
: 31st coef of scan read index\n\n\\[23:18\\]
: 32st coef of scan read index\n\n\\[17:12\\]
: 33st coef of scan read index\n\n\\[11:6\\]
: 34st coef of scan read index\n\n\\[5:0\\]
: 35st coef of scan read index\n\nVP7\n\n\\[31:2\\]
: DCT stream MB row 2,2n+4 start address"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg23(&mut self) -> MfrReg23W<Swreg143Spec> {
        MfrReg23W::new(self, 0)
    }
}
#[doc = "multi format reuse register23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg143::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg143::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg143Spec;
impl crate::RegisterSpec for Swreg143Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg143::R`](R) reader structure"]
impl crate::Readable for Swreg143Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg143::W`](W) writer structure"]
impl crate::Writable for Swreg143Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG143 to value 0"]
impl crate::Resettable for Swreg143Spec {
    const RESET_VALUE: u32 = 0;
}
