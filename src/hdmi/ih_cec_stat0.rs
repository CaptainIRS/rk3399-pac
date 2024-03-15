#[doc = "Register `IH_CEC_STAT0` reader"]
pub type R = crate::R<IhCecStat0Spec>;
#[doc = "Register `IH_CEC_STAT0` writer"]
pub type W = crate::W<IhCecStat0Spec>;
#[doc = "Field `DONE` reader - CEC Done Indication"]
pub type DoneR = crate::BitReader;
#[doc = "Field `DONE` writer - CEC Done Indication"]
pub type DoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EOM` reader - CEC End of Message Indication"]
pub type EomR = crate::BitReader;
#[doc = "Field `EOM` writer - CEC End of Message Indication"]
pub type EomW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `NACK` reader - CEC Not Acknowledge indication"]
pub type NackR = crate::BitReader;
#[doc = "Field `NACK` writer - CEC Not Acknowledge indication"]
pub type NackW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ARB_LOST` reader - CEC Arbitration Lost indication"]
pub type ArbLostR = crate::BitReader;
#[doc = "Field `ARB_LOST` writer - CEC Arbitration Lost indication"]
pub type ArbLostW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ERROR_INITIATOR` reader - CEC Error Initiator indication"]
pub type ErrorInitiatorR = crate::BitReader;
#[doc = "Field `ERROR_INITIATOR` writer - CEC Error Initiator indication"]
pub type ErrorInitiatorW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ERROR_FOLLOW` reader - CEC Error Follow indication"]
pub type ErrorFollowR = crate::BitReader;
#[doc = "Field `ERROR_FOLLOW` writer - CEC Error Follow indication"]
pub type ErrorFollowW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `WAKEUP` reader - CEC Wake-up indication"]
pub type WakeupR = crate::BitReader;
#[doc = "Field `WAKEUP` writer - CEC Wake-up indication"]
pub type WakeupW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - CEC Done Indication"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CEC End of Message Indication"]
    #[inline(always)]
    pub fn eom(&self) -> EomR {
        EomR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CEC Not Acknowledge indication"]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CEC Arbitration Lost indication"]
    #[inline(always)]
    pub fn arb_lost(&self) -> ArbLostR {
        ArbLostR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CEC Error Initiator indication"]
    #[inline(always)]
    pub fn error_initiator(&self) -> ErrorInitiatorR {
        ErrorInitiatorR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CEC Error Follow indication"]
    #[inline(always)]
    pub fn error_follow(&self) -> ErrorFollowR {
        ErrorFollowR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CEC Wake-up indication"]
    #[inline(always)]
    pub fn wakeup(&self) -> WakeupR {
        WakeupR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CEC Done Indication"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DoneW<IhCecStat0Spec> {
        DoneW::new(self, 0)
    }
    #[doc = "Bit 1 - CEC End of Message Indication"]
    #[inline(always)]
    #[must_use]
    pub fn eom(&mut self) -> EomW<IhCecStat0Spec> {
        EomW::new(self, 1)
    }
    #[doc = "Bit 2 - CEC Not Acknowledge indication"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NackW<IhCecStat0Spec> {
        NackW::new(self, 2)
    }
    #[doc = "Bit 3 - CEC Arbitration Lost indication"]
    #[inline(always)]
    #[must_use]
    pub fn arb_lost(&mut self) -> ArbLostW<IhCecStat0Spec> {
        ArbLostW::new(self, 3)
    }
    #[doc = "Bit 4 - CEC Error Initiator indication"]
    #[inline(always)]
    #[must_use]
    pub fn error_initiator(&mut self) -> ErrorInitiatorW<IhCecStat0Spec> {
        ErrorInitiatorW::new(self, 4)
    }
    #[doc = "Bit 5 - CEC Error Follow indication"]
    #[inline(always)]
    #[must_use]
    pub fn error_follow(&mut self) -> ErrorFollowW<IhCecStat0Spec> {
        ErrorFollowW::new(self, 5)
    }
    #[doc = "Bit 6 - CEC Wake-up indication"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup(&mut self) -> WakeupW<IhCecStat0Spec> {
        WakeupW::new(self, 6)
    }
}
#[doc = "CEC Done Indication\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_cec_stat0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_cec_stat0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IhCecStat0Spec;
impl crate::RegisterSpec for IhCecStat0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ih_cec_stat0::R`](R) reader structure"]
impl crate::Readable for IhCecStat0Spec {}
#[doc = "`write(|w| ..)` method takes [`ih_cec_stat0::W`](W) writer structure"]
impl crate::Writable for IhCecStat0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0x7f;
}
#[doc = "`reset()` method sets IH_CEC_STAT0 to value 0"]
impl crate::Resettable for IhCecStat0Spec {
    const RESET_VALUE: u8 = 0;
}
