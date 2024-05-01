#[doc = "Register `SWREG154` reader"]
pub type R = crate::R<Swreg154Spec>;
#[doc = "Register `SWREG154` writer"]
pub type W = crate::W<Swreg154Spec>;
#[doc = "Field `MFR_REG34` reader - multi format reuse register34 except h264\n\nVP6/VP7:\n\n\\[31:22\\]
: prediction filter with set 1,tap2\n\n\\[21:12\\]
: prediction filter with set 1,tap3\n\n\\[11:2\\]
: prediction filter with set 2,tap0"]
pub type MfrReg34R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG34` writer - multi format reuse register34 except h264\n\nVP6/VP7:\n\n\\[31:22\\]
: prediction filter with set 1,tap2\n\n\\[21:12\\]
: prediction filter with set 1,tap3\n\n\\[11:2\\]
: prediction filter with set 2,tap0"]
pub type MfrReg34W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register34 except h264\n\nVP6/VP7:\n\n\\[31:22\\]
: prediction filter with set 1,tap2\n\n\\[21:12\\]
: prediction filter with set 1,tap3\n\n\\[11:2\\]
: prediction filter with set 2,tap0"]
    #[inline(always)]
    pub fn mfr_reg34(&self) -> MfrReg34R {
        MfrReg34R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register34 except h264\n\nVP6/VP7:\n\n\\[31:22\\]
: prediction filter with set 1,tap2\n\n\\[21:12\\]
: prediction filter with set 1,tap3\n\n\\[11:2\\]
: prediction filter with set 2,tap0"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg34(&mut self) -> MfrReg34W<Swreg154Spec> {
        MfrReg34W::new(self, 0)
    }
}
#[doc = "multi format reuse register34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg154::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg154::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg154Spec;
impl crate::RegisterSpec for Swreg154Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg154::R`](R) reader structure"]
impl crate::Readable for Swreg154Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg154::W`](W) writer structure"]
impl crate::Writable for Swreg154Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG154 to value 0"]
impl crate::Resettable for Swreg154Spec {
    const RESET_VALUE: u32 = 0;
}
