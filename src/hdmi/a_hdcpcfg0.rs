#[doc = "Register `A_HDCPCFG0` reader"]
pub type R = crate::R<AHdcpcfg0Spec>;
#[doc = "Register `A_HDCPCFG0` writer"]
pub type W = crate::W<AHdcpcfg0Spec>;
#[doc = "Field `HDMIDVI` reader - Configures the transmitter to operate with a HDMI\n\ncapable device or with a DVI device."]
pub type HdmidviR = crate::BitReader;
#[doc = "Field `HDMIDVI` writer - Configures the transmitter to operate with a HDMI\n\ncapable device or with a DVI device."]
pub type HdmidviW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN11FEATURE` reader - Enable the use of features 1.1 from the\n\ntransmitter's side"]
pub type En11featureR = crate::BitReader;
#[doc = "Field `EN11FEATURE` writer - Enable the use of features 1.1 from the\n\ntransmitter's side"]
pub type En11featureW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDETECT` reader - Information that a sink device was detected\n\nconnected to the HDMI port"]
pub type RxdetectR = crate::BitReader;
#[doc = "Field `RXDETECT` writer - Information that a sink device was detected\n\nconnected to the HDMI port"]
pub type RxdetectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVMUTE` reader - This register holds the current AVMUTE state of the\n\nHdmi_tx controller, as expected to be perceived by\n\nthe connected HDMI/HDCP sink device."]
pub type AvmuteR = crate::BitReader;
#[doc = "Field `SYNCRICHECK` reader - Configures if the Ri check should be done at every\n\n2s even or synchronously to every 128 encrypted\n\nframe."]
pub type SyncricheckR = crate::BitReader;
#[doc = "Field `SYNCRICHECK` writer - Configures if the Ri check should be done at every\n\n2s even or synchronously to every 128 encrypted\n\nframe."]
pub type SyncricheckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPENCRYPTION` reader - Bypasses all the data encryption stages\n\nReset Value: (HDMI_HDCP_BYPASS== 1) ? 1 : 0"]
pub type BypencryptionR = crate::BitReader;
#[doc = "Field `BYPENCRYPTION` writer - Bypasses all the data encryption stages\n\nReset Value: (HDMI_HDCP_BYPASS== 1) ? 1 : 0"]
pub type BypencryptionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2CFASTMODE` reader - Enable the I2C fast mode option from the\n\ntransmitter's side."]
pub type I2cfastmodeR = crate::BitReader;
#[doc = "Field `I2CFASTMODE` writer - Enable the I2C fast mode option from the\n\ntransmitter's side."]
pub type I2cfastmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELVENA` reader - Enables the Enhanced Link Verification from the\n\ntransmitter's side"]
pub type ElvenaR = crate::BitReader;
#[doc = "Field `ELVENA` writer - Enables the Enhanced Link Verification from the\n\ntransmitter's side"]
pub type ElvenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures the transmitter to operate with a HDMI\n\ncapable device or with a DVI device."]
    #[inline(always)]
    pub fn hdmidvi(&self) -> HdmidviR {
        HdmidviR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable the use of features 1.1 from the\n\ntransmitter's side"]
    #[inline(always)]
    pub fn en11feature(&self) -> En11featureR {
        En11featureR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Information that a sink device was detected\n\nconnected to the HDMI port"]
    #[inline(always)]
    pub fn rxdetect(&self) -> RxdetectR {
        RxdetectR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This register holds the current AVMUTE state of the\n\nHdmi_tx controller, as expected to be perceived by\n\nthe connected HDMI/HDCP sink device."]
    #[inline(always)]
    pub fn avmute(&self) -> AvmuteR {
        AvmuteR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures if the Ri check should be done at every\n\n2s even or synchronously to every 128 encrypted\n\nframe."]
    #[inline(always)]
    pub fn syncricheck(&self) -> SyncricheckR {
        SyncricheckR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bypasses all the data encryption stages\n\nReset Value: (HDMI_HDCP_BYPASS== 1) ? 1 : 0"]
    #[inline(always)]
    pub fn bypencryption(&self) -> BypencryptionR {
        BypencryptionR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable the I2C fast mode option from the\n\ntransmitter's side."]
    #[inline(always)]
    pub fn i2cfastmode(&self) -> I2cfastmodeR {
        I2cfastmodeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables the Enhanced Link Verification from the\n\ntransmitter's side"]
    #[inline(always)]
    pub fn elvena(&self) -> ElvenaR {
        ElvenaR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configures the transmitter to operate with a HDMI\n\ncapable device or with a DVI device."]
    #[inline(always)]
    #[must_use]
    pub fn hdmidvi(&mut self) -> HdmidviW<AHdcpcfg0Spec> {
        HdmidviW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable the use of features 1.1 from the\n\ntransmitter's side"]
    #[inline(always)]
    #[must_use]
    pub fn en11feature(&mut self) -> En11featureW<AHdcpcfg0Spec> {
        En11featureW::new(self, 1)
    }
    #[doc = "Bit 2 - Information that a sink device was detected\n\nconnected to the HDMI port"]
    #[inline(always)]
    #[must_use]
    pub fn rxdetect(&mut self) -> RxdetectW<AHdcpcfg0Spec> {
        RxdetectW::new(self, 2)
    }
    #[doc = "Bit 4 - Configures if the Ri check should be done at every\n\n2s even or synchronously to every 128 encrypted\n\nframe."]
    #[inline(always)]
    #[must_use]
    pub fn syncricheck(&mut self) -> SyncricheckW<AHdcpcfg0Spec> {
        SyncricheckW::new(self, 4)
    }
    #[doc = "Bit 5 - Bypasses all the data encryption stages\n\nReset Value: (HDMI_HDCP_BYPASS== 1) ? 1 : 0"]
    #[inline(always)]
    #[must_use]
    pub fn bypencryption(&mut self) -> BypencryptionW<AHdcpcfg0Spec> {
        BypencryptionW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable the I2C fast mode option from the\n\ntransmitter's side."]
    #[inline(always)]
    #[must_use]
    pub fn i2cfastmode(&mut self) -> I2cfastmodeW<AHdcpcfg0Spec> {
        I2cfastmodeW::new(self, 6)
    }
    #[doc = "Bit 7 - Enables the Enhanced Link Verification from the\n\ntransmitter's side"]
    #[inline(always)]
    #[must_use]
    pub fn elvena(&mut self) -> ElvenaW<AHdcpcfg0Spec> {
        ElvenaW::new(self, 7)
    }
}
#[doc = "HDCP Enable and Functional Control Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_hdcpcfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_hdcpcfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHdcpcfg0Spec;
impl crate::RegisterSpec for AHdcpcfg0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`a_hdcpcfg0::R`](R) reader structure"]
impl crate::Readable for AHdcpcfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`a_hdcpcfg0::W`](W) writer structure"]
impl crate::Writable for AHdcpcfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets A_HDCPCFG0 to value 0"]
impl crate::Resettable for AHdcpcfg0Spec {
    const RESET_VALUE: u8 = 0;
}
