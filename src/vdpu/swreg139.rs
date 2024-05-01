#[doc = "Register `SWREG139` reader"]
pub type R = crate::R<Swreg139Spec>;
#[doc = "Register `SWREG139` writer"]
pub type W = crate::W<Swreg139Spec>;
#[doc = "Field `MFR_REG19` reader - multi format reuse register19 except h264\n\nMPEG4:\n\n\\[26:0\\]
: reference distance syntax for delta value1 be used\n\nJPEG:\n\n\\[31:24\\]
: tab2:code words of length 12\n\n\\[23:16\\]
: tab2:code words of length 11\n\n\\[15:8\\]
: tab2:code words of length 10\n\n\\[7:0\\]
: tab2:code words of length 9\n\nVP6/VP7:\n\n\\[29:24\\]
: 11st coef of scan read index\n\n\\[23:18\\]
: 12st coef of scan read index\n\n\\[17:12\\]
: 13st coef of scan read index\n\n\\[11:6\\]
: 14st coef of scan read index\n\n\\[5:0\\]
: 15st coef of scan read index"]
pub type MfrReg19R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG19` writer - multi format reuse register19 except h264\n\nMPEG4:\n\n\\[26:0\\]
: reference distance syntax for delta value1 be used\n\nJPEG:\n\n\\[31:24\\]
: tab2:code words of length 12\n\n\\[23:16\\]
: tab2:code words of length 11\n\n\\[15:8\\]
: tab2:code words of length 10\n\n\\[7:0\\]
: tab2:code words of length 9\n\nVP6/VP7:\n\n\\[29:24\\]
: 11st coef of scan read index\n\n\\[23:18\\]
: 12st coef of scan read index\n\n\\[17:12\\]
: 13st coef of scan read index\n\n\\[11:6\\]
: 14st coef of scan read index\n\n\\[5:0\\]
: 15st coef of scan read index"]
pub type MfrReg19W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register19 except h264\n\nMPEG4:\n\n\\[26:0\\]
: reference distance syntax for delta value1 be used\n\nJPEG:\n\n\\[31:24\\]
: tab2:code words of length 12\n\n\\[23:16\\]
: tab2:code words of length 11\n\n\\[15:8\\]
: tab2:code words of length 10\n\n\\[7:0\\]
: tab2:code words of length 9\n\nVP6/VP7:\n\n\\[29:24\\]
: 11st coef of scan read index\n\n\\[23:18\\]
: 12st coef of scan read index\n\n\\[17:12\\]
: 13st coef of scan read index\n\n\\[11:6\\]
: 14st coef of scan read index\n\n\\[5:0\\]
: 15st coef of scan read index"]
    #[inline(always)]
    pub fn mfr_reg19(&self) -> MfrReg19R {
        MfrReg19R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register19 except h264\n\nMPEG4:\n\n\\[26:0\\]
: reference distance syntax for delta value1 be used\n\nJPEG:\n\n\\[31:24\\]
: tab2:code words of length 12\n\n\\[23:16\\]
: tab2:code words of length 11\n\n\\[15:8\\]
: tab2:code words of length 10\n\n\\[7:0\\]
: tab2:code words of length 9\n\nVP6/VP7:\n\n\\[29:24\\]
: 11st coef of scan read index\n\n\\[23:18\\]
: 12st coef of scan read index\n\n\\[17:12\\]
: 13st coef of scan read index\n\n\\[11:6\\]
: 14st coef of scan read index\n\n\\[5:0\\]
: 15st coef of scan read index"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg19(&mut self) -> MfrReg19W<Swreg139Spec> {
        MfrReg19W::new(self, 0)
    }
}
#[doc = "multi format reuse register19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg139::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg139::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg139Spec;
impl crate::RegisterSpec for Swreg139Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg139::R`](R) reader structure"]
impl crate::Readable for Swreg139Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg139::W`](W) writer structure"]
impl crate::Writable for Swreg139Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG139 to value 0"]
impl crate::Resettable for Swreg139Spec {
    const RESET_VALUE: u32 = 0;
}
