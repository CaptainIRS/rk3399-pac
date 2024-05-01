#[doc = "Register `SWREG133` reader"]
pub type R = crate::R<Swreg133Spec>;
#[doc = "Register `SWREG133` writer"]
pub type W = crate::W<Swreg133Spec>;
#[doc = "Field `MFR_REG13` reader - multi format reuse register13 except h264\n\nVP6:\n\n\\[29:24\\]
: 51st coef of scan read index\n\n\\[23:18\\]
: 52st coef of scan read index\n\n\\[17:12\\]
: 53st coef of scan read index\n\n\\[11:6\\]
: 54st coef of scan read index\n\n\\[5:0\\]
: 55st coef of scan read index\n\nVP7:\n\n\\[27:21\\]
: reference frame type0 adjustment of filter level\n\n\\[20:14\\]
: reference frame type1 adjustment of filter level\n\n\\[13:7\\]
: reference frame type2 adjustment of filter level\n\n\\[6:0\\]
: reference frame type3 adjustment of filter level"]
pub type MfrReg13R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG13` writer - multi format reuse register13 except h264\n\nVP6:\n\n\\[29:24\\]
: 51st coef of scan read index\n\n\\[23:18\\]
: 52st coef of scan read index\n\n\\[17:12\\]
: 53st coef of scan read index\n\n\\[11:6\\]
: 54st coef of scan read index\n\n\\[5:0\\]
: 55st coef of scan read index\n\nVP7:\n\n\\[27:21\\]
: reference frame type0 adjustment of filter level\n\n\\[20:14\\]
: reference frame type1 adjustment of filter level\n\n\\[13:7\\]
: reference frame type2 adjustment of filter level\n\n\\[6:0\\]
: reference frame type3 adjustment of filter level"]
pub type MfrReg13W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register13 except h264\n\nVP6:\n\n\\[29:24\\]
: 51st coef of scan read index\n\n\\[23:18\\]
: 52st coef of scan read index\n\n\\[17:12\\]
: 53st coef of scan read index\n\n\\[11:6\\]
: 54st coef of scan read index\n\n\\[5:0\\]
: 55st coef of scan read index\n\nVP7:\n\n\\[27:21\\]
: reference frame type0 adjustment of filter level\n\n\\[20:14\\]
: reference frame type1 adjustment of filter level\n\n\\[13:7\\]
: reference frame type2 adjustment of filter level\n\n\\[6:0\\]
: reference frame type3 adjustment of filter level"]
    #[inline(always)]
    pub fn mfr_reg13(&self) -> MfrReg13R {
        MfrReg13R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register13 except h264\n\nVP6:\n\n\\[29:24\\]
: 51st coef of scan read index\n\n\\[23:18\\]
: 52st coef of scan read index\n\n\\[17:12\\]
: 53st coef of scan read index\n\n\\[11:6\\]
: 54st coef of scan read index\n\n\\[5:0\\]
: 55st coef of scan read index\n\nVP7:\n\n\\[27:21\\]
: reference frame type0 adjustment of filter level\n\n\\[20:14\\]
: reference frame type1 adjustment of filter level\n\n\\[13:7\\]
: reference frame type2 adjustment of filter level\n\n\\[6:0\\]
: reference frame type3 adjustment of filter level"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg13(&mut self) -> MfrReg13W<Swreg133Spec> {
        MfrReg13W::new(self, 0)
    }
}
#[doc = "multi format reuse register13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg133::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg133::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg133Spec;
impl crate::RegisterSpec for Swreg133Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg133::R`](R) reader structure"]
impl crate::Readable for Swreg133Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg133::W`](W) writer structure"]
impl crate::Writable for Swreg133Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG133 to value 0"]
impl crate::Resettable for Swreg133Spec {
    const RESET_VALUE: u32 = 0;
}
