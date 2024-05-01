#[doc = "Register `SWREG146` reader"]
pub type R = crate::R<Swreg146Spec>;
#[doc = "Register `SWREG146` writer"]
pub type W = crate::W<Swreg146Spec>;
#[doc = "Field `MFR_REG26` reader - multi format reuse register26 except h264\n\nJPEG:\n\n\\[31:28\\]
: tab3:code words of length 16\n\n\\[27:24\\]
: tab3:code words of length 15\n\n\\[23:20\\]
: tab3:code words of length 14\n\n\\[19:16\\]
: tab3:code words of length 13\n\n\\[15:12\\]
: tab3:code words of length 12\n\n\\[11:8\\]
: tab3:code words of length 11\n\n\\[6:4\\]
: tab3:code words of length 10\n\n\\[1:0\\]
: tab3:code words of length 9\n\nVP6:\n\n\\[29:24\\]
: 41st coef of scan read index\n\n\\[23:18\\]
: 42st coef of scan read index\n\n\\[17:12\\]
: 43st coef of scan read index\n\n\\[11:6\\]
: 44st coef of scan read index\n\n\\[5:0\\]
: 45st coef of scan read index\n\nVP7\n\n\\[31:2\\]
: DCT stream MB row 2,2n+6 start address"]
pub type MfrReg26R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG26` writer - multi format reuse register26 except h264\n\nJPEG:\n\n\\[31:28\\]
: tab3:code words of length 16\n\n\\[27:24\\]
: tab3:code words of length 15\n\n\\[23:20\\]
: tab3:code words of length 14\n\n\\[19:16\\]
: tab3:code words of length 13\n\n\\[15:12\\]
: tab3:code words of length 12\n\n\\[11:8\\]
: tab3:code words of length 11\n\n\\[6:4\\]
: tab3:code words of length 10\n\n\\[1:0\\]
: tab3:code words of length 9\n\nVP6:\n\n\\[29:24\\]
: 41st coef of scan read index\n\n\\[23:18\\]
: 42st coef of scan read index\n\n\\[17:12\\]
: 43st coef of scan read index\n\n\\[11:6\\]
: 44st coef of scan read index\n\n\\[5:0\\]
: 45st coef of scan read index\n\nVP7\n\n\\[31:2\\]
: DCT stream MB row 2,2n+6 start address"]
pub type MfrReg26W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register26 except h264\n\nJPEG:\n\n\\[31:28\\]
: tab3:code words of length 16\n\n\\[27:24\\]
: tab3:code words of length 15\n\n\\[23:20\\]
: tab3:code words of length 14\n\n\\[19:16\\]
: tab3:code words of length 13\n\n\\[15:12\\]
: tab3:code words of length 12\n\n\\[11:8\\]
: tab3:code words of length 11\n\n\\[6:4\\]
: tab3:code words of length 10\n\n\\[1:0\\]
: tab3:code words of length 9\n\nVP6:\n\n\\[29:24\\]
: 41st coef of scan read index\n\n\\[23:18\\]
: 42st coef of scan read index\n\n\\[17:12\\]
: 43st coef of scan read index\n\n\\[11:6\\]
: 44st coef of scan read index\n\n\\[5:0\\]
: 45st coef of scan read index\n\nVP7\n\n\\[31:2\\]
: DCT stream MB row 2,2n+6 start address"]
    #[inline(always)]
    pub fn mfr_reg26(&self) -> MfrReg26R {
        MfrReg26R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register26 except h264\n\nJPEG:\n\n\\[31:28\\]
: tab3:code words of length 16\n\n\\[27:24\\]
: tab3:code words of length 15\n\n\\[23:20\\]
: tab3:code words of length 14\n\n\\[19:16\\]
: tab3:code words of length 13\n\n\\[15:12\\]
: tab3:code words of length 12\n\n\\[11:8\\]
: tab3:code words of length 11\n\n\\[6:4\\]
: tab3:code words of length 10\n\n\\[1:0\\]
: tab3:code words of length 9\n\nVP6:\n\n\\[29:24\\]
: 41st coef of scan read index\n\n\\[23:18\\]
: 42st coef of scan read index\n\n\\[17:12\\]
: 43st coef of scan read index\n\n\\[11:6\\]
: 44st coef of scan read index\n\n\\[5:0\\]
: 45st coef of scan read index\n\nVP7\n\n\\[31:2\\]
: DCT stream MB row 2,2n+6 start address"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg26(&mut self) -> MfrReg26W<Swreg146Spec> {
        MfrReg26W::new(self, 0)
    }
}
#[doc = "multi format reuse register26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg146::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg146::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg146Spec;
impl crate::RegisterSpec for Swreg146Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg146::R`](R) reader structure"]
impl crate::Readable for Swreg146Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg146::W`](W) writer structure"]
impl crate::Writable for Swreg146Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG146 to value 0"]
impl crate::Resettable for Swreg146Spec {
    const RESET_VALUE: u32 = 0;
}
