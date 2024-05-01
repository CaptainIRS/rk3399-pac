#[doc = "Register `SWREG130` reader"]
pub type R = crate::R<Swreg130Spec>;
#[doc = "Register `SWREG130` writer"]
pub type W = crate::W<Swreg130Spec>;
#[doc = "Field `MFR_REG10` reader - multi format reuse register10 except h264\n\nVP6:\n\n\\[29:24\\]
: 61st coef of scan read index\n\n\\[23:18\\]
: 62st coef of scan read index\n\n\\[17:12\\]
: 63st coef of scan read index\n\nVP7:\n\n\\[21:11\\]
: QP0 for VP7 and quantisizer value\n\n\\[10:0\\]
: QP1 for VP7 and quantisizer value"]
pub type MfrReg10R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG10` writer - multi format reuse register10 except h264\n\nVP6:\n\n\\[29:24\\]
: 61st coef of scan read index\n\n\\[23:18\\]
: 62st coef of scan read index\n\n\\[17:12\\]
: 63st coef of scan read index\n\nVP7:\n\n\\[21:11\\]
: QP0 for VP7 and quantisizer value\n\n\\[10:0\\]
: QP1 for VP7 and quantisizer value"]
pub type MfrReg10W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register10 except h264\n\nVP6:\n\n\\[29:24\\]
: 61st coef of scan read index\n\n\\[23:18\\]
: 62st coef of scan read index\n\n\\[17:12\\]
: 63st coef of scan read index\n\nVP7:\n\n\\[21:11\\]
: QP0 for VP7 and quantisizer value\n\n\\[10:0\\]
: QP1 for VP7 and quantisizer value"]
    #[inline(always)]
    pub fn mfr_reg10(&self) -> MfrReg10R {
        MfrReg10R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register10 except h264\n\nVP6:\n\n\\[29:24\\]
: 61st coef of scan read index\n\n\\[23:18\\]
: 62st coef of scan read index\n\n\\[17:12\\]
: 63st coef of scan read index\n\nVP7:\n\n\\[21:11\\]
: QP0 for VP7 and quantisizer value\n\n\\[10:0\\]
: QP1 for VP7 and quantisizer value"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg10(&mut self) -> MfrReg10W<Swreg130Spec> {
        MfrReg10W::new(self, 0)
    }
}
#[doc = "multi format reuse register10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg130::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg130::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg130Spec;
impl crate::RegisterSpec for Swreg130Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg130::R`](R) reader structure"]
impl crate::Readable for Swreg130Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg130::W`](W) writer structure"]
impl crate::Writable for Swreg130Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG130 to value 0"]
impl crate::Resettable for Swreg130Spec {
    const RESET_VALUE: u32 = 0;
}
