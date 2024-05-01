#[doc = "Register `SWREG158` reader"]
pub type R = crate::R<Swreg158Spec>;
#[doc = "Register `SWREG158` writer"]
pub type W = crate::W<Swreg158Spec>;
#[doc = "Field `MFR_REG38` reader - multi format reuse register38 except h264\n\nVP6/VP7:\n\n\\[31:22\\]
: prediction filter with set 4,tap2\n\n\\[21:12\\]
: prediction filter with set 4,tap3\n\n\\[11:2\\]
: prediction filter with set 5,tap0"]
pub type MfrReg38R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG38` writer - multi format reuse register38 except h264\n\nVP6/VP7:\n\n\\[31:22\\]
: prediction filter with set 4,tap2\n\n\\[21:12\\]
: prediction filter with set 4,tap3\n\n\\[11:2\\]
: prediction filter with set 5,tap0"]
pub type MfrReg38W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register38 except h264\n\nVP6/VP7:\n\n\\[31:22\\]
: prediction filter with set 4,tap2\n\n\\[21:12\\]
: prediction filter with set 4,tap3\n\n\\[11:2\\]
: prediction filter with set 5,tap0"]
    #[inline(always)]
    pub fn mfr_reg38(&self) -> MfrReg38R {
        MfrReg38R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register38 except h264\n\nVP6/VP7:\n\n\\[31:22\\]
: prediction filter with set 4,tap2\n\n\\[21:12\\]
: prediction filter with set 4,tap3\n\n\\[11:2\\]
: prediction filter with set 5,tap0"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg38(&mut self) -> MfrReg38W<Swreg158Spec> {
        MfrReg38W::new(self, 0)
    }
}
#[doc = "multi format reuse register38\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg158::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg158::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg158Spec;
impl crate::RegisterSpec for Swreg158Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg158::R`](R) reader structure"]
impl crate::Readable for Swreg158Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg158::W`](W) writer structure"]
impl crate::Writable for Swreg158Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG158 to value 0"]
impl crate::Resettable for Swreg158Spec {
    const RESET_VALUE: u32 = 0;
}
