#[doc = "Register `A_HDCPOBS3` reader"]
pub type R = crate::R<AHdcpobs3Spec>;
#[doc = "Field `FAST_REAUTHENTICATION` reader - Register read from attached sink device: Bcap(0x40) bit 0."]
pub type FastReauthenticationR = crate::BitReader;
#[doc = "Field `FEATURES_1_1` reader - Register read from attached sink device: Bcap(0x40) bit 1."]
pub type Features1_1R = crate::BitReader;
#[doc = "Field `HDMI_MODE` reader - Register read from attached sink device: Bstatus(0x41) bit 12."]
pub type HdmiModeR = crate::BitReader;
#[doc = "Field `FAST_I2C` reader - Register read from attached sink device: Bcap(0x40) bit 4."]
pub type FastI2cR = crate::BitReader;
#[doc = "Field `KSV_FIFO_READY` reader - Register read from attached sink device: Bcap(0x40) bit 5."]
pub type KsvFifoReadyR = crate::BitReader;
#[doc = "Field `REPEATER` reader - Register read from attached sink device: Bcap(0x40) bit 6."]
pub type RepeaterR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Register read from attached sink device: Bcap(0x40) bit 0."]
    #[inline(always)]
    pub fn fast_reauthentication(&self) -> FastReauthenticationR {
        FastReauthenticationR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Register read from attached sink device: Bcap(0x40) bit 1."]
    #[inline(always)]
    pub fn features_1_1(&self) -> Features1_1R {
        Features1_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Register read from attached sink device: Bstatus(0x41) bit 12."]
    #[inline(always)]
    pub fn hdmi_mode(&self) -> HdmiModeR {
        HdmiModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Register read from attached sink device: Bcap(0x40) bit 4."]
    #[inline(always)]
    pub fn fast_i2c(&self) -> FastI2cR {
        FastI2cR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Register read from attached sink device: Bcap(0x40) bit 5."]
    #[inline(always)]
    pub fn ksv_fifo_ready(&self) -> KsvFifoReadyR {
        KsvFifoReadyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Register read from attached sink device: Bcap(0x40) bit 6."]
    #[inline(always)]
    pub fn repeater(&self) -> RepeaterR {
        RepeaterR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Register read from attached sink device: Bcap(0x40) bit 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_hdcpobs3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHdcpobs3Spec;
impl crate::RegisterSpec for AHdcpobs3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`a_hdcpobs3::R`](R) reader structure"]
impl crate::Readable for AHdcpobs3Spec {}
#[doc = "`reset()` method sets A_HDCPOBS3 to value 0"]
impl crate::Resettable for AHdcpobs3Spec {
    const RESET_VALUE: u8 = 0;
}
