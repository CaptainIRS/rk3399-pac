#[doc = "Register `SWREG156` reader"]
pub type R = crate::R<Swreg156Spec>;
#[doc = "Register `SWREG156` writer"]
pub type W = crate::W<Swreg156Spec>;
#[doc = "Field `MFR_REG36` reader - multi format reuse register36 except h264\n\nVP6/VP7:\n\n\\[31:22\\]
: prediction filter with set 3,tap0\n\n\\[21:12\\]
: prediction filter with set 3,tap1\n\n\\[11:2\\]
: prediction filter with set 3,tap2"]
pub type MfrReg36R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG36` writer - multi format reuse register36 except h264\n\nVP6/VP7:\n\n\\[31:22\\]
: prediction filter with set 3,tap0\n\n\\[21:12\\]
: prediction filter with set 3,tap1\n\n\\[11:2\\]
: prediction filter with set 3,tap2"]
pub type MfrReg36W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register36 except h264\n\nVP6/VP7:\n\n\\[31:22\\]
: prediction filter with set 3,tap0\n\n\\[21:12\\]
: prediction filter with set 3,tap1\n\n\\[11:2\\]
: prediction filter with set 3,tap2"]
    #[inline(always)]
    pub fn mfr_reg36(&self) -> MfrReg36R {
        MfrReg36R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register36 except h264\n\nVP6/VP7:\n\n\\[31:22\\]
: prediction filter with set 3,tap0\n\n\\[21:12\\]
: prediction filter with set 3,tap1\n\n\\[11:2\\]
: prediction filter with set 3,tap2"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg36(&mut self) -> MfrReg36W<Swreg156Spec> {
        MfrReg36W::new(self, 0)
    }
}
#[doc = "multi format reuse register36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg156::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg156::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg156Spec;
impl crate::RegisterSpec for Swreg156Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg156::R`](R) reader structure"]
impl crate::Readable for Swreg156Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg156::W`](W) writer structure"]
impl crate::Writable for Swreg156Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG156 to value 0"]
impl crate::Resettable for Swreg156Spec {
    const RESET_VALUE: u32 = 0;
}
