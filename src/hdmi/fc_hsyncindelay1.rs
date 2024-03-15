#[doc = "Register `FC_HSYNCINDELAY1` reader"]
pub type R = crate::R<FcHsyncindelay1Spec>;
#[doc = "Register `FC_HSYNCINDELAY1` writer"]
pub type W = crate::W<FcHsyncindelay1Spec>;
#[doc = "Field `H_IN_DELAY` reader - Input video Horizontal active edge delay."]
pub type HInDelayR = crate::FieldReader;
#[doc = "Field `H_IN_DELAY` writer - Input video Horizontal active edge delay."]
pub type HInDelayW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `H_IN_DELAY_12` reader - Input video Horizontal active edge delay. If configuration parameter HDMI_TX_14 = True (1), this bit field holds bit 12. Integer number of pixel clock cycles from \"de\" non-active edge of the last \"de\" valid period \\[0...8191\\]."]
pub type HInDelay12R = crate::FieldReader;
#[doc = "Field `H_IN_DELAY_12` writer - Input video Horizontal active edge delay. If configuration parameter HDMI_TX_14 = True (1), this bit field holds bit 12. Integer number of pixel clock cycles from \"de\" non-active edge of the last \"de\" valid period \\[0...8191\\]."]
pub type HInDelay12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - Input video Horizontal active edge delay."]
    #[inline(always)]
    pub fn h_in_delay(&self) -> HInDelayR {
        HInDelayR::new(self.bits & 7)
    }
    #[doc = "Bits 3:4 - Input video Horizontal active edge delay. If configuration parameter HDMI_TX_14 = True (1), this bit field holds bit 12. Integer number of pixel clock cycles from \"de\" non-active edge of the last \"de\" valid period \\[0...8191\\]."]
    #[inline(always)]
    pub fn h_in_delay_12(&self) -> HInDelay12R {
        HInDelay12R::new((self.bits >> 3) & 3)
    }
}
impl W {
    #[doc = "Bits 0:2 - Input video Horizontal active edge delay."]
    #[inline(always)]
    #[must_use]
    pub fn h_in_delay(&mut self) -> HInDelayW<FcHsyncindelay1Spec> {
        HInDelayW::new(self, 0)
    }
    #[doc = "Bits 3:4 - Input video Horizontal active edge delay. If configuration parameter HDMI_TX_14 = True (1), this bit field holds bit 12. Integer number of pixel clock cycles from \"de\" non-active edge of the last \"de\" valid period \\[0...8191\\]."]
    #[inline(always)]
    #[must_use]
    pub fn h_in_delay_12(&mut self) -> HInDelay12W<FcHsyncindelay1Spec> {
        HInDelay12W::new(self, 3)
    }
}
#[doc = "Input video Horizontal active edge delay.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_hsyncindelay1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_hsyncindelay1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcHsyncindelay1Spec;
impl crate::RegisterSpec for FcHsyncindelay1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_hsyncindelay1::R`](R) reader structure"]
impl crate::Readable for FcHsyncindelay1Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_hsyncindelay1::W`](W) writer structure"]
impl crate::Writable for FcHsyncindelay1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_HSYNCINDELAY1 to value 0"]
impl crate::Resettable for FcHsyncindelay1Spec {
    const RESET_VALUE: u8 = 0;
}
