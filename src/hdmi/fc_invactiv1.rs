#[doc = "Register `FC_INVACTIV1` reader"]
pub type R = crate::R<FcInvactiv1Spec>;
#[doc = "Register `FC_INVACTIV1` writer"]
pub type W = crate::W<FcInvactiv1Spec>;
#[doc = "Field `V_IN_ACTIV` reader - Input video Vertical active pixel region width. This\n\nbit field holds bits 9:8 of number of Vertical active\n\npixels."]
pub type VInActivR = crate::FieldReader;
#[doc = "Field `V_IN_ACTIV` writer - Input video Vertical active pixel region width. This\n\nbit field holds bits 9:8 of number of Vertical active\n\npixels."]
pub type VInActivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `V_IN_ACTIV_12_11` reader - Input video Vertical active pixel region width.\n\nIf the configuration parameter HDMI_TX_14 = True\n\n(1), this bit field holds bits 12:10 of number of\n\nVertical active pixels."]
pub type VInActiv12_11R = crate::FieldReader;
#[doc = "Field `V_IN_ACTIV_12_11` writer - Input video Vertical active pixel region width.\n\nIf the configuration parameter HDMI_TX_14 = True\n\n(1), this bit field holds bits 12:10 of number of\n\nVertical active pixels."]
pub type VInActiv12_11W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - Input video Vertical active pixel region width. This\n\nbit field holds bits 9:8 of number of Vertical active\n\npixels."]
    #[inline(always)]
    pub fn v_in_activ(&self) -> VInActivR {
        VInActivR::new(self.bits & 7)
    }
    #[doc = "Bits 3:4 - Input video Vertical active pixel region width.\n\nIf the configuration parameter HDMI_TX_14 = True\n\n(1), this bit field holds bits 12:10 of number of\n\nVertical active pixels."]
    #[inline(always)]
    pub fn v_in_activ_12_11(&self) -> VInActiv12_11R {
        VInActiv12_11R::new((self.bits >> 3) & 3)
    }
}
impl W {
    #[doc = "Bits 0:2 - Input video Vertical active pixel region width. This\n\nbit field holds bits 9:8 of number of Vertical active\n\npixels."]
    #[inline(always)]
    #[must_use]
    pub fn v_in_activ(&mut self) -> VInActivW<FcInvactiv1Spec> {
        VInActivW::new(self, 0)
    }
    #[doc = "Bits 3:4 - Input video Vertical active pixel region width.\n\nIf the configuration parameter HDMI_TX_14 = True\n\n(1), this bit field holds bits 12:10 of number of\n\nVertical active pixels."]
    #[inline(always)]
    #[must_use]
    pub fn v_in_activ_12_11(&mut self) -> VInActiv12_11W<FcInvactiv1Spec> {
        VInActiv12_11W::new(self, 3)
    }
}
#[doc = "Frame Composer Input Video VActive Pixels Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_invactiv1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_invactiv1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcInvactiv1Spec;
impl crate::RegisterSpec for FcInvactiv1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_invactiv1::R`](R) reader structure"]
impl crate::Readable for FcInvactiv1Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_invactiv1::W`](W) writer structure"]
impl crate::Writable for FcInvactiv1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_INVACTIV1 to value 0"]
impl crate::Resettable for FcInvactiv1Spec {
    const RESET_VALUE: u8 = 0;
}
