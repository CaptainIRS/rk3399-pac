#[doc = "Register `SWREG153` reader"]
pub type R = crate::R<Swreg153Spec>;
#[doc = "Register `SWREG153` writer"]
pub type W = crate::W<Swreg153Spec>;
#[doc = "Field `MFR_REG33` reader - multi format reuse register33 except h264\n\nVP6/VP7:\n\n\\[31:22\\]
: prediction filter with set 0,tap3 (also for mpeg4)\n\n\\[21:12\\]
: prediction filter with set 1,tap0\n\n\\[11:2\\]
: prediction filter with set 1,tap1"]
pub type MfrReg33R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG33` writer - multi format reuse register33 except h264\n\nVP6/VP7:\n\n\\[31:22\\]
: prediction filter with set 0,tap3 (also for mpeg4)\n\n\\[21:12\\]
: prediction filter with set 1,tap0\n\n\\[11:2\\]
: prediction filter with set 1,tap1"]
pub type MfrReg33W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register33 except h264\n\nVP6/VP7:\n\n\\[31:22\\]
: prediction filter with set 0,tap3 (also for mpeg4)\n\n\\[21:12\\]
: prediction filter with set 1,tap0\n\n\\[11:2\\]
: prediction filter with set 1,tap1"]
    #[inline(always)]
    pub fn mfr_reg33(&self) -> MfrReg33R {
        MfrReg33R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register33 except h264\n\nVP6/VP7:\n\n\\[31:22\\]
: prediction filter with set 0,tap3 (also for mpeg4)\n\n\\[21:12\\]
: prediction filter with set 1,tap0\n\n\\[11:2\\]
: prediction filter with set 1,tap1"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg33(&mut self) -> MfrReg33W<Swreg153Spec> {
        MfrReg33W::new(self, 0)
    }
}
#[doc = "multi format reuse register33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg153::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg153::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg153Spec;
impl crate::RegisterSpec for Swreg153Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg153::R`](R) reader structure"]
impl crate::Readable for Swreg153Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg153::W`](W) writer structure"]
impl crate::Writable for Swreg153Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG153 to value 0"]
impl crate::Resettable for Swreg153Spec {
    const RESET_VALUE: u32 = 0;
}
