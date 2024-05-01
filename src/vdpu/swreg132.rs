#[doc = "Register `SWREG132` reader"]
pub type R = crate::R<Swreg132Spec>;
#[doc = "Register `SWREG132` writer"]
pub type W = crate::W<Swreg132Spec>;
#[doc = "Field `MFR_REG12` reader - multi format reuse register12 except h264\n\nVP7:\n\n\\[31\\]
: type of loop filter\n\n\\[30:28\\]
: sharpness of loop filter\n\n\\[27:21\\]
: MB type0 adjustment of filter level\n\n\\[20:14\\]
: MB type1 adjustment of filter level\n\n\\[13:7\\]
: MB type2 adjustment of filter level\n\n\\[6:0\\]
: MB type3 adjustment of filter level"]
pub type MfrReg12R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG12` writer - multi format reuse register12 except h264\n\nVP7:\n\n\\[31\\]
: type of loop filter\n\n\\[30:28\\]
: sharpness of loop filter\n\n\\[27:21\\]
: MB type0 adjustment of filter level\n\n\\[20:14\\]
: MB type1 adjustment of filter level\n\n\\[13:7\\]
: MB type2 adjustment of filter level\n\n\\[6:0\\]
: MB type3 adjustment of filter level"]
pub type MfrReg12W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register12 except h264\n\nVP7:\n\n\\[31\\]
: type of loop filter\n\n\\[30:28\\]
: sharpness of loop filter\n\n\\[27:21\\]
: MB type0 adjustment of filter level\n\n\\[20:14\\]
: MB type1 adjustment of filter level\n\n\\[13:7\\]
: MB type2 adjustment of filter level\n\n\\[6:0\\]
: MB type3 adjustment of filter level"]
    #[inline(always)]
    pub fn mfr_reg12(&self) -> MfrReg12R {
        MfrReg12R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register12 except h264\n\nVP7:\n\n\\[31\\]
: type of loop filter\n\n\\[30:28\\]
: sharpness of loop filter\n\n\\[27:21\\]
: MB type0 adjustment of filter level\n\n\\[20:14\\]
: MB type1 adjustment of filter level\n\n\\[13:7\\]
: MB type2 adjustment of filter level\n\n\\[6:0\\]
: MB type3 adjustment of filter level"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg12(&mut self) -> MfrReg12W<Swreg132Spec> {
        MfrReg12W::new(self, 0)
    }
}
#[doc = "multi format reuse register12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg132::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg132::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg132Spec;
impl crate::RegisterSpec for Swreg132Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg132::R`](R) reader structure"]
impl crate::Readable for Swreg132Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg132::W`](W) writer structure"]
impl crate::Writable for Swreg132Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG132 to value 0"]
impl crate::Resettable for Swreg132Spec {
    const RESET_VALUE: u32 = 0;
}
