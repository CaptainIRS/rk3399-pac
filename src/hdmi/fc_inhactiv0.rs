#[doc = "Register `FC_INHACTIV0` reader"]
pub type R = crate::R<FcInhactiv0Spec>;
#[doc = "Register `FC_INHACTIV0` writer"]
pub type W = crate::W<FcInhactiv0Spec>;
#[doc = "Field `H_IN_ACTIV` reader - Input video Horizontal active pixel region width. Number of Horizontal active pixels \\[0...8191\\]."]
pub type HInActivR = crate::FieldReader;
#[doc = "Field `H_IN_ACTIV` writer - Input video Horizontal active pixel region width. Number of Horizontal active pixels \\[0...8191\\]."]
pub type HInActivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Input video Horizontal active pixel region width. Number of Horizontal active pixels \\[0...8191\\]."]
    #[inline(always)]
    pub fn h_in_activ(&self) -> HInActivR {
        HInActivR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Input video Horizontal active pixel region width. Number of Horizontal active pixels \\[0...8191\\]."]
    #[inline(always)]
    #[must_use]
    pub fn h_in_activ(&mut self) -> HInActivW<FcInhactiv0Spec> {
        HInActivW::new(self, 0)
    }
}
#[doc = "Input video Horizontal active pixel region width. Number of Horizontal active pixels \\[0...8191\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_inhactiv0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_inhactiv0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcInhactiv0Spec;
impl crate::RegisterSpec for FcInhactiv0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_inhactiv0::R`](R) reader structure"]
impl crate::Readable for FcInhactiv0Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_inhactiv0::W`](W) writer structure"]
impl crate::Writable for FcInhactiv0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_INHACTIV0 to value 0"]
impl crate::Resettable for FcInhactiv0Spec {
    const RESET_VALUE: u8 = 0;
}
