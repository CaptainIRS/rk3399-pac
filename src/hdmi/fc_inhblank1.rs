#[doc = "Register `FC_INHBLANK1` reader"]
pub type R = crate::R<FcInhblank1Spec>;
#[doc = "Register `FC_INHBLANK1` writer"]
pub type W = crate::W<FcInhblank1Spec>;
#[doc = "Field `H_IN_BLANK` reader - Input video Horizontal blanking pixel region width\n\nthis bit field holds bits 9:8 of number of Horizontal\n\nblanking pixels."]
pub type HInBlankR = crate::FieldReader;
#[doc = "Field `H_IN_BLANK` writer - Input video Horizontal blanking pixel region width\n\nthis bit field holds bits 9:8 of number of Horizontal\n\nblanking pixels."]
pub type HInBlankW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `H_IN_BLANK_12` reader - Input video Horizontal blanking pixel region width\n\nIf configuration parameter HDMI_TX_14 = True\n\n(1), this bit field holds bit 12:10 of number of\n\nhorizontal blanking pixels."]
pub type HInBlank12R = crate::FieldReader;
#[doc = "Field `H_IN_BLANK_12` writer - Input video Horizontal blanking pixel region width\n\nIf configuration parameter HDMI_TX_14 = True\n\n(1), this bit field holds bit 12:10 of number of\n\nhorizontal blanking pixels."]
pub type HInBlank12W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - Input video Horizontal blanking pixel region width\n\nthis bit field holds bits 9:8 of number of Horizontal\n\nblanking pixels."]
    #[inline(always)]
    pub fn h_in_blank(&self) -> HInBlankR {
        HInBlankR::new(self.bits & 3)
    }
    #[doc = "Bits 2:4 - Input video Horizontal blanking pixel region width\n\nIf configuration parameter HDMI_TX_14 = True\n\n(1), this bit field holds bit 12:10 of number of\n\nhorizontal blanking pixels."]
    #[inline(always)]
    pub fn h_in_blank_12(&self) -> HInBlank12R {
        HInBlank12R::new((self.bits >> 2) & 7)
    }
}
impl W {
    #[doc = "Bits 0:1 - Input video Horizontal blanking pixel region width\n\nthis bit field holds bits 9:8 of number of Horizontal\n\nblanking pixels."]
    #[inline(always)]
    #[must_use]
    pub fn h_in_blank(&mut self) -> HInBlankW<FcInhblank1Spec> {
        HInBlankW::new(self, 0)
    }
    #[doc = "Bits 2:4 - Input video Horizontal blanking pixel region width\n\nIf configuration parameter HDMI_TX_14 = True\n\n(1), this bit field holds bit 12:10 of number of\n\nhorizontal blanking pixels."]
    #[inline(always)]
    #[must_use]
    pub fn h_in_blank_12(&mut self) -> HInBlank12W<FcInhblank1Spec> {
        HInBlank12W::new(self, 2)
    }
}
#[doc = "Frame Composer Input Video HBlank Pixels Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_inhblank1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_inhblank1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcInhblank1Spec;
impl crate::RegisterSpec for FcInhblank1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_inhblank1::R`](R) reader structure"]
impl crate::Readable for FcInhblank1Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_inhblank1::W`](W) writer structure"]
impl crate::Writable for FcInhblank1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_INHBLANK1 to value 0"]
impl crate::Resettable for FcInhblank1Spec {
    const RESET_VALUE: u8 = 0;
}
