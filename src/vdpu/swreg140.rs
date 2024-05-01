#[doc = "Register `SWREG140` reader"]
pub type R = crate::R<Swreg140Spec>;
#[doc = "Register `SWREG140` writer"]
pub type W = crate::W<Swreg140Spec>;
#[doc = "Field `MFR_REG20` reader - multi format reuse register20 except h264\n\nJPEG:\n\n\\[31:24\\]
: tab2:code words of length 16\n\n\\[23:16\\]
: tab2:code words of length 15\n\n\\[15:8\\]
: tab2:code words of length 14\n\n\\[7:0\\]
: tab2:code words of length 13\n\nVP6:\n\n\\[29:24\\]
: 16st coef of scan read index\n\n\\[23:18\\]
: 17st coef of scan read index\n\n\\[17:12\\]
: 18st coef of scan read index\n\n\\[11:6\\]
: 19st coef of scan read index\n\n\\[5:0\\]
: 20st coef of scan read index\n\nVP7:\n\n\\[31:2\\]
: DCT stream MB row 2,2n+1 start address"]
pub type MfrReg20R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG20` writer - multi format reuse register20 except h264\n\nJPEG:\n\n\\[31:24\\]
: tab2:code words of length 16\n\n\\[23:16\\]
: tab2:code words of length 15\n\n\\[15:8\\]
: tab2:code words of length 14\n\n\\[7:0\\]
: tab2:code words of length 13\n\nVP6:\n\n\\[29:24\\]
: 16st coef of scan read index\n\n\\[23:18\\]
: 17st coef of scan read index\n\n\\[17:12\\]
: 18st coef of scan read index\n\n\\[11:6\\]
: 19st coef of scan read index\n\n\\[5:0\\]
: 20st coef of scan read index\n\nVP7:\n\n\\[31:2\\]
: DCT stream MB row 2,2n+1 start address"]
pub type MfrReg20W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register20 except h264\n\nJPEG:\n\n\\[31:24\\]
: tab2:code words of length 16\n\n\\[23:16\\]
: tab2:code words of length 15\n\n\\[15:8\\]
: tab2:code words of length 14\n\n\\[7:0\\]
: tab2:code words of length 13\n\nVP6:\n\n\\[29:24\\]
: 16st coef of scan read index\n\n\\[23:18\\]
: 17st coef of scan read index\n\n\\[17:12\\]
: 18st coef of scan read index\n\n\\[11:6\\]
: 19st coef of scan read index\n\n\\[5:0\\]
: 20st coef of scan read index\n\nVP7:\n\n\\[31:2\\]
: DCT stream MB row 2,2n+1 start address"]
    #[inline(always)]
    pub fn mfr_reg20(&self) -> MfrReg20R {
        MfrReg20R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register20 except h264\n\nJPEG:\n\n\\[31:24\\]
: tab2:code words of length 16\n\n\\[23:16\\]
: tab2:code words of length 15\n\n\\[15:8\\]
: tab2:code words of length 14\n\n\\[7:0\\]
: tab2:code words of length 13\n\nVP6:\n\n\\[29:24\\]
: 16st coef of scan read index\n\n\\[23:18\\]
: 17st coef of scan read index\n\n\\[17:12\\]
: 18st coef of scan read index\n\n\\[11:6\\]
: 19st coef of scan read index\n\n\\[5:0\\]
: 20st coef of scan read index\n\nVP7:\n\n\\[31:2\\]
: DCT stream MB row 2,2n+1 start address"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg20(&mut self) -> MfrReg20W<Swreg140Spec> {
        MfrReg20W::new(self, 0)
    }
}
#[doc = "multi format reuse register20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg140::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg140::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg140Spec;
impl crate::RegisterSpec for Swreg140Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg140::R`](R) reader structure"]
impl crate::Readable for Swreg140Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg140::W`](W) writer structure"]
impl crate::Writable for Swreg140Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG140 to value 0"]
impl crate::Resettable for Swreg140Spec {
    const RESET_VALUE: u32 = 0;
}
