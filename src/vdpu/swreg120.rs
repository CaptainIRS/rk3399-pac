#[doc = "Register `SWREG120` reader"]
pub type R = crate::R<Swreg120Spec>;
#[doc = "Register `SWREG120` writer"]
pub type W = crate::W<Swreg120Spec>;
#[doc = "Field `MFR_REG0` reader - multi format reuse register0 except h264\n\nMPEG4/JPEG/MPEG2/VP6/RV/VP7/VP:\n\n\\[31:2\\]\n\nRLC mode: Base address for RLC data\n\nVLC mode: Stream start address\n\nVP7:\n\n\\[31:2\\]\n\nThis base address is used as sw_dct_strm0_base including DCT\n\nstream for MB rows 0,2n"]
pub type MfrReg0R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG0` writer - multi format reuse register0 except h264\n\nMPEG4/JPEG/MPEG2/VP6/RV/VP7/VP:\n\n\\[31:2\\]\n\nRLC mode: Base address for RLC data\n\nVLC mode: Stream start address\n\nVP7:\n\n\\[31:2\\]\n\nThis base address is used as sw_dct_strm0_base including DCT\n\nstream for MB rows 0,2n"]
pub type MfrReg0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register0 except h264\n\nMPEG4/JPEG/MPEG2/VP6/RV/VP7/VP:\n\n\\[31:2\\]\n\nRLC mode: Base address for RLC data\n\nVLC mode: Stream start address\n\nVP7:\n\n\\[31:2\\]\n\nThis base address is used as sw_dct_strm0_base including DCT\n\nstream for MB rows 0,2n"]
    #[inline(always)]
    pub fn mfr_reg0(&self) -> MfrReg0R {
        MfrReg0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register0 except h264\n\nMPEG4/JPEG/MPEG2/VP6/RV/VP7/VP:\n\n\\[31:2\\]\n\nRLC mode: Base address for RLC data\n\nVLC mode: Stream start address\n\nVP7:\n\n\\[31:2\\]\n\nThis base address is used as sw_dct_strm0_base including DCT\n\nstream for MB rows 0,2n"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg0(&mut self) -> MfrReg0W<Swreg120Spec> {
        MfrReg0W::new(self, 0)
    }
}
#[doc = "multi format reuse register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg120::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg120::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg120Spec;
impl crate::RegisterSpec for Swreg120Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg120::R`](R) reader structure"]
impl crate::Readable for Swreg120Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg120::W`](W) writer structure"]
impl crate::Writable for Swreg120Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG120 to value 0"]
impl crate::Resettable for Swreg120Spec {
    const RESET_VALUE: u32 = 0;
}
