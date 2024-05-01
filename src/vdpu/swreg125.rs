#[doc = "Register `SWREG125` reader"]
pub type R = crate::R<Swreg125Spec>;
#[doc = "Register `SWREG125` writer"]
pub type W = crate::W<Swreg125Spec>;
#[doc = "Field `MFR_REG5` reader - multi format reuse register5 except h264\n\nJPEG:\n\n\\[31:2\\]
: Cb ACDC coeff start address\n\nVP6/VP7/vp:\n\n\\[31:22\\]
: prediction filter with set 5 and tap 1\n\n\\[21:12\\]
: prediction filter with set 5 and tap 2\n\n\\[11:2\\]
: prediction filter with set 5 and tap 3"]
pub type MfrReg5R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG5` writer - multi format reuse register5 except h264\n\nJPEG:\n\n\\[31:2\\]
: Cb ACDC coeff start address\n\nVP6/VP7/vp:\n\n\\[31:22\\]
: prediction filter with set 5 and tap 1\n\n\\[21:12\\]
: prediction filter with set 5 and tap 2\n\n\\[11:2\\]
: prediction filter with set 5 and tap 3"]
pub type MfrReg5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register5 except h264\n\nJPEG:\n\n\\[31:2\\]
: Cb ACDC coeff start address\n\nVP6/VP7/vp:\n\n\\[31:22\\]
: prediction filter with set 5 and tap 1\n\n\\[21:12\\]
: prediction filter with set 5 and tap 2\n\n\\[11:2\\]
: prediction filter with set 5 and tap 3"]
    #[inline(always)]
    pub fn mfr_reg5(&self) -> MfrReg5R {
        MfrReg5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register5 except h264\n\nJPEG:\n\n\\[31:2\\]
: Cb ACDC coeff start address\n\nVP6/VP7/vp:\n\n\\[31:22\\]
: prediction filter with set 5 and tap 1\n\n\\[21:12\\]
: prediction filter with set 5 and tap 2\n\n\\[11:2\\]
: prediction filter with set 5 and tap 3"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg5(&mut self) -> MfrReg5W<Swreg125Spec> {
        MfrReg5W::new(self, 0)
    }
}
#[doc = "multi format reuse register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg125::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg125::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg125Spec;
impl crate::RegisterSpec for Swreg125Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg125::R`](R) reader structure"]
impl crate::Readable for Swreg125Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg125::W`](W) writer structure"]
impl crate::Writable for Swreg125Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG125 to value 0"]
impl crate::Resettable for Swreg125Spec {
    const RESET_VALUE: u32 = 0;
}
