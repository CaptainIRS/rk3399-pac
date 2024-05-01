#[doc = "Register `SWREG135` reader"]
pub type R = crate::R<Swreg135Spec>;
#[doc = "Register `SWREG135` writer"]
pub type W = crate::W<Swreg135Spec>;
#[doc = "Field `MFR_REG15` reader - multi format reuse register15 except h264\n\nMPEG4/MEPG2:\n\n\\[31:2\\]
: reference pic3 start address\n\nJPEG:\n\n\\[30:24\\]
: code words of length 10\n\n\\[23:16\\]
: code words of length 9\n\n\\[15:8\\]
: code words of length 8\n\n\\[7:0\\]
: code words of length 7"]
pub type MfrReg15R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG15` writer - multi format reuse register15 except h264\n\nMPEG4/MEPG2:\n\n\\[31:2\\]
: reference pic3 start address\n\nJPEG:\n\n\\[30:24\\]
: code words of length 10\n\n\\[23:16\\]
: code words of length 9\n\n\\[15:8\\]
: code words of length 8\n\n\\[7:0\\]
: code words of length 7"]
pub type MfrReg15W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register15 except h264\n\nMPEG4/MEPG2:\n\n\\[31:2\\]
: reference pic3 start address\n\nJPEG:\n\n\\[30:24\\]
: code words of length 10\n\n\\[23:16\\]
: code words of length 9\n\n\\[15:8\\]
: code words of length 8\n\n\\[7:0\\]
: code words of length 7"]
    #[inline(always)]
    pub fn mfr_reg15(&self) -> MfrReg15R {
        MfrReg15R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register15 except h264\n\nMPEG4/MEPG2:\n\n\\[31:2\\]
: reference pic3 start address\n\nJPEG:\n\n\\[30:24\\]
: code words of length 10\n\n\\[23:16\\]
: code words of length 9\n\n\\[15:8\\]
: code words of length 8\n\n\\[7:0\\]
: code words of length 7"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg15(&mut self) -> MfrReg15W<Swreg135Spec> {
        MfrReg15W::new(self, 0)
    }
}
#[doc = "multi format reuse register15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg135::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg135::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg135Spec;
impl crate::RegisterSpec for Swreg135Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg135::R`](R) reader structure"]
impl crate::Readable for Swreg135Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg135::W`](W) writer structure"]
impl crate::Writable for Swreg135Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG135 to value 0"]
impl crate::Resettable for Swreg135Spec {
    const RESET_VALUE: u32 = 0;
}
