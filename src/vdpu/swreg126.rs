#[doc = "Register `SWREG126` reader"]
pub type R = crate::R<Swreg126Spec>;
#[doc = "Register `SWREG126` writer"]
pub type W = crate::W<Swreg126Spec>;
#[doc = "Field `MFR_REG6` reader - multi format reuse register6 except h264\n\nJPEG:\n\n\\[31:2\\]
: Cr ACDC coeff start address\n\nVP6/VP7/vp:\n\n\\[31:22\\]
: prediction filter with set 6 and tap 0\n\n\\[21:12\\]
: prediction filter with set 6 and tap 1\n\n\\[11:2\\]
: prediction filter with set 6 and tap 2"]
pub type MfrReg6R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG6` writer - multi format reuse register6 except h264\n\nJPEG:\n\n\\[31:2\\]
: Cr ACDC coeff start address\n\nVP6/VP7/vp:\n\n\\[31:22\\]
: prediction filter with set 6 and tap 0\n\n\\[21:12\\]
: prediction filter with set 6 and tap 1\n\n\\[11:2\\]
: prediction filter with set 6 and tap 2"]
pub type MfrReg6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register6 except h264\n\nJPEG:\n\n\\[31:2\\]
: Cr ACDC coeff start address\n\nVP6/VP7/vp:\n\n\\[31:22\\]
: prediction filter with set 6 and tap 0\n\n\\[21:12\\]
: prediction filter with set 6 and tap 1\n\n\\[11:2\\]
: prediction filter with set 6 and tap 2"]
    #[inline(always)]
    pub fn mfr_reg6(&self) -> MfrReg6R {
        MfrReg6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register6 except h264\n\nJPEG:\n\n\\[31:2\\]
: Cr ACDC coeff start address\n\nVP6/VP7/vp:\n\n\\[31:22\\]
: prediction filter with set 6 and tap 0\n\n\\[21:12\\]
: prediction filter with set 6 and tap 1\n\n\\[11:2\\]
: prediction filter with set 6 and tap 2"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg6(&mut self) -> MfrReg6W<Swreg126Spec> {
        MfrReg6W::new(self, 0)
    }
}
#[doc = "multi format reuse register6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg126::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg126::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg126Spec;
impl crate::RegisterSpec for Swreg126Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg126::R`](R) reader structure"]
impl crate::Readable for Swreg126Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg126::W`](W) writer structure"]
impl crate::Writable for Swreg126Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG126 to value 0"]
impl crate::Resettable for Swreg126Spec {
    const RESET_VALUE: u32 = 0;
}
