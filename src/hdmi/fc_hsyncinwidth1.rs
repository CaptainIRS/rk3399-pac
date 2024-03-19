#[doc = "Register `FC_HSYNCINWIDTH1` reader"]
pub type R = crate::R<FcHsyncinwidth1Spec>;
#[doc = "Register `FC_HSYNCINWIDTH1` writer"]
pub type W = crate::W<FcHsyncinwidth1Spec>;
#[doc = "Field `H_IN_WIDTH` reader - Input video Hsync active pulse width."]
pub type HInWidthR = crate::BitReader;
#[doc = "Field `H_IN_WIDTH` writer - Input video Hsync active pulse width."]
pub type HInWidthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_IN_WIDTH_9` reader - Input video Hsync active pulse width.\n\nIf configuration parameter HDMI_TX_14 = True\n\n(1), then this bit field holds bit 9. Number of\n\nHorizontal active pixels \\[0...1024\\]."]
pub type HInWidth9R = crate::BitReader;
#[doc = "Field `H_IN_WIDTH_9` writer - Input video Hsync active pulse width.\n\nIf configuration parameter HDMI_TX_14 = True\n\n(1), then this bit field holds bit 9. Number of\n\nHorizontal active pixels \\[0...1024\\]."]
pub type HInWidth9W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Input video Hsync active pulse width."]
    #[inline(always)]
    pub fn h_in_width(&self) -> HInWidthR {
        HInWidthR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input video Hsync active pulse width.\n\nIf configuration parameter HDMI_TX_14 = True\n\n(1), then this bit field holds bit 9. Number of\n\nHorizontal active pixels \\[0...1024\\]."]
    #[inline(always)]
    pub fn h_in_width_9(&self) -> HInWidth9R {
        HInWidth9R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Input video Hsync active pulse width."]
    #[inline(always)]
    #[must_use]
    pub fn h_in_width(&mut self) -> HInWidthW<FcHsyncinwidth1Spec> {
        HInWidthW::new(self, 0)
    }
    #[doc = "Bit 1 - Input video Hsync active pulse width.\n\nIf configuration parameter HDMI_TX_14 = True\n\n(1), then this bit field holds bit 9. Number of\n\nHorizontal active pixels \\[0...1024\\]."]
    #[inline(always)]
    #[must_use]
    pub fn h_in_width_9(&mut self) -> HInWidth9W<FcHsyncinwidth1Spec> {
        HInWidth9W::new(self, 1)
    }
}
#[doc = "Frame Composer Input Video HSync Width Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_hsyncinwidth1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_hsyncinwidth1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcHsyncinwidth1Spec;
impl crate::RegisterSpec for FcHsyncinwidth1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_hsyncinwidth1::R`](R) reader structure"]
impl crate::Readable for FcHsyncinwidth1Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_hsyncinwidth1::W`](W) writer structure"]
impl crate::Writable for FcHsyncinwidth1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_HSYNCINWIDTH1 to value 0"]
impl crate::Resettable for FcHsyncinwidth1Spec {
    const RESET_VALUE: u8 = 0;
}
