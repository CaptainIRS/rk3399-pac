#[doc = "Register `IH_MUTE_CEC_STAT0` reader"]
pub type R = crate::R<IhMuteCecStat0Spec>;
#[doc = "Register `IH_MUTE_CEC_STAT0` writer"]
pub type W = crate::W<IhMuteCecStat0Spec>;
#[doc = "Field `DONE` reader - When set to 1, mutes ih_cec_stat0\\[0\\]"]
pub type DoneR = crate::BitReader;
#[doc = "Field `DONE` writer - When set to 1, mutes ih_cec_stat0\\[0\\]"]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOM` reader - When set to 1, mutes ih_cec_stat0\\[1\\]"]
pub type EomR = crate::BitReader;
#[doc = "Field `EOM` writer - When set to 1, mutes ih_cec_stat0\\[1\\]"]
pub type EomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` reader - When set to 1, mutes ih_cec_stat0\\[2\\]"]
pub type NackR = crate::BitReader;
#[doc = "Field `NACK` writer - When set to 1, mutes ih_cec_stat0\\[2\\]"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARB_LOST` reader - When set to 1, mutes ih_cec_stat0\\[3\\]"]
pub type ArbLostR = crate::BitReader;
#[doc = "Field `ARB_LOST` writer - When set to 1, mutes ih_cec_stat0\\[3\\]"]
pub type ArbLostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR_INITIATOR` reader - When set to 1, mutes ih_cec_stat0\\[4\\]"]
pub type ErrorInitiatorR = crate::BitReader;
#[doc = "Field `ERROR_INITIATOR` writer - When set to 1, mutes ih_cec_stat0\\[4\\]"]
pub type ErrorInitiatorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR_FOLLOW` reader - When set to 1, mutes ih_cec_stat0\\[5\\]"]
pub type ErrorFollowR = crate::BitReader;
#[doc = "Field `ERROR_FOLLOW` writer - When set to 1, mutes ih_cec_stat0\\[5\\]"]
pub type ErrorFollowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP` reader - When set to 1, mutes ih_cec_stat0\\[6\\]"]
pub type WakeupR = crate::BitReader;
#[doc = "Field `WAKEUP` writer - When set to 1, mutes ih_cec_stat0\\[6\\]"]
pub type WakeupW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When set to 1, mutes ih_cec_stat0\\[0\\]"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set to 1, mutes ih_cec_stat0\\[1\\]"]
    #[inline(always)]
    pub fn eom(&self) -> EomR {
        EomR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When set to 1, mutes ih_cec_stat0\\[2\\]"]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When set to 1, mutes ih_cec_stat0\\[3\\]"]
    #[inline(always)]
    pub fn arb_lost(&self) -> ArbLostR {
        ArbLostR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When set to 1, mutes ih_cec_stat0\\[4\\]"]
    #[inline(always)]
    pub fn error_initiator(&self) -> ErrorInitiatorR {
        ErrorInitiatorR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When set to 1, mutes ih_cec_stat0\\[5\\]"]
    #[inline(always)]
    pub fn error_follow(&self) -> ErrorFollowR {
        ErrorFollowR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When set to 1, mutes ih_cec_stat0\\[6\\]"]
    #[inline(always)]
    pub fn wakeup(&self) -> WakeupR {
        WakeupR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set to 1, mutes ih_cec_stat0\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DoneW<IhMuteCecStat0Spec> {
        DoneW::new(self, 0)
    }
    #[doc = "Bit 1 - When set to 1, mutes ih_cec_stat0\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn eom(&mut self) -> EomW<IhMuteCecStat0Spec> {
        EomW::new(self, 1)
    }
    #[doc = "Bit 2 - When set to 1, mutes ih_cec_stat0\\[2\\]"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NackW<IhMuteCecStat0Spec> {
        NackW::new(self, 2)
    }
    #[doc = "Bit 3 - When set to 1, mutes ih_cec_stat0\\[3\\]"]
    #[inline(always)]
    #[must_use]
    pub fn arb_lost(&mut self) -> ArbLostW<IhMuteCecStat0Spec> {
        ArbLostW::new(self, 3)
    }
    #[doc = "Bit 4 - When set to 1, mutes ih_cec_stat0\\[4\\]"]
    #[inline(always)]
    #[must_use]
    pub fn error_initiator(&mut self) -> ErrorInitiatorW<IhMuteCecStat0Spec> {
        ErrorInitiatorW::new(self, 4)
    }
    #[doc = "Bit 5 - When set to 1, mutes ih_cec_stat0\\[5\\]"]
    #[inline(always)]
    #[must_use]
    pub fn error_follow(&mut self) -> ErrorFollowW<IhMuteCecStat0Spec> {
        ErrorFollowW::new(self, 5)
    }
    #[doc = "Bit 6 - When set to 1, mutes ih_cec_stat0\\[6\\]"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup(&mut self) -> WakeupW<IhMuteCecStat0Spec> {
        WakeupW::new(self, 6)
    }
}
#[doc = "When set to 1, mutes ih_cec_stat0\\[0\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_mute_cec_stat0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_mute_cec_stat0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IhMuteCecStat0Spec;
impl crate::RegisterSpec for IhMuteCecStat0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ih_mute_cec_stat0::R`](R) reader structure"]
impl crate::Readable for IhMuteCecStat0Spec {}
#[doc = "`write(|w| ..)` method takes [`ih_mute_cec_stat0::W`](W) writer structure"]
impl crate::Writable for IhMuteCecStat0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IH_MUTE_CEC_STAT0 to value 0"]
impl crate::Resettable for IhMuteCecStat0Spec {
    const RESET_VALUE: u8 = 0;
}
