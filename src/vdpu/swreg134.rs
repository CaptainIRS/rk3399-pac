#[doc = "Register `SWREG134` reader"]
pub type R = crate::R<Swreg134Spec>;
#[doc = "Register `SWREG134` writer"]
pub type W = crate::W<Swreg134Spec>;
#[doc = "Field `MFR_REG14` reader - multi format reuse register14 except h264\n\nMPEG4/MEPG2:\n\n\\[31:2\\]
: reference pic2 start address\n\nJPEG:\n\n\\[30:24\\]
: code words of length 6\n\n\\[21:16\\]
: code words of length 5\n\n\\[15:11\\]
: code words of length 4\n\n\\[10:7\\]
: code words of length 3\n\n\\[5:3\\]
: code words of length 2\n\n\\[1:0\\]
: code words of length 1"]
pub type MfrReg14R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG14` writer - multi format reuse register14 except h264\n\nMPEG4/MEPG2:\n\n\\[31:2\\]
: reference pic2 start address\n\nJPEG:\n\n\\[30:24\\]
: code words of length 6\n\n\\[21:16\\]
: code words of length 5\n\n\\[15:11\\]
: code words of length 4\n\n\\[10:7\\]
: code words of length 3\n\n\\[5:3\\]
: code words of length 2\n\n\\[1:0\\]
: code words of length 1"]
pub type MfrReg14W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register14 except h264\n\nMPEG4/MEPG2:\n\n\\[31:2\\]
: reference pic2 start address\n\nJPEG:\n\n\\[30:24\\]
: code words of length 6\n\n\\[21:16\\]
: code words of length 5\n\n\\[15:11\\]
: code words of length 4\n\n\\[10:7\\]
: code words of length 3\n\n\\[5:3\\]
: code words of length 2\n\n\\[1:0\\]
: code words of length 1"]
    #[inline(always)]
    pub fn mfr_reg14(&self) -> MfrReg14R {
        MfrReg14R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register14 except h264\n\nMPEG4/MEPG2:\n\n\\[31:2\\]
: reference pic2 start address\n\nJPEG:\n\n\\[30:24\\]
: code words of length 6\n\n\\[21:16\\]
: code words of length 5\n\n\\[15:11\\]
: code words of length 4\n\n\\[10:7\\]
: code words of length 3\n\n\\[5:3\\]
: code words of length 2\n\n\\[1:0\\]
: code words of length 1"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg14(&mut self) -> MfrReg14W<Swreg134Spec> {
        MfrReg14W::new(self, 0)
    }
}
#[doc = "multi format reuse register14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg134::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg134::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg134Spec;
impl crate::RegisterSpec for Swreg134Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg134::R`](R) reader structure"]
impl crate::Readable for Swreg134Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg134::W`](W) writer structure"]
impl crate::Writable for Swreg134Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG134 to value 0"]
impl crate::Resettable for Swreg134Spec {
    const RESET_VALUE: u32 = 0;
}
