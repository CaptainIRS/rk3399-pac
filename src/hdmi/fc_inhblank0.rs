#[doc = "Register `FC_INHBLANK0` reader"]
pub type R = crate::R<FcInhblank0Spec>;
#[doc = "Register `FC_INHBLANK0` writer"]
pub type W = crate::W<FcInhblank0Spec>;
#[doc = "Field `H_IN_BLANK` reader - Input video Horizontal blanking pixel region width.\n\nNumber of Horizontal blanking pixels \\[0...4095\\]."]
pub type HInBlankR = crate::FieldReader;
#[doc = "Field `H_IN_BLANK` writer - Input video Horizontal blanking pixel region width.\n\nNumber of Horizontal blanking pixels \\[0...4095\\]."]
pub type HInBlankW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Input video Horizontal blanking pixel region width.\n\nNumber of Horizontal blanking pixels \\[0...4095\\]."]
    #[inline(always)]
    pub fn h_in_blank(&self) -> HInBlankR {
        HInBlankR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Input video Horizontal blanking pixel region width.\n\nNumber of Horizontal blanking pixels \\[0...4095\\]."]
    #[inline(always)]
    #[must_use]
    pub fn h_in_blank(&mut self) -> HInBlankW<FcInhblank0Spec> {
        HInBlankW::new(self, 0)
    }
}
#[doc = "Frame Composer Input Video HBlank Pixels Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_inhblank0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_inhblank0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcInhblank0Spec;
impl crate::RegisterSpec for FcInhblank0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_inhblank0::R`](R) reader structure"]
impl crate::Readable for FcInhblank0Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_inhblank0::W`](W) writer structure"]
impl crate::Writable for FcInhblank0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_INHBLANK0 to value 0"]
impl crate::Resettable for FcInhblank0Spec {
    const RESET_VALUE: u8 = 0;
}
