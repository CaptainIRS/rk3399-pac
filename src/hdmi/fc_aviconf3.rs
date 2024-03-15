#[doc = "Register `FC_AVICONF3` reader"]
pub type R = crate::R<FcAviconf3Spec>;
#[doc = "Register `FC_AVICONF3` writer"]
pub type W = crate::W<FcAviconf3Spec>;
#[doc = "Field `CN` reader - IT content type according to CEA the specification"]
pub type CnR = crate::FieldReader;
#[doc = "Field `CN` writer - IT content type according to CEA the specification"]
pub type CnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `YQ` reader - YCC Quantization range according to the CEA specification"]
pub type YqR = crate::FieldReader;
#[doc = "Field `YQ` writer - YCC Quantization range according to the CEA specification"]
pub type YqW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - IT content type according to CEA the specification"]
    #[inline(always)]
    pub fn cn(&self) -> CnR {
        CnR::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - YCC Quantization range according to the CEA specification"]
    #[inline(always)]
    pub fn yq(&self) -> YqR {
        YqR::new((self.bits >> 2) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - IT content type according to CEA the specification"]
    #[inline(always)]
    #[must_use]
    pub fn cn(&mut self) -> CnW<FcAviconf3Spec> {
        CnW::new(self, 0)
    }
    #[doc = "Bits 2:3 - YCC Quantization range according to the CEA specification"]
    #[inline(always)]
    #[must_use]
    pub fn yq(&mut self) -> YqW<FcAviconf3Spec> {
        YqW::new(self, 2)
    }
}
#[doc = "IT content type according to CEA the specification\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_aviconf3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_aviconf3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAviconf3Spec;
impl crate::RegisterSpec for FcAviconf3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_aviconf3::R`](R) reader structure"]
impl crate::Readable for FcAviconf3Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_aviconf3::W`](W) writer structure"]
impl crate::Writable for FcAviconf3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_AVICONF3 to value 0"]
impl crate::Resettable for FcAviconf3Spec {
    const RESET_VALUE: u8 = 0;
}
