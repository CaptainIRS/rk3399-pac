#[doc = "Register `SWREG138` reader"]
pub type R = crate::R<Swreg138Spec>;
#[doc = "Register `SWREG138` writer"]
pub type W = crate::W<Swreg138Spec>;
#[doc = "Field `MFR_REG18` reader - multi format reuse register18 except h264\n\nMPEG4:\n\n\\[26:0\\]
: reference distance syntax for delta value -1 be used\n\nJPEG:\n\n\\[31:24\\]
: tab2:code words of length 8\n\n\\[23:16\\]
: tab2:code words of length 7\n\n\\[14:8\\]
: tab2:code words of length 6\n\n\\[5:0\\]
: tab2:code words of length 5\n\nVP6/VP7:\n\n\\[29:24\\]
: 6st coef of scan read index\n\n\\[23:18\\]
: 7st coef of scan read index\n\n\\[17:12\\]
: 8st coef of scan read index\n\n\\[11:6\\]
: 9st coef of scan read index\n\n\\[5:0\\]
: 10st coef of scan read index"]
pub type MfrReg18R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG18` writer - multi format reuse register18 except h264\n\nMPEG4:\n\n\\[26:0\\]
: reference distance syntax for delta value -1 be used\n\nJPEG:\n\n\\[31:24\\]
: tab2:code words of length 8\n\n\\[23:16\\]
: tab2:code words of length 7\n\n\\[14:8\\]
: tab2:code words of length 6\n\n\\[5:0\\]
: tab2:code words of length 5\n\nVP6/VP7:\n\n\\[29:24\\]
: 6st coef of scan read index\n\n\\[23:18\\]
: 7st coef of scan read index\n\n\\[17:12\\]
: 8st coef of scan read index\n\n\\[11:6\\]
: 9st coef of scan read index\n\n\\[5:0\\]
: 10st coef of scan read index"]
pub type MfrReg18W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register18 except h264\n\nMPEG4:\n\n\\[26:0\\]
: reference distance syntax for delta value -1 be used\n\nJPEG:\n\n\\[31:24\\]
: tab2:code words of length 8\n\n\\[23:16\\]
: tab2:code words of length 7\n\n\\[14:8\\]
: tab2:code words of length 6\n\n\\[5:0\\]
: tab2:code words of length 5\n\nVP6/VP7:\n\n\\[29:24\\]
: 6st coef of scan read index\n\n\\[23:18\\]
: 7st coef of scan read index\n\n\\[17:12\\]
: 8st coef of scan read index\n\n\\[11:6\\]
: 9st coef of scan read index\n\n\\[5:0\\]
: 10st coef of scan read index"]
    #[inline(always)]
    pub fn mfr_reg18(&self) -> MfrReg18R {
        MfrReg18R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register18 except h264\n\nMPEG4:\n\n\\[26:0\\]
: reference distance syntax for delta value -1 be used\n\nJPEG:\n\n\\[31:24\\]
: tab2:code words of length 8\n\n\\[23:16\\]
: tab2:code words of length 7\n\n\\[14:8\\]
: tab2:code words of length 6\n\n\\[5:0\\]
: tab2:code words of length 5\n\nVP6/VP7:\n\n\\[29:24\\]
: 6st coef of scan read index\n\n\\[23:18\\]
: 7st coef of scan read index\n\n\\[17:12\\]
: 8st coef of scan read index\n\n\\[11:6\\]
: 9st coef of scan read index\n\n\\[5:0\\]
: 10st coef of scan read index"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg18(&mut self) -> MfrReg18W<Swreg138Spec> {
        MfrReg18W::new(self, 0)
    }
}
#[doc = "multi format reuse register18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg138::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg138::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg138Spec;
impl crate::RegisterSpec for Swreg138Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg138::R`](R) reader structure"]
impl crate::Readable for Swreg138Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg138::W`](W) writer structure"]
impl crate::Writable for Swreg138Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG138 to value 0"]
impl crate::Resettable for Swreg138Spec {
    const RESET_VALUE: u32 = 0;
}
