#[doc = "Register `SWREG152` reader"]
pub type R = crate::R<Swreg152Spec>;
#[doc = "Register `SWREG152` writer"]
pub type W = crate::W<Swreg152Spec>;
#[doc = "Field `MFR_REG32` reader - multi format reuse register32 except h264\n\nVP7:\n\n\\[21:11\\]
: QP4 for VP7 and quantisizer value\n\n\\[10:0\\]
: QP5 for VP7 and quantisizer value"]
pub type MfrReg32R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG32` writer - multi format reuse register32 except h264\n\nVP7:\n\n\\[21:11\\]
: QP4 for VP7 and quantisizer value\n\n\\[10:0\\]
: QP5 for VP7 and quantisizer value"]
pub type MfrReg32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register32 except h264\n\nVP7:\n\n\\[21:11\\]
: QP4 for VP7 and quantisizer value\n\n\\[10:0\\]
: QP5 for VP7 and quantisizer value"]
    #[inline(always)]
    pub fn mfr_reg32(&self) -> MfrReg32R {
        MfrReg32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register32 except h264\n\nVP7:\n\n\\[21:11\\]
: QP4 for VP7 and quantisizer value\n\n\\[10:0\\]
: QP5 for VP7 and quantisizer value"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg32(&mut self) -> MfrReg32W<Swreg152Spec> {
        MfrReg32W::new(self, 0)
    }
}
#[doc = "multi format reuse register32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg152::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg152::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg152Spec;
impl crate::RegisterSpec for Swreg152Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg152::R`](R) reader structure"]
impl crate::Readable for Swreg152Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg152::W`](W) writer structure"]
impl crate::Writable for Swreg152Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG152 to value 0"]
impl crate::Resettable for Swreg152Spec {
    const RESET_VALUE: u32 = 0;
}
