#[doc = "Register `SWREG150` reader"]
pub type R = crate::R<Swreg150Spec>;
#[doc = "Register `SWREG150` writer"]
pub type W = crate::W<Swreg150Spec>;
#[doc = "Field `MFR_REG30` reader - multi format reuse register30 except h264\n\nVP7:\n\n\\[29:24\\]
: DCT stream partition index 3 of start bit\n\n\\[23:18\\]
: DCT stream partition index 4 of start bit\n\n\\[17:12\\]
: DCT stream partition index 5 of start bit\n\n\\[11:6\\]
: DCT stream partition index 6 of start bit\n\n\\[5:0\\]
: DCT stream partition index 7 of start bit"]
pub type MfrReg30R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG30` writer - multi format reuse register30 except h264\n\nVP7:\n\n\\[29:24\\]
: DCT stream partition index 3 of start bit\n\n\\[23:18\\]
: DCT stream partition index 4 of start bit\n\n\\[17:12\\]
: DCT stream partition index 5 of start bit\n\n\\[11:6\\]
: DCT stream partition index 6 of start bit\n\n\\[5:0\\]
: DCT stream partition index 7 of start bit"]
pub type MfrReg30W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register30 except h264\n\nVP7:\n\n\\[29:24\\]
: DCT stream partition index 3 of start bit\n\n\\[23:18\\]
: DCT stream partition index 4 of start bit\n\n\\[17:12\\]
: DCT stream partition index 5 of start bit\n\n\\[11:6\\]
: DCT stream partition index 6 of start bit\n\n\\[5:0\\]
: DCT stream partition index 7 of start bit"]
    #[inline(always)]
    pub fn mfr_reg30(&self) -> MfrReg30R {
        MfrReg30R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register30 except h264\n\nVP7:\n\n\\[29:24\\]
: DCT stream partition index 3 of start bit\n\n\\[23:18\\]
: DCT stream partition index 4 of start bit\n\n\\[17:12\\]
: DCT stream partition index 5 of start bit\n\n\\[11:6\\]
: DCT stream partition index 6 of start bit\n\n\\[5:0\\]
: DCT stream partition index 7 of start bit"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg30(&mut self) -> MfrReg30W<Swreg150Spec> {
        MfrReg30W::new(self, 0)
    }
}
#[doc = "multi format reuse register30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg150::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg150::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg150Spec;
impl crate::RegisterSpec for Swreg150Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg150::R`](R) reader structure"]
impl crate::Readable for Swreg150Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg150::W`](W) writer structure"]
impl crate::Writable for Swreg150Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG150 to value 0"]
impl crate::Resettable for Swreg150Spec {
    const RESET_VALUE: u32 = 0;
}
