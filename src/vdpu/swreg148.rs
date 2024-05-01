#[doc = "Register `SWREG148` reader"]
pub type R = crate::R<Swreg148Spec>;
#[doc = "Register `SWREG148` writer"]
pub type W = crate::W<Swreg148Spec>;
#[doc = "Field `MFR_REG28` reader - multi format reuse register28 except h264\n\nMPEG4/MPEG2/VP7:\n\n\\[31:2\\]
: ref pic index 1 start address\n\nJPEG:\n\n\\[7:0\\]
: snapshot"]
pub type MfrReg28R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG28` writer - multi format reuse register28 except h264\n\nMPEG4/MPEG2/VP7:\n\n\\[31:2\\]
: ref pic index 1 start address\n\nJPEG:\n\n\\[7:0\\]
: snapshot"]
pub type MfrReg28W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register28 except h264\n\nMPEG4/MPEG2/VP7:\n\n\\[31:2\\]
: ref pic index 1 start address\n\nJPEG:\n\n\\[7:0\\]
: snapshot"]
    #[inline(always)]
    pub fn mfr_reg28(&self) -> MfrReg28R {
        MfrReg28R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register28 except h264\n\nMPEG4/MPEG2/VP7:\n\n\\[31:2\\]
: ref pic index 1 start address\n\nJPEG:\n\n\\[7:0\\]
: snapshot"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg28(&mut self) -> MfrReg28W<Swreg148Spec> {
        MfrReg28W::new(self, 0)
    }
}
#[doc = "multi format reuse register28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg148::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg148::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg148Spec;
impl crate::RegisterSpec for Swreg148Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg148::R`](R) reader structure"]
impl crate::Readable for Swreg148Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg148::W`](W) writer structure"]
impl crate::Writable for Swreg148Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG148 to value 0"]
impl crate::Resettable for Swreg148Spec {
    const RESET_VALUE: u32 = 0;
}
