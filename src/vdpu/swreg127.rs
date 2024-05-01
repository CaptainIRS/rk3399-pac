#[doc = "Register `SWREG127` reader"]
pub type R = crate::R<Swreg127Spec>;
#[doc = "Register `SWREG127` writer"]
pub type W = crate::W<Swreg127Spec>;
#[doc = "Field `MFR_REG7` reader - multi format reuse register7 except h264\n\nVP6/VP7/vp:\n\n\\[31:22\\]
: prediction filter with set 6 and tap 3\n\n\\[21:12\\]
: prediction filter with set 7 and tap 0\n\n\\[11:2\\]
: prediction filter with set 7 and tap 1"]
pub type MfrReg7R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG7` writer - multi format reuse register7 except h264\n\nVP6/VP7/vp:\n\n\\[31:22\\]
: prediction filter with set 6 and tap 3\n\n\\[21:12\\]
: prediction filter with set 7 and tap 0\n\n\\[11:2\\]
: prediction filter with set 7 and tap 1"]
pub type MfrReg7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register7 except h264\n\nVP6/VP7/vp:\n\n\\[31:22\\]
: prediction filter with set 6 and tap 3\n\n\\[21:12\\]
: prediction filter with set 7 and tap 0\n\n\\[11:2\\]
: prediction filter with set 7 and tap 1"]
    #[inline(always)]
    pub fn mfr_reg7(&self) -> MfrReg7R {
        MfrReg7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register7 except h264\n\nVP6/VP7/vp:\n\n\\[31:22\\]
: prediction filter with set 6 and tap 3\n\n\\[21:12\\]
: prediction filter with set 7 and tap 0\n\n\\[11:2\\]
: prediction filter with set 7 and tap 1"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg7(&mut self) -> MfrReg7W<Swreg127Spec> {
        MfrReg7W::new(self, 0)
    }
}
#[doc = "multi format reuse register7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg127::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg127::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg127Spec;
impl crate::RegisterSpec for Swreg127Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg127::R`](R) reader structure"]
impl crate::Readable for Swreg127Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg127::W`](W) writer structure"]
impl crate::Writable for Swreg127Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG127 to value 0"]
impl crate::Resettable for Swreg127Spec {
    const RESET_VALUE: u32 = 0;
}
