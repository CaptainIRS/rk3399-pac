#[doc = "Register `FC_INVACTIV0` reader"]
pub type R = crate::R<FcInvactiv0Spec>;
#[doc = "Register `FC_INVACTIV0` writer"]
pub type W = crate::W<FcInvactiv0Spec>;
#[doc = "Field `V_IN_ACTIV` reader - Input video Vertical active pixel region width. This bit field holds bits 7:0 of number of Vertical active pixels."]
pub type VInActivR = crate::FieldReader;
#[doc = "Field `V_IN_ACTIV` writer - Input video Vertical active pixel region width. This bit field holds bits 7:0 of number of Vertical active pixels."]
pub type VInActivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Input video Vertical active pixel region width. This bit field holds bits 7:0 of number of Vertical active pixels."]
    #[inline(always)]
    pub fn v_in_activ(&self) -> VInActivR {
        VInActivR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Input video Vertical active pixel region width. This bit field holds bits 7:0 of number of Vertical active pixels."]
    #[inline(always)]
    #[must_use]
    pub fn v_in_activ(&mut self) -> VInActivW<FcInvactiv0Spec> {
        VInActivW::new(self, 0)
    }
}
#[doc = "Input video Vertical active pixel region width. This bit field holds bits 7:0 of number of Vertical active pixels.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_invactiv0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_invactiv0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcInvactiv0Spec;
impl crate::RegisterSpec for FcInvactiv0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_invactiv0::R`](R) reader structure"]
impl crate::Readable for FcInvactiv0Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_invactiv0::W`](W) writer structure"]
impl crate::Writable for FcInvactiv0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_INVACTIV0 to value 0"]
impl crate::Resettable for FcInvactiv0Spec {
    const RESET_VALUE: u8 = 0;
}
