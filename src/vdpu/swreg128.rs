#[doc = "Register `SWREG128` reader"]
pub type R = crate::R<Swreg128Spec>;
#[doc = "Register `SWREG128` writer"]
pub type W = crate::W<Swreg128Spec>;
#[doc = "Field `MFR_REG8` reader - multi format reuse register8 except h264\n\nVP6:\n\n\\[31:22\\]
: prediction filter with set 7 and tap 2\n\n\\[21:12\\]
: prediction filter with set 7 and tap 3\n\nVP7/VP:\n\n\\[31:22\\]
: prediction filter with set 7 and tap 2\n\n\\[21:12\\]
: prediction filter with set 7 and tap 3\n\n\\[11:10\\]
: extra prediction filter with set 2 and tap -1\n\n\\[9:8\\]
: extra prediction filter with set 2 and tap 4\n\n\\[7:6\\]
: extra prediction filter with set 4 and tap -1\n\n\\[5:4\\]
: extra prediction filter with set 4 and tap 4\n\n\\[3:2\\]
: extra prediction filter with set 6 and tap -1\n\n\\[1:0\\]
: extra prediction filter with set 6 and tap 4"]
pub type MfrReg8R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG8` writer - multi format reuse register8 except h264\n\nVP6:\n\n\\[31:22\\]
: prediction filter with set 7 and tap 2\n\n\\[21:12\\]
: prediction filter with set 7 and tap 3\n\nVP7/VP:\n\n\\[31:22\\]
: prediction filter with set 7 and tap 2\n\n\\[21:12\\]
: prediction filter with set 7 and tap 3\n\n\\[11:10\\]
: extra prediction filter with set 2 and tap -1\n\n\\[9:8\\]
: extra prediction filter with set 2 and tap 4\n\n\\[7:6\\]
: extra prediction filter with set 4 and tap -1\n\n\\[5:4\\]
: extra prediction filter with set 4 and tap 4\n\n\\[3:2\\]
: extra prediction filter with set 6 and tap -1\n\n\\[1:0\\]
: extra prediction filter with set 6 and tap 4"]
pub type MfrReg8W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register8 except h264\n\nVP6:\n\n\\[31:22\\]
: prediction filter with set 7 and tap 2\n\n\\[21:12\\]
: prediction filter with set 7 and tap 3\n\nVP7/VP:\n\n\\[31:22\\]
: prediction filter with set 7 and tap 2\n\n\\[21:12\\]
: prediction filter with set 7 and tap 3\n\n\\[11:10\\]
: extra prediction filter with set 2 and tap -1\n\n\\[9:8\\]
: extra prediction filter with set 2 and tap 4\n\n\\[7:6\\]
: extra prediction filter with set 4 and tap -1\n\n\\[5:4\\]
: extra prediction filter with set 4 and tap 4\n\n\\[3:2\\]
: extra prediction filter with set 6 and tap -1\n\n\\[1:0\\]
: extra prediction filter with set 6 and tap 4"]
    #[inline(always)]
    pub fn mfr_reg8(&self) -> MfrReg8R {
        MfrReg8R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register8 except h264\n\nVP6:\n\n\\[31:22\\]
: prediction filter with set 7 and tap 2\n\n\\[21:12\\]
: prediction filter with set 7 and tap 3\n\nVP7/VP:\n\n\\[31:22\\]
: prediction filter with set 7 and tap 2\n\n\\[21:12\\]
: prediction filter with set 7 and tap 3\n\n\\[11:10\\]
: extra prediction filter with set 2 and tap -1\n\n\\[9:8\\]
: extra prediction filter with set 2 and tap 4\n\n\\[7:6\\]
: extra prediction filter with set 4 and tap -1\n\n\\[5:4\\]
: extra prediction filter with set 4 and tap 4\n\n\\[3:2\\]
: extra prediction filter with set 6 and tap -1\n\n\\[1:0\\]
: extra prediction filter with set 6 and tap 4"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg8(&mut self) -> MfrReg8W<Swreg128Spec> {
        MfrReg8W::new(self, 0)
    }
}
#[doc = "multi format reuse register8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg128::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg128::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg128Spec;
impl crate::RegisterSpec for Swreg128Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg128::R`](R) reader structure"]
impl crate::Readable for Swreg128Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg128::W`](W) writer structure"]
impl crate::Writable for Swreg128Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG128 to value 0"]
impl crate::Resettable for Swreg128Spec {
    const RESET_VALUE: u32 = 0;
}
