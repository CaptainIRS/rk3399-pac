#[doc = "Register `WDT_STAT` reader"]
pub type R = crate::R<WdtStatSpec>;
#[doc = "This register shows the interrupt status of the WDT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WdtStatus {
    #[doc = "1: Interrupt is inactive."]
    B1 = 1,
    #[doc = "0: Interrupt is inactive."]
    B0 = 0,
}
impl From<WdtStatus> for bool {
    #[inline(always)]
    fn from(variant: WdtStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDT_STATUS` reader - This register shows the interrupt status of the WDT."]
pub type WdtStatusR = crate::BitReader<WdtStatus>;
impl WdtStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WdtStatus {
        match self.bits {
            true => WdtStatus::B1,
            false => WdtStatus::B0,
        }
    }
    #[doc = "Interrupt is inactive."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WdtStatus::B1
    }
    #[doc = "Interrupt is inactive."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WdtStatus::B0
    }
}
impl R {
    #[doc = "Bit 0 - This register shows the interrupt status of the WDT."]
    #[inline(always)]
    pub fn wdt_status(&self) -> WdtStatusR {
        WdtStatusR::new((self.bits & 1) != 0)
    }
}
#[doc = "Interrupt status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdt_stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtStatSpec;
impl crate::RegisterSpec for WdtStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt_stat::R`](R) reader structure"]
impl crate::Readable for WdtStatSpec {}
#[doc = "`reset()` method sets WDT_STAT to value 0"]
impl crate::Resettable for WdtStatSpec {
    const RESET_VALUE: u32 = 0;
}
