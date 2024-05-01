#[doc = "Register `SWREG157` reader"]
pub type R = crate::R<Swreg157Spec>;
#[doc = "Register `SWREG157` writer"]
pub type W = crate::W<Swreg157Spec>;
#[doc = "Field `MFR_REG37` reader - multi format reuse register37 except h264\n\nVP6/VP7:\n\n\\[31:22\\]
: prediction filter with set 3,tap3\n\n\\[21:12\\]
: prediction filter with set 4,tap0\n\n\\[11:2\\]
: prediction filter with set 4,tap1"]
pub type MfrReg37R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG37` writer - multi format reuse register37 except h264\n\nVP6/VP7:\n\n\\[31:22\\]
: prediction filter with set 3,tap3\n\n\\[21:12\\]
: prediction filter with set 4,tap0\n\n\\[11:2\\]
: prediction filter with set 4,tap1"]
pub type MfrReg37W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register37 except h264\n\nVP6/VP7:\n\n\\[31:22\\]
: prediction filter with set 3,tap3\n\n\\[21:12\\]
: prediction filter with set 4,tap0\n\n\\[11:2\\]
: prediction filter with set 4,tap1"]
    #[inline(always)]
    pub fn mfr_reg37(&self) -> MfrReg37R {
        MfrReg37R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register37 except h264\n\nVP6/VP7:\n\n\\[31:22\\]
: prediction filter with set 3,tap3\n\n\\[21:12\\]
: prediction filter with set 4,tap0\n\n\\[11:2\\]
: prediction filter with set 4,tap1"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg37(&mut self) -> MfrReg37W<Swreg157Spec> {
        MfrReg37W::new(self, 0)
    }
}
#[doc = "multi format reuse register37\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg157::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg157::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg157Spec;
impl crate::RegisterSpec for Swreg157Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg157::R`](R) reader structure"]
impl crate::Readable for Swreg157Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg157::W`](W) writer structure"]
impl crate::Writable for Swreg157Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG157 to value 0"]
impl crate::Resettable for Swreg157Spec {
    const RESET_VALUE: u32 = 0;
}
