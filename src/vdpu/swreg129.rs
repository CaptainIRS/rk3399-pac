#[doc = "Register `SWREG129` reader"]
pub type R = crate::R<Swreg129Spec>;
#[doc = "Register `SWREG129` writer"]
pub type W = crate::W<Swreg129Spec>;
#[doc = "Field `MFR_REG9` reader - multi format reuse register9 except h264\n\nVP6:\n\n\\[29:24\\]
: 56st coef of scan read index\n\n\\[23:18\\]
: 57st coef of scan read index\n\n\\[17:12\\]
: 58st coef of scan read index\n\n\\[11:6\\]
: 59st coef of scan read index\n\n\\[5:0\\]
: 60st coef of scan read index"]
pub type MfrReg9R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG9` writer - multi format reuse register9 except h264\n\nVP6:\n\n\\[29:24\\]
: 56st coef of scan read index\n\n\\[23:18\\]
: 57st coef of scan read index\n\n\\[17:12\\]
: 58st coef of scan read index\n\n\\[11:6\\]
: 59st coef of scan read index\n\n\\[5:0\\]
: 60st coef of scan read index"]
pub type MfrReg9W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register9 except h264\n\nVP6:\n\n\\[29:24\\]
: 56st coef of scan read index\n\n\\[23:18\\]
: 57st coef of scan read index\n\n\\[17:12\\]
: 58st coef of scan read index\n\n\\[11:6\\]
: 59st coef of scan read index\n\n\\[5:0\\]
: 60st coef of scan read index"]
    #[inline(always)]
    pub fn mfr_reg9(&self) -> MfrReg9R {
        MfrReg9R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register9 except h264\n\nVP6:\n\n\\[29:24\\]
: 56st coef of scan read index\n\n\\[23:18\\]
: 57st coef of scan read index\n\n\\[17:12\\]
: 58st coef of scan read index\n\n\\[11:6\\]
: 59st coef of scan read index\n\n\\[5:0\\]
: 60st coef of scan read index"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg9(&mut self) -> MfrReg9W<Swreg129Spec> {
        MfrReg9W::new(self, 0)
    }
}
#[doc = "multi format reuse register9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg129::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg129::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg129Spec;
impl crate::RegisterSpec for Swreg129Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg129::R`](R) reader structure"]
impl crate::Readable for Swreg129Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg129::W`](W) writer structure"]
impl crate::Writable for Swreg129Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG129 to value 0"]
impl crate::Resettable for Swreg129Spec {
    const RESET_VALUE: u32 = 0;
}
