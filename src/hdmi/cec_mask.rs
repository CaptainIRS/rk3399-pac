#[doc = "Register `CEC_MASK` reader"]
pub type R = crate::R<CecMaskSpec>;
#[doc = "Register `CEC_MASK` writer"]
pub type W = crate::W<CecMaskSpec>;
#[doc = "Field `DONE` reader - The current transmission is successful (for initiator\n\nonly)"]
pub type DoneR = crate::BitReader;
#[doc = "Field `DONE` writer - The current transmission is successful (for initiator\n\nonly)"]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOM` reader - EOM is detected so that the received data is ready in\n\nthe receiver data buffer (for follower only)"]
pub type EomR = crate::BitReader;
#[doc = "Field `EOM` writer - EOM is detected so that the received data is ready in\n\nthe receiver data buffer (for follower only)"]
pub type EomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` reader - A frame is not acknowledged in a directly addressed\n\nmessage. Or a frame is negatively acknowledged in a\n\nbroadcast message (for initiator only)"]
pub type NackR = crate::BitReader;
#[doc = "Field `NACK` writer - A frame is not acknowledged in a directly addressed\n\nmessage. Or a frame is negatively acknowledged in a\n\nbroadcast message (for initiator only)"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARB_LOST` reader - The initiator losses the CEC line arbitration to a second\n\ninitiator. (specification CEC 9)"]
pub type ArbLostR = crate::BitReader;
#[doc = "Field `ARB_LOST` writer - The initiator losses the CEC line arbitration to a second\n\ninitiator. (specification CEC 9)"]
pub type ArbLostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR_INITIATOR` reader - An error is detected on a CEC line (for initiator only)."]
pub type ErrorInitiatorR = crate::BitReader;
#[doc = "Field `ERROR_INITIATOR` writer - An error is detected on a CEC line (for initiator only)."]
pub type ErrorInitiatorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR_FLOW` reader - An error is notified by a follower. Abnormal logic data\n\nbit error (for follower)"]
pub type ErrorFlowR = crate::BitReader;
#[doc = "Field `ERROR_FLOW` writer - An error is notified by a follower. Abnormal logic data\n\nbit error (for follower)"]
pub type ErrorFlowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP` reader - Follower wake-up signal mask"]
pub type WakeupR = crate::BitReader;
#[doc = "Field `WAKEUP` writer - Follower wake-up signal mask"]
pub type WakeupW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The current transmission is successful (for initiator\n\nonly)"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EOM is detected so that the received data is ready in\n\nthe receiver data buffer (for follower only)"]
    #[inline(always)]
    pub fn eom(&self) -> EomR {
        EomR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A frame is not acknowledged in a directly addressed\n\nmessage. Or a frame is negatively acknowledged in a\n\nbroadcast message (for initiator only)"]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The initiator losses the CEC line arbitration to a second\n\ninitiator. (specification CEC 9)"]
    #[inline(always)]
    pub fn arb_lost(&self) -> ArbLostR {
        ArbLostR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - An error is detected on a CEC line (for initiator only)."]
    #[inline(always)]
    pub fn error_initiator(&self) -> ErrorInitiatorR {
        ErrorInitiatorR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - An error is notified by a follower. Abnormal logic data\n\nbit error (for follower)"]
    #[inline(always)]
    pub fn error_flow(&self) -> ErrorFlowR {
        ErrorFlowR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Follower wake-up signal mask"]
    #[inline(always)]
    pub fn wakeup(&self) -> WakeupR {
        WakeupR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The current transmission is successful (for initiator\n\nonly)"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DoneW<CecMaskSpec> {
        DoneW::new(self, 0)
    }
    #[doc = "Bit 1 - EOM is detected so that the received data is ready in\n\nthe receiver data buffer (for follower only)"]
    #[inline(always)]
    #[must_use]
    pub fn eom(&mut self) -> EomW<CecMaskSpec> {
        EomW::new(self, 1)
    }
    #[doc = "Bit 2 - A frame is not acknowledged in a directly addressed\n\nmessage. Or a frame is negatively acknowledged in a\n\nbroadcast message (for initiator only)"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NackW<CecMaskSpec> {
        NackW::new(self, 2)
    }
    #[doc = "Bit 3 - The initiator losses the CEC line arbitration to a second\n\ninitiator. (specification CEC 9)"]
    #[inline(always)]
    #[must_use]
    pub fn arb_lost(&mut self) -> ArbLostW<CecMaskSpec> {
        ArbLostW::new(self, 3)
    }
    #[doc = "Bit 4 - An error is detected on a CEC line (for initiator only)."]
    #[inline(always)]
    #[must_use]
    pub fn error_initiator(&mut self) -> ErrorInitiatorW<CecMaskSpec> {
        ErrorInitiatorW::new(self, 4)
    }
    #[doc = "Bit 5 - An error is notified by a follower. Abnormal logic data\n\nbit error (for follower)"]
    #[inline(always)]
    #[must_use]
    pub fn error_flow(&mut self) -> ErrorFlowW<CecMaskSpec> {
        ErrorFlowW::new(self, 5)
    }
    #[doc = "Bit 6 - Follower wake-up signal mask"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup(&mut self) -> WakeupW<CecMaskSpec> {
        WakeupW::new(self, 6)
    }
}
#[doc = "CEC Interrupt Mask Register\n\nThis read/write register masks/unmasks the interrupt events. When the bit is set to 1\n\n(masked), the corresponding event does not trigger an interrupt signal at the system\n\ninterface. When the bit is reset to 0, the interrupt event is unmasked.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cec_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cec_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CecMaskSpec;
impl crate::RegisterSpec for CecMaskSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cec_mask::R`](R) reader structure"]
impl crate::Readable for CecMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`cec_mask::W`](W) writer structure"]
impl crate::Writable for CecMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CEC_MASK to value 0"]
impl crate::Resettable for CecMaskSpec {
    const RESET_VALUE: u8 = 0;
}
