#[doc = "Register `MC_SWRSTZREQ` reader"]
pub type R = crate::R<McSwrstzreqSpec>;
#[doc = "Register `MC_SWRSTZREQ` writer"]
pub type W = crate::W<McSwrstzreqSpec>;
#[doc = "Field `PIXELSWRST_REQ` reader - Pixel software reset request."]
pub type PixelswrstReqR = crate::BitReader;
#[doc = "Field `PIXELSWRST_REQ` writer - Pixel software reset request."]
pub type PixelswrstReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMDSSWRST_REQ` reader - TMDS software reset request. It is required to perform a write action on one of the following registers: fc_invidconf, fc_inhactiv0, fc_inhactiv1, fc_inhblank0, fc_inhblank1, fc_invactiv0 fc_invactiv1, fc_invblank, fc_hsyncindelay0, fc_hsyncindelay1, fc_hsyncinwidth0 fc_hsyncinwidth1, fc_vsyncindelay, fc_vsyncinwidth, fc_ctrldur, fc_exctrldur, fc_exctrlspac"]
pub type TmdsswrstReqR = crate::BitReader;
#[doc = "Field `TMDSSWRST_REQ` writer - TMDS software reset request. It is required to perform a write action on one of the following registers: fc_invidconf, fc_inhactiv0, fc_inhactiv1, fc_inhblank0, fc_inhblank1, fc_invactiv0 fc_invactiv1, fc_invblank, fc_hsyncindelay0, fc_hsyncindelay1, fc_hsyncinwidth0 fc_hsyncinwidth1, fc_vsyncindelay, fc_vsyncinwidth, fc_ctrldur, fc_exctrldur, fc_exctrlspac"]
pub type TmdsswrstReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREPSWRST_REQ` reader - Pixel Repetition software reset request."]
pub type PrepswrstReqR = crate::BitReader;
#[doc = "Field `PREPSWRST_REQ` writer - Pixel Repetition software reset request."]
pub type PrepswrstReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `II2SSWRST_REQ` reader - I2S audio software reset request."]
pub type Ii2sswrstReqR = crate::BitReader;
#[doc = "Field `II2SSWRST_REQ` writer - I2S audio software reset request."]
pub type Ii2sswrstReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISPDIFSWRST_REQ` reader - SPDIF audio software reset request."]
pub type IspdifswrstReqR = crate::BitReader;
#[doc = "Field `ISPDIFSWRST_REQ` writer - SPDIF audio software reset request."]
pub type IspdifswrstReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CECSWRST_REQ` reader - CEC software reset request. Defaults back to 1b after reset request. Note: After you configure cecswrst_req, set the value of the bit csc_clk_disable of the register mc_clkdis to 1, 0, and then 1 again."]
pub type CecswrstReqR = crate::BitReader;
#[doc = "Field `CECSWRST_REQ` writer - CEC software reset request. Defaults back to 1b after reset request. Note: After you configure cecswrst_req, set the value of the bit csc_clk_disable of the register mc_clkdis to 1, 0, and then 1 again."]
pub type CecswrstReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGPASWRST_REQ` reader - GPAUD interface soft reset request. This bit is enabled when the Generic Parallel Audio (GPAUD) interface is enabled (AUDIO_IF = 6). Otherwise, this bit returns zero."]
pub type IgpaswrstReqR = crate::BitReader;
#[doc = "Field `IGPASWRST_REQ` writer - GPAUD interface soft reset request. This bit is enabled when the Generic Parallel Audio (GPAUD) interface is enabled (AUDIO_IF = 6). Otherwise, this bit returns zero."]
pub type IgpaswrstReqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pixel software reset request."]
    #[inline(always)]
    pub fn pixelswrst_req(&self) -> PixelswrstReqR {
        PixelswrstReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TMDS software reset request. It is required to perform a write action on one of the following registers: fc_invidconf, fc_inhactiv0, fc_inhactiv1, fc_inhblank0, fc_inhblank1, fc_invactiv0 fc_invactiv1, fc_invblank, fc_hsyncindelay0, fc_hsyncindelay1, fc_hsyncinwidth0 fc_hsyncinwidth1, fc_vsyncindelay, fc_vsyncinwidth, fc_ctrldur, fc_exctrldur, fc_exctrlspac"]
    #[inline(always)]
    pub fn tmdsswrst_req(&self) -> TmdsswrstReqR {
        TmdsswrstReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pixel Repetition software reset request."]
    #[inline(always)]
    pub fn prepswrst_req(&self) -> PrepswrstReqR {
        PrepswrstReqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2S audio software reset request."]
    #[inline(always)]
    pub fn ii2sswrst_req(&self) -> Ii2sswrstReqR {
        Ii2sswrstReqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SPDIF audio software reset request."]
    #[inline(always)]
    pub fn ispdifswrst_req(&self) -> IspdifswrstReqR {
        IspdifswrstReqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - CEC software reset request. Defaults back to 1b after reset request. Note: After you configure cecswrst_req, set the value of the bit csc_clk_disable of the register mc_clkdis to 1, 0, and then 1 again."]
    #[inline(always)]
    pub fn cecswrst_req(&self) -> CecswrstReqR {
        CecswrstReqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPAUD interface soft reset request. This bit is enabled when the Generic Parallel Audio (GPAUD) interface is enabled (AUDIO_IF = 6). Otherwise, this bit returns zero."]
    #[inline(always)]
    pub fn igpaswrst_req(&self) -> IgpaswrstReqR {
        IgpaswrstReqR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pixel software reset request."]
    #[inline(always)]
    #[must_use]
    pub fn pixelswrst_req(&mut self) -> PixelswrstReqW<McSwrstzreqSpec> {
        PixelswrstReqW::new(self, 0)
    }
    #[doc = "Bit 1 - TMDS software reset request. It is required to perform a write action on one of the following registers: fc_invidconf, fc_inhactiv0, fc_inhactiv1, fc_inhblank0, fc_inhblank1, fc_invactiv0 fc_invactiv1, fc_invblank, fc_hsyncindelay0, fc_hsyncindelay1, fc_hsyncinwidth0 fc_hsyncinwidth1, fc_vsyncindelay, fc_vsyncinwidth, fc_ctrldur, fc_exctrldur, fc_exctrlspac"]
    #[inline(always)]
    #[must_use]
    pub fn tmdsswrst_req(&mut self) -> TmdsswrstReqW<McSwrstzreqSpec> {
        TmdsswrstReqW::new(self, 1)
    }
    #[doc = "Bit 2 - Pixel Repetition software reset request."]
    #[inline(always)]
    #[must_use]
    pub fn prepswrst_req(&mut self) -> PrepswrstReqW<McSwrstzreqSpec> {
        PrepswrstReqW::new(self, 2)
    }
    #[doc = "Bit 3 - I2S audio software reset request."]
    #[inline(always)]
    #[must_use]
    pub fn ii2sswrst_req(&mut self) -> Ii2sswrstReqW<McSwrstzreqSpec> {
        Ii2sswrstReqW::new(self, 3)
    }
    #[doc = "Bit 4 - SPDIF audio software reset request."]
    #[inline(always)]
    #[must_use]
    pub fn ispdifswrst_req(&mut self) -> IspdifswrstReqW<McSwrstzreqSpec> {
        IspdifswrstReqW::new(self, 4)
    }
    #[doc = "Bit 6 - CEC software reset request. Defaults back to 1b after reset request. Note: After you configure cecswrst_req, set the value of the bit csc_clk_disable of the register mc_clkdis to 1, 0, and then 1 again."]
    #[inline(always)]
    #[must_use]
    pub fn cecswrst_req(&mut self) -> CecswrstReqW<McSwrstzreqSpec> {
        CecswrstReqW::new(self, 6)
    }
    #[doc = "Bit 7 - GPAUD interface soft reset request. This bit is enabled when the Generic Parallel Audio (GPAUD) interface is enabled (AUDIO_IF = 6). Otherwise, this bit returns zero."]
    #[inline(always)]
    #[must_use]
    pub fn igpaswrst_req(&mut self) -> IgpaswrstReqW<McSwrstzreqSpec> {
        IgpaswrstReqW::new(self, 7)
    }
}
#[doc = "Pixel software reset request.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mc_swrstzreq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mc_swrstzreq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McSwrstzreqSpec;
impl crate::RegisterSpec for McSwrstzreqSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mc_swrstzreq::R`](R) reader structure"]
impl crate::Readable for McSwrstzreqSpec {}
#[doc = "`write(|w| ..)` method takes [`mc_swrstzreq::W`](W) writer structure"]
impl crate::Writable for McSwrstzreqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MC_SWRSTZREQ to value 0xdf"]
impl crate::Resettable for McSwrstzreqSpec {
    const RESET_VALUE: u8 = 0xdf;
}
