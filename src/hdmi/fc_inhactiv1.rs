#[doc = "Register `FC_INHACTIV1` reader"]
pub type R = crate::R<FcInhactiv1Spec>;
#[doc = "Register `FC_INHACTIV1` writer"]
pub type W = crate::W<FcInhactiv1Spec>;
#[doc = "Field `H_IN_ACTIV_13` reader - Input video Horizontal active pixel region width (0 .. 16383) If the configuration parameter HDMI_TX_20 = True (1), this bit field holds bit 13."]
pub type HInActiv13R = crate::BitReader;
#[doc = "Field `H_IN_ACTIV_13` writer - Input video Horizontal active pixel region width (0 .. 16383) If the configuration parameter HDMI_TX_20 = True (1), this bit field holds bit 13."]
pub type HInActiv13W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - Input video Horizontal active pixel region width (0 .. 16383) If the configuration parameter HDMI_TX_20 = True (1), this bit field holds bit 13."]
    #[inline(always)]
    pub fn h_in_activ_13(&self) -> HInActiv13R {
        HInActiv13R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Input video Horizontal active pixel region width (0 .. 16383) If the configuration parameter HDMI_TX_20 = True (1), this bit field holds bit 13."]
    #[inline(always)]
    #[must_use]
    pub fn h_in_activ_13(&mut self) -> HInActiv13W<FcInhactiv1Spec> {
        HInActiv13W::new(self, 5)
    }
}
#[doc = "Input video Horizontal active pixel region width (0 .. 16383) If the configuration parameter HDMI_TX_20 = True (1), this bit field holds bit 13.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_inhactiv1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_inhactiv1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcInhactiv1Spec;
impl crate::RegisterSpec for FcInhactiv1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_inhactiv1::R`](R) reader structure"]
impl crate::Readable for FcInhactiv1Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_inhactiv1::W`](W) writer structure"]
impl crate::Writable for FcInhactiv1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_INHACTIV1 to value 0"]
impl crate::Resettable for FcInhactiv1Spec {
    const RESET_VALUE: u8 = 0;
}
