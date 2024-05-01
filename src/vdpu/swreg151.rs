#[doc = "Register `SWREG151` reader"]
pub type R = crate::R<Swreg151Spec>;
#[doc = "Register `SWREG151` writer"]
pub type W = crate::W<Swreg151Spec>;
#[doc = "Field `MFR_REG31` reader - multi format reuse register31 except h264\n\nVP7:\n\n\\[21:11\\]
: QP2 for VP7 and quantisizer value\n\n\\[10:0\\]
: QP3 for VP7 and quantisizer value"]
pub type MfrReg31R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG31` writer - multi format reuse register31 except h264\n\nVP7:\n\n\\[21:11\\]
: QP2 for VP7 and quantisizer value\n\n\\[10:0\\]
: QP3 for VP7 and quantisizer value"]
pub type MfrReg31W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register31 except h264\n\nVP7:\n\n\\[21:11\\]
: QP2 for VP7 and quantisizer value\n\n\\[10:0\\]
: QP3 for VP7 and quantisizer value"]
    #[inline(always)]
    pub fn mfr_reg31(&self) -> MfrReg31R {
        MfrReg31R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register31 except h264\n\nVP7:\n\n\\[21:11\\]
: QP2 for VP7 and quantisizer value\n\n\\[10:0\\]
: QP3 for VP7 and quantisizer value"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg31(&mut self) -> MfrReg31W<Swreg151Spec> {
        MfrReg31W::new(self, 0)
    }
}
#[doc = "multi format reuse register31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg151::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg151::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg151Spec;
impl crate::RegisterSpec for Swreg151Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg151::R`](R) reader structure"]
impl crate::Readable for Swreg151Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg151::W`](W) writer structure"]
impl crate::Writable for Swreg151Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG151 to value 0"]
impl crate::Resettable for Swreg151Spec {
    const RESET_VALUE: u32 = 0;
}
