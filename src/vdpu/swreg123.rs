#[doc = "Register `SWREG123` reader"]
pub type R = crate::R<Swreg123Spec>;
#[doc = "Register `SWREG123` writer"]
pub type W = crate::W<Swreg123Spec>;
#[doc = "Field `MFR_REG3` reader - multi format reuse register3 except h264\n\nJPEG:\n\n\\[15:0\\]
: start marker frequency.\n\nvp6:\n\n\\[17:14\\]
: loop filter limit value\n\n\\[13\\]
: enable for variance test\n\n\\[12:10\\]
: filter MV size threshold\n\n\\[9:0\\]
: filter variance threshold\n\nVP7/VP :\n\n\\[31:16\\]
: value 0 for inital dc predictor\n\n\\[15:0\\]
: value 1 for inital dc predictor"]
pub type MfrReg3R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG3` writer - multi format reuse register3 except h264\n\nJPEG:\n\n\\[15:0\\]
: start marker frequency.\n\nvp6:\n\n\\[17:14\\]
: loop filter limit value\n\n\\[13\\]
: enable for variance test\n\n\\[12:10\\]
: filter MV size threshold\n\n\\[9:0\\]
: filter variance threshold\n\nVP7/VP :\n\n\\[31:16\\]
: value 0 for inital dc predictor\n\n\\[15:0\\]
: value 1 for inital dc predictor"]
pub type MfrReg3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register3 except h264\n\nJPEG:\n\n\\[15:0\\]
: start marker frequency.\n\nvp6:\n\n\\[17:14\\]
: loop filter limit value\n\n\\[13\\]
: enable for variance test\n\n\\[12:10\\]
: filter MV size threshold\n\n\\[9:0\\]
: filter variance threshold\n\nVP7/VP :\n\n\\[31:16\\]
: value 0 for inital dc predictor\n\n\\[15:0\\]
: value 1 for inital dc predictor"]
    #[inline(always)]
    pub fn mfr_reg3(&self) -> MfrReg3R {
        MfrReg3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register3 except h264\n\nJPEG:\n\n\\[15:0\\]
: start marker frequency.\n\nvp6:\n\n\\[17:14\\]
: loop filter limit value\n\n\\[13\\]
: enable for variance test\n\n\\[12:10\\]
: filter MV size threshold\n\n\\[9:0\\]
: filter variance threshold\n\nVP7/VP :\n\n\\[31:16\\]
: value 0 for inital dc predictor\n\n\\[15:0\\]
: value 1 for inital dc predictor"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg3(&mut self) -> MfrReg3W<Swreg123Spec> {
        MfrReg3W::new(self, 0)
    }
}
#[doc = "multi format reuse register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg123::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg123::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg123Spec;
impl crate::RegisterSpec for Swreg123Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg123::R`](R) reader structure"]
impl crate::Readable for Swreg123Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg123::W`](W) writer structure"]
impl crate::Writable for Swreg123Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG123 to value 0"]
impl crate::Resettable for Swreg123Spec {
    const RESET_VALUE: u32 = 0;
}
