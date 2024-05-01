#[doc = "Register `SWREG137` reader"]
pub type R = crate::R<Swreg137Spec>;
#[doc = "Register `SWREG137` writer"]
pub type W = crate::W<Swreg137Spec>;
#[doc = "Field `MFR_REG17` reader - multi format reuse register17 except h264\n\nMPEG4:\n\n\\[26:0\\]
: reference distance syntax for delta value0 be used\n\nJPEG:\n\n\\[31:27\\]
: tab2:code words of length 4\n\n\\[26:23\\]
: tab2:code words of length 3\n\n\\[21:19\\]
: tab2:code words of length 2\n\n\\[17:16\\]
: tab2:code words of length 1\n\n\\[15:8\\]
: tab1:code words of length 16\n\n\\[7:0\\]
: tab1:code words of length 15"]
pub type MfrReg17R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG17` writer - multi format reuse register17 except h264\n\nMPEG4:\n\n\\[26:0\\]
: reference distance syntax for delta value0 be used\n\nJPEG:\n\n\\[31:27\\]
: tab2:code words of length 4\n\n\\[26:23\\]
: tab2:code words of length 3\n\n\\[21:19\\]
: tab2:code words of length 2\n\n\\[17:16\\]
: tab2:code words of length 1\n\n\\[15:8\\]
: tab1:code words of length 16\n\n\\[7:0\\]
: tab1:code words of length 15"]
pub type MfrReg17W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register17 except h264\n\nMPEG4:\n\n\\[26:0\\]
: reference distance syntax for delta value0 be used\n\nJPEG:\n\n\\[31:27\\]
: tab2:code words of length 4\n\n\\[26:23\\]
: tab2:code words of length 3\n\n\\[21:19\\]
: tab2:code words of length 2\n\n\\[17:16\\]
: tab2:code words of length 1\n\n\\[15:8\\]
: tab1:code words of length 16\n\n\\[7:0\\]
: tab1:code words of length 15"]
    #[inline(always)]
    pub fn mfr_reg17(&self) -> MfrReg17R {
        MfrReg17R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register17 except h264\n\nMPEG4:\n\n\\[26:0\\]
: reference distance syntax for delta value0 be used\n\nJPEG:\n\n\\[31:27\\]
: tab2:code words of length 4\n\n\\[26:23\\]
: tab2:code words of length 3\n\n\\[21:19\\]
: tab2:code words of length 2\n\n\\[17:16\\]
: tab2:code words of length 1\n\n\\[15:8\\]
: tab1:code words of length 16\n\n\\[7:0\\]
: tab1:code words of length 15"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg17(&mut self) -> MfrReg17W<Swreg137Spec> {
        MfrReg17W::new(self, 0)
    }
}
#[doc = "multi format reuse register17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg137::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg137::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg137Spec;
impl crate::RegisterSpec for Swreg137Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg137::R`](R) reader structure"]
impl crate::Readable for Swreg137Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg137::W`](W) writer structure"]
impl crate::Writable for Swreg137Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG137 to value 0"]
impl crate::Resettable for Swreg137Spec {
    const RESET_VALUE: u32 = 0;
}
