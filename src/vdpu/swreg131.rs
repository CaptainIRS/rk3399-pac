#[doc = "Register `SWREG131` reader"]
pub type R = crate::R<Swreg131Spec>;
#[doc = "Register `SWREG131` writer"]
pub type W = crate::W<Swreg131Spec>;
#[doc = "Field `MFR_REG11` reader - multi format reuse register11 except h264\n\nMPEG4/H263/vp6 /VP7:\n\n\\[31:2\\]
: reference pic0 start address\n\nJPEG:\n\n\\[31:2\\]
: the ch decoder output start address"]
pub type MfrReg11R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG11` writer - multi format reuse register11 except h264\n\nMPEG4/H263/vp6 /VP7:\n\n\\[31:2\\]
: reference pic0 start address\n\nJPEG:\n\n\\[31:2\\]
: the ch decoder output start address"]
pub type MfrReg11W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register11 except h264\n\nMPEG4/H263/vp6 /VP7:\n\n\\[31:2\\]
: reference pic0 start address\n\nJPEG:\n\n\\[31:2\\]
: the ch decoder output start address"]
    #[inline(always)]
    pub fn mfr_reg11(&self) -> MfrReg11R {
        MfrReg11R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register11 except h264\n\nMPEG4/H263/vp6 /VP7:\n\n\\[31:2\\]
: reference pic0 start address\n\nJPEG:\n\n\\[31:2\\]
: the ch decoder output start address"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg11(&mut self) -> MfrReg11W<Swreg131Spec> {
        MfrReg11W::new(self, 0)
    }
}
#[doc = "multi format reuse register11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg131::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg131::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg131Spec;
impl crate::RegisterSpec for Swreg131Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg131::R`](R) reader structure"]
impl crate::Readable for Swreg131Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg131::W`](W) writer structure"]
impl crate::Writable for Swreg131Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG131 to value 0"]
impl crate::Resettable for Swreg131Spec {
    const RESET_VALUE: u32 = 0;
}
