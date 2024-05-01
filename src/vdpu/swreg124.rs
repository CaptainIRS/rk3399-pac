#[doc = "Register `SWREG124` reader"]
pub type R = crate::R<Swreg124Spec>;
#[doc = "Register `SWREG124` writer"]
pub type W = crate::W<Swreg124Spec>;
#[doc = "Field `MFR_REG4` reader - multi format reuse register4 except h264\n\nMPEG4:\n\n\\[31:2\\]
: MB ctrl start address\n\nVP6:\n\n\\[23:0\\]
: total of CTRL stream data\n\nVP7/VP :\n\n\\[27:24\\]
: coeffient partitions number\n\n\\[23:0\\]
: total of CTRL stream data"]
pub type MfrReg4R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG4` writer - multi format reuse register4 except h264\n\nMPEG4:\n\n\\[31:2\\]
: MB ctrl start address\n\nVP6:\n\n\\[23:0\\]
: total of CTRL stream data\n\nVP7/VP :\n\n\\[27:24\\]
: coeffient partitions number\n\n\\[23:0\\]
: total of CTRL stream data"]
pub type MfrReg4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register4 except h264\n\nMPEG4:\n\n\\[31:2\\]
: MB ctrl start address\n\nVP6:\n\n\\[23:0\\]
: total of CTRL stream data\n\nVP7/VP :\n\n\\[27:24\\]
: coeffient partitions number\n\n\\[23:0\\]
: total of CTRL stream data"]
    #[inline(always)]
    pub fn mfr_reg4(&self) -> MfrReg4R {
        MfrReg4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register4 except h264\n\nMPEG4:\n\n\\[31:2\\]
: MB ctrl start address\n\nVP6:\n\n\\[23:0\\]
: total of CTRL stream data\n\nVP7/VP :\n\n\\[27:24\\]
: coeffient partitions number\n\n\\[23:0\\]
: total of CTRL stream data"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg4(&mut self) -> MfrReg4W<Swreg124Spec> {
        MfrReg4W::new(self, 0)
    }
}
#[doc = "multi format reuse register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg124::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg124::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg124Spec;
impl crate::RegisterSpec for Swreg124Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg124::R`](R) reader structure"]
impl crate::Readable for Swreg124Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg124::W`](W) writer structure"]
impl crate::Writable for Swreg124Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG124 to value 0"]
impl crate::Resettable for Swreg124Spec {
    const RESET_VALUE: u32 = 0;
}
